#[shared]
struct Shared {
    counter: u32,
}

#[local]
struct Local {
    buffer: [u8; 32],
}

#[init]
fn init(_: init::Context) -> (Shared, Local) {
    (Shared { counter: 0 }, Local { buffer: [0; 32] })
}

#[task(binds = USART1, shared = [counter], local = [buffer])]
fn usart1(mut cx: usart1::Context) {
    cx.shared.counter.lock(|counter| {
        *counter += 1;
    });
    cx.local.buffer[0] = 1;
}

#[cfg(feature = "f405")]
#[task(binds = UART4)]
fn f405_uart4(_: f405_uart4::Context) {}
