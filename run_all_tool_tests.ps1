Param(
    [switch]$FailFast
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$toolsRoot = Join-Path $root "tools"

$toolDirs = Get-ChildItem $toolsRoot -Directory | Where-Object {
    Test-Path (Join-Path $_.FullName "Cargo.toml")
}

$failed = @()

foreach ($dir in $toolDirs) {
    Write-Host "Running tests for" $dir.Name "..." -ForegroundColor Cyan
    Push-Location $dir.FullName
    try {
        cargo test --all-targets --all-features
        Pop-Location
    } catch {
        Pop-Location
        Write-Host "Tests failed for" $dir.Name -ForegroundColor Red
        $failed += $dir.Name
        if ($FailFast) {
            Write-Host "FailFast is enabled; aborting." -ForegroundColor Red
            if ($failed.Count -gt 0) {
                Write-Host "Failed tool crates:" ($failed -join ", ")
            }
            exit 1
        }
    }
}

if ($failed.Count -gt 0) {
    Write-Host "Some tool crates failed tests:" ($failed -join ", ") -ForegroundColor Red
    exit 1
}

Write-Host "All tool crates passed tests." -ForegroundColor Green
exit 0

