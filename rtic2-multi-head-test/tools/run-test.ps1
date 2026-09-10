$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspace = Split-Path -Parent $root
$manifest = Join-Path $workspace 'Cargo.toml'
$reportPath = Join-Path $workspace 'docs\test-report.md'
$target = 'thumbv7em-none-eabihf'
$cases = @(
    @{ Name = 'f401 shared body'; Features = 'f401'; Expected = 0 },
    @{ Name = 'f405 shared body'; Features = 'f405'; Expected = 0 },
    @{ Name = 'f411 shared body'; Features = 'f411'; Expected = 0 },
    @{ Name = 'f401 generated body'; Features = 'f401,generated-body'; Expected = 0 },
    @{ Name = 'f405 generated body'; Features = 'f405,generated-body'; Expected = 0 },
    @{ Name = 'syn reduction removes f405-only UART4 for f411'; Features = 'f411'; Expected = 0 },
    @{ Name = 'f405-only UART4 retained'; Features = 'f405'; Expected = 0 },
    @{ Name = 'f405-only UART4 removed for f401'; Features = 'f401'; Expected = 0 },
    @{ Name = 'f405-only UART4 removed for f411'; Features = 'f411'; Expected = 0 },
    @{ Name = 'multiple heads rejected'; Features = 'f401,f405'; Expected = 101 },
    @{ Name = 'invalid interrupt rejected'; Features = 'f401,invalid-interrupt'; Expected = 101 }
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
        $output = & cargo check --offline --manifest-path $manifest --target $target --no-default-features --features $case.Features 2>&1 | Out-String
        $exitCode = $LASTEXITCODE
        $ErrorActionPreference = $previousErrorActionPreference
        $passed = $exitCode -eq $case.Expected
        if (-not $passed) { $failed = $true }
        $status = if ($passed) { 'PASS' } else { 'FAIL' }
        $report += "## $status - $($case.Name)"
        $report += (('`') + "cargo check --offline --target $target --no-default-features --features $($case.Features)" + ('`'))
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
