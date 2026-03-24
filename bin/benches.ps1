param(
    [switch]$Doc,
    [string]$DocFile
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
# Temporarily disable LocationChangedAction around Set-Location because tools like
# zoxide/starship/oh-my-posh register a buggy hook that throws when combined with
# $ErrorActionPreference = "Stop". The hook is restored immediately after.
$savedAction = $ExecutionContext.InvokeCommand.LocationChangedAction
$ExecutionContext.InvokeCommand.LocationChangedAction = $null
Set-Location -Path $repoRoot
$ExecutionContext.InvokeCommand.LocationChangedAction = $savedAction

# $env:RUST_BACKTRACE = "1"
if (-not $env:RUSTFLAGS)
{
    $env:RUSTFLAGS = "-Ctarget-cpu=native"
}

function Invoke-BenchCapture
{
    param(
        [Parameter(Mandatory = $true)]
        [string]$BenchName
    )

    $lines = [System.Collections.Generic.List[string]]::new()
    $cmd = "cargo bench -q --all-features --bench $BenchName 2>&1"
    & cmd /d /c $cmd | ForEach-Object {
        $line = $_.ToString()
        $lines.Add($line)
        Write-Host $line
    }

    if ($LASTEXITCODE -ne 0)
    {
        throw "Benchmark '$BenchName' failed with exit code $LASTEXITCODE."
    }

    return ($lines -join [Environment]::NewLine)
}

$runDate = Get-Date -Format "yyyy-MM-dd HH:mm:ss zzz"

try
{
    $os = (Get-CimInstance Win32_OperatingSystem -ErrorAction Stop).Caption
} catch
{
    $os = [System.Runtime.InteropServices.RuntimeInformation]::OSDescription
}

try
{
    $cpu = (Get-CimInstance Win32_Processor -ErrorAction Stop | Select-Object -First 1 -ExpandProperty Name).Trim()
} catch
{
    $cpu = $env:PROCESSOR_IDENTIFIER
}
if (-not $cpu)
{
    $cpu = "unknown"
}

try
{
    $ramGiB = [math]::Round((Get-CimInstance Win32_ComputerSystem -ErrorAction Stop).TotalPhysicalMemory / 1GB, 1)
    $ram = "$ramGiB GiB"
} catch
{
    $ram = "unknown"
}

$rustcVersion = (& rustc --version 2>$null)
if (-not $rustcVersion)
{
    $rustcVersion = "rustc (not found)"
}

$cargoVersion = (& cargo --version 2>$null)
if (-not $cargoVersion)
{
    $cargoVersion = "cargo (not found)"
}

$gitCommit = (& git rev-parse --short HEAD 2>$null)
if (-not $gitCommit)
{
    $gitCommit = "unknown"
}

Write-Host "Benchmark run metadata:"
Write-Host "  Run date: $runDate"
Write-Host "  OS: $os"
Write-Host "  CPU: $cpu"
Write-Host "  RAM: $ram"
Write-Host "  Toolchain: $rustcVersion"
Write-Host "  Cargo: $cargoVersion"
Write-Host "  Git commit: $gitCommit"
Write-Host "  RUSTFLAGS: $env:RUSTFLAGS"
Write-Host ""

$brwvOutput = Invoke-BenchCapture -BenchName "brwv"
$brwuOutput = Invoke-BenchCapture -BenchName "brwu"

if ($Doc -or $DocFile)
{
    $docTemplate = @'
# Benchmarks

This document tracks the current benchmark targets defined in `Cargo.toml`:

- `brwv` -> `benches/borrowed-real-world-validated.rs`
- `brwu` -> `benches/borrowed-real-world-unvalidated.rs`

## Benchmark environment

- Run date: `{{RUN_DATE}}`
- OS: `{{OS}}`
- CPU: `{{CPU}}`
- RAM: `{{RAM}}`
- Toolchain: `{{RUSTC}}`
- Cargo: `{{CARGO}}`
- Git commit: `{{GIT_COMMIT}}`

## Commands

```powershell
$env:RUSTFLAGS="-Ctarget-cpu=native"
cargo bench -q --all-features --bench brwv
cargo bench -q --all-features --bench brwu
```

## Results: `brwv` (validated parsers)

```txt
{{BRWV_OUTPUT}}
```

## Results: `brwu` (unvalidated parsers)

```txt
{{BRWU_OUTPUT}}
```

These numbers are synthetic and depend on hardware, toolchain version, and CPU frequency scaling.
'@

    $docText = $docTemplate
    $docText = $docText.Replace('{{RUN_DATE}}', $runDate)
    $docText = $docText.Replace('{{OS}}', $os)
    $docText = $docText.Replace('{{CPU}}', $cpu)
    $docText = $docText.Replace('{{RAM}}', $ram)
    $docText = $docText.Replace('{{RUSTC}}', $rustcVersion)
    $docText = $docText.Replace('{{CARGO}}', $cargoVersion)
    $docText = $docText.Replace('{{GIT_COMMIT}}', $gitCommit)
    $docText = $docText.Replace('{{BRWV_OUTPUT}}', $brwvOutput)
    $docText = $docText.Replace('{{BRWU_OUTPUT}}', $brwuOutput)

    if ($DocFile)
    {
        Set-Content -Path $DocFile -Value $docText -Encoding UTF8
        Write-Host "Wrote markdown doc block to $DocFile"
    }

    if ($Doc)
    {
        Write-Output $docText
    }
}
