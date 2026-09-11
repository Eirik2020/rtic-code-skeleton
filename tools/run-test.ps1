$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspace = Split-Path -Parent $root
$manifest = Join-Path $workspace 'Cargo.toml'
$reportPath = Join-Path $workspace 'docs\test-report.md'
$target = 'thumbv7em-none-eabihf'
$cases = @(
    @{ Name = 'default F401 system'; Features = 'system-default-f401'; Expected = 0 },
    @{ Name = 'default F405 system'; Features = 'system-default-f405'; Expected = 0 },
    @{ Name = 'default F411 system'; Features = 'system-default-f411'; Expected = 0 },
    @{ Name = 'two systems sharing F401 rejected'; Features = 'system-default-f401,system-nucleo-f401re'; Expected = 101; ErrorPattern = 'select at most one system feature' },
    @{ Name = 'systems using different chips rejected'; Features = 'system-default-f401,system-default-f405'; Expected = 101 },
    @{ Name = 'Nucleo system selects F401 and its hardware'; Features = 'system-nucleo-f401re'; Expected = 0 },
    @{ Name = 'Nucleo system accepts explicit matching chip'; Features = 'system-nucleo-f401re,f401'; Expected = 0 },
    @{ Name = 'Nucleo system rejects incompatible chip'; Features = 'system-nucleo-f401re,f405'; Expected = 101 },
    @{ Name = 'Nucleo system rejects generated-body fixture'; Features = 'system-nucleo-f401re,generated-body'; Expected = 101 },
    @{ Name = 'Nucleo system rejects invalid interrupt'; Features = 'system-nucleo-f401re,invalid-interrupt'; Expected = 101 },
    @{ Name = 'f401 shared body'; Features = 'f401'; Expected = 0 },
    @{ Name = 'f405 shared body'; Features = 'f405'; Expected = 0 },
    @{ Name = 'f411 shared body'; Features = 'f411'; Expected = 0 },
    @{ Name = 'f401 generated body'; Features = 'f401,generated-body'; Expected = 0 },
    @{ Name = 'f405 generated body'; Features = 'f405,generated-body'; Expected = 0 },
    @{ Name = 'syn reduction removes f405-only UART4 for f411'; Features = 'f411'; Expected = 0 },
    @{ Name = 'f405-only UART4 retained'; Features = 'f405'; Expected = 0 },
    @{ Name = 'f405-only UART4 removed for f401'; Features = 'f401'; Expected = 0 },
    @{ Name = 'f405-only UART4 removed for f411'; Features = 'f411'; Expected = 0 },
    @{ Name = 'cfg-gated resources compile for f405'; Features = 'f405'; Expected = 0 },
    @{ Name = 'cfg-gated resources removed for f411'; Features = 'f411'; Expected = 0 },
    @{ Name = 'cfg whole task keeps f405 and removes f411 control'; Features = 'f405'; Expected = 0 },
    @{ Name = 'cfg whole task keeps f411 control'; Features = 'f411'; Expected = 0 },
    @{ Name = 'cfg task header keeps f405 and removes f411 control'; Features = 'f405'; Expected = 0 },
    @{ Name = 'cfg task header keeps f411 control'; Features = 'f411'; Expected = 0 },
    @{ Name = 'cfg task resource keeps f405 and removes f411 control'; Features = 'f405'; Expected = 0 },
    @{ Name = 'cfg task resource keeps f411 control'; Features = 'f411'; Expected = 0 },
    @{ Name = 'multiple heads rejected'; Features = 'f401,f405'; Expected = 101 },
    @{ Name = 'invalid interrupt rejected'; Features = 'f401,invalid-interrupt'; Expected = 101 },
    @{ Name = 'no chip rejected'; Features = ''; Expected = 101 }
)

$report = @(
    '# RTIC2 Multi-Head App Composition Test Report',
    '',
    "- Run: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss K')",
    ("- Target: " + '`' + $target + '`'),
    ("- Manifest: " + '`' + $manifest + '`'),
    ''
)
$failed = $false

Push-Location $workspace
try {
    foreach ($case in $cases) {
        Write-Host "Running: $($case.Name)"
        $previousErrorActionPreference = $ErrorActionPreference
        $ErrorActionPreference = 'Continue'
        $cargoArgs = @('check', '--offline', '--manifest-path', $manifest, '--target', $target, '--no-default-features')
        if ($case.Features) {
            $cargoArgs += @('--features', $case.Features)
        }
        $output = & cargo @cargoArgs 2>&1 | Out-String
        $exitCode = $LASTEXITCODE
        $ErrorActionPreference = $previousErrorActionPreference
        $passed = $exitCode -eq $case.Expected
        if ($case.ErrorPattern) { $passed = $passed -and $output.Contains($case.ErrorPattern) }
        if (-not $passed) { $failed = $true }
        $status = if ($passed) { 'PASS' } else { 'FAIL' }
        $report += "## $status - $($case.Name)"
        $command = "cargo check --offline --target $target --no-default-features"
        if ($case.Features) { $command += " --features $($case.Features)" }
        $report += (('`') + $command + ('`'))
        $report += "- Expected exit code: $($case.Expected)"
        $report += "- Actual exit code: $exitCode"
        $report += ''
        $report += '```text'
        $report += $output.TrimEnd()
        $report += '```'
        $report += ''
    }
}
finally {
    Pop-Location
}

$report += if ($failed) { 'Result: FAIL' } else { 'Result: PASS' }
$report | Set-Content -Path $reportPath -Encoding UTF8
Write-Host "Report: $reportPath"
if ($failed) { exit 1 }
