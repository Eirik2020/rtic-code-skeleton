"""Check completion, hover, navigation and unsaved expansion via real RA LSP.

Run from the repository root: py -X utf8 tools/check-ide.py f401
Requires rust-analyzer on PATH. Never changes the source file on disk.
"""

import json
import pathlib
import queue
import subprocess
import sys
import threading
import time

root = pathlib.Path(__file__).resolve().parent.parent
source = root / 'src/app_body_skeleton.rs'
chip = sys.argv[1] if len(sys.argv) > 1 else 'f401'
if chip not in ('f401', 'f405', 'f411', 'system-nucleo-f401re',
                'system-default-f401', 'system-default-f405', 'system-default-f411'):
    raise SystemExit('expected a registered chip or system feature')
(root / 'target').mkdir(exist_ok=True)
log = open(root / 'target' / ('lsp-' + chip + '-stderr.log'), 'w')
server = subprocess.Popen(['rust-analyzer'], cwd=root, stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, stderr=log)
messages = queue.Queue()

def reader():
    while True:
        headers = {}
        while True:
            line = server.stdout.readline()
            if not line:
                return
            if line == b'\r\n':
                break
            key, value = line.decode().split(':', 1)
            headers[key.lower()] = value.strip()
        messages.put(json.loads(server.stdout.read(int(headers['content-length']))))

threading.Thread(target=reader, daemon=True).start()
sequence = 0

def send(method, params, request=False):
    global sequence
    payload = dict(jsonrpc='2.0', method=method, params=params)
    if request:
        sequence += 1
        payload['id'] = sequence
    encoded = json.dumps(payload).encode()
    server.stdin.write(f'Content-Length: {len(encoded)}\r\n\r\n'.encode() + encoded)
    server.stdin.flush()
    return sequence

def receive_until(predicate, timeout=180):
    end = time.monotonic() + timeout
    while time.monotonic() < end:
        message = messages.get(timeout=max(0.1, end-time.monotonic()))
        if 'method' in message and 'id' in message:
            encoded = json.dumps(dict(jsonrpc='2.0', id=message['id'], result=None)).encode()
            server.stdin.write(f'Content-Length: {len(encoded)}\r\n\r\n'.encode() + encoded)
            server.stdin.flush()
        if predicate(message):
            return message
    raise TimeoutError('LSP response timed out')

def request(method, params):
    ident = send(method, params, True)
    return receive_until(lambda m: m.get('id') == ident)

def position(text, needle, offset=0):
    index = text.index(needle) + offset
    return dict(line=text[:index].count('\n'), character=len(text[:index].rsplit('\n', 1)[-1]))

try:
    initialized = request('initialize', dict(processId=None, rootUri=root.as_uri(),
        capabilities={'experimental': {'serverStatusNotification': True}},
        initializationOptions={'cargo': {'features': [chip], 'target': 'thumbv7em-none-eabihf'},
                               'checkOnSave': False, 'procMacro': {'enable': True}}))
    print(initialized['result']['serverInfo'], flush=True)
    send('initialized', {})
    text = source.read_text(encoding='utf-8')
    send('textDocument/didOpen', {'textDocument': dict(uri=source.as_uri(), languageId='rust', version=1, text=text)})
    receive_until(lambda m: m.get('method') == 'experimental/serverStatus' and m['params'].get('quiescent'))
    # A query can arrive before macro expansion finishes; retry until hover resolves.
    for attempt in range(20):
        hover = request('textDocument/hover', {'textDocument': {'uri': source.as_uri()},
            'position': position(text, 'cx.local.buffer[0]', len('cx.local.'))})
        if hover.get('result'):
            break
        time.sleep(1)
    assert '[u8; 32]' in json.dumps(hover.get('result')), hover
    print(f'{chip}: hover resolves buffer as &mut [u8; 32]', flush=True)
    for method, needle, offset in [
        ('textDocument/completion', 'cx.local.buffer[0]', len('cx.local.')),
        ('textDocument/definition', 'cx.local.buffer[0]', len('cx.local.')),
        ('textDocument/definition', 'cx.shared.counter.lock', len('cx.shared.')),
    ]:
        result = request(method, {'textDocument': {'uri': source.as_uri()},
            'position': position(text, needle, offset)})['result']
        if method == 'textDocument/completion':
            assert any(item['label'] == 'buffer' for item in result['items']), result
        else:
            assert any(item.get('uri', '').lower() == source.as_uri().lower() for item in result), result
        print(f'{chip}: {method} passed', flush=True)

    changed = text.replace('[u8; 32]', '[u8; 64]').replace('[0; 32]', '[0; 64]')
    send('textDocument/didChange', {'textDocument': {'uri': source.as_uri(), 'version': 2},
                                  'contentChanges': [{'text': changed}]})
    hover = request('textDocument/hover', {'textDocument': {'uri': source.as_uri()},
        'position': position(changed, 'cx.local.buffer[0]', len('cx.local.'))})
    assert '[u8; 64]' in json.dumps(hover.get('result')), hover
    print(f'{chip}: unsaved field type change updates hover to &mut [u8; 64]', flush=True)

    broken = text.replace('cx.local.buffer[0] = 1;', 'cx.local.buffer[0] = 1; missing_ide_probe();')
    send('textDocument/didChange', {'textDocument': {'uri': source.as_uri(), 'version': 3},
                                  'contentChanges': [{'text': broken}]})
    diagnostic = request('textDocument/diagnostic', {'textDocument': {'uri': source.as_uri()}})
    # Native diagnostics inside RTIC expansions vary by RA version. Record this
    # separately; completion/navigation passing does not prove squiggles work.
    print('Unsaved unresolved-function diagnostics:', json.dumps(diagnostic.get('result')), flush=True)
finally:
    server.terminate()
    server.wait(timeout=10)
    log.close()
