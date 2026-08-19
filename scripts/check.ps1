Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw 'Cargo was not found. Install Rust 1.93 or later and ensure cargo is on PATH.'
}

function Invoke-Cargo {
    param([string[]] $Arguments)

    & cargo @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw ('cargo {0} failed with exit code {1}.' -f ($Arguments -join ' '), $LASTEXITCODE)
    }
}

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$previousLocation = (Get-Location).Path
$hadRustdocFlags = Test-Path Env:RUSTDOCFLAGS
$previousRustdocFlags = if ($hadRustdocFlags) {
    (Get-Item Env:RUSTDOCFLAGS).Value
} else {
    $null
}

try {
    Set-Location -LiteralPath $repositoryRoot

    Invoke-Cargo @('fmt', '--check')
    Invoke-Cargo @('test', '--all-features')
    Invoke-Cargo @('clippy', '--all-targets', '--all-features', '--', '-D', 'warnings')

    $env:RUSTDOCFLAGS = '-D warnings'
    Invoke-Cargo @('doc', '--no-deps', '--all-features')

    Set-Location -LiteralPath (Join-Path $repositoryRoot 'web')
    Invoke-Cargo @('test')
} finally {
    try {
        Set-Location -LiteralPath $previousLocation
    } finally {
        if ($hadRustdocFlags) {
            $env:RUSTDOCFLAGS = $previousRustdocFlags
        } else {
            Remove-Item Env:RUSTDOCFLAGS -ErrorAction SilentlyContinue
        }
    }
}
