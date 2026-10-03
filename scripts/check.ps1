#Requires -Version 7.0
[CmdletBinding()]
param(
    [ValidateSet('Quick', 'Full')]
    [string]$Mode = 'Full'
)

$ErrorActionPreference = 'Stop'
$taskRepository = Split-Path -Parent $PSScriptRoot

function Invoke-CheckedCommand {
    param([string]$Executable, [string[]]$Arguments)
    Write-Host "> $Executable $($Arguments -join ' ')"
    & $Executable @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Executable failed with exit code $LASTEXITCODE"
    }
}

Push-Location $taskRepository
try {
    Invoke-CheckedCommand 'python' @('scripts/check_repository.py')
    Invoke-CheckedCommand 'git' @('diff', '--check')
    Invoke-CheckedCommand 'git' @('diff', '--cached', '--check')
    Invoke-CheckedCommand 'cargo' @('fmt', '--all', '--', '--check')
    Invoke-CheckedCommand 'cargo' @('clippy', '--workspace', '--all-targets', '--locked', '--', '-D', 'warnings')
    if ($Mode -eq 'Full') {
        Invoke-CheckedCommand 'cargo' @('test', '--workspace', '--locked')
        Invoke-CheckedCommand 'cargo' @('build', '--workspace', '--release', '--locked')
        $taskPreviousRustdocFlags = $env:RUSTDOCFLAGS
        try {
            $env:RUSTDOCFLAGS = '-D warnings'
            Invoke-CheckedCommand 'cargo' @('doc', '--workspace', '--no-deps', '--locked')
        }
        finally {
            $env:RUSTDOCFLAGS = $taskPreviousRustdocFlags
        }
    }
    Write-Host "PASS: $Mode repository and Rust checks. Hardware and drivers were not exercised."
}
finally {
    Pop-Location
}
