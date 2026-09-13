[CmdletBinding()]
param(
    [string]$OutputPath = "benchmarks/windows.md",
    [ValidateRange(1, 10000)]
    [int]$Repetitions = 1,
    [ValidateSet('no-features', 'jiff', 'time', 'chrono', 'parquet')]
    [string[]]$ConfigurationOrder = @('no-features', 'jiff', 'time', 'chrono', 'parquet'),
    [string[]]$BenchArgs = @(),
    [switch]$AllocationProfile,
    [ValidateSet("windows")]
    [string]$TestPlatform
)

$ErrorActionPreference = "Stop"
$platform = "windows"

if (($BenchArgs.Count -gt 0 -or $AllocationProfile -or $Repetitions -gt 1 -or $PSBoundParameters.ContainsKey('ConfigurationOrder') -or $env:DIVAN_SAMPLE_COUNT -or $env:DIVAN_SAMPLE_SIZE -or $env:DIVAN_MIN_TIME -or $env:DIVAN_MAX_TIME -or $env:DIVAN_SKIP_EXT_TIME -or $env:CF_BENCH_RECORDS) -and -not $PSBoundParameters.ContainsKey('OutputPath'))
{
    throw 'Custom runs require -OutputPath to protect published platform reports.'
}
if ($ConfigurationOrder.Count -eq 0 -or @($ConfigurationOrder | Select-Object -Unique).Count -ne $ConfigurationOrder.Count)
{
    throw 'ConfigurationOrder must be a nonempty list without duplicates.'
}

if (-not $IsWindows -and $TestPlatform -ne "windows")
{
    throw "Unsupported host platform. Use -TestPlatform windows only for tests."
}

$repoRoot = Split-Path -Parent $PSScriptRoot
$savedAction = $ExecutionContext.InvokeCommand.LocationChangedAction
$ExecutionContext.InvokeCommand.LocationChangedAction = $null
Set-Location -Path $repoRoot
$ExecutionContext.InvokeCommand.LocationChangedAction = $savedAction

if (-not $env:RUSTFLAGS)
{
    $env:RUSTFLAGS = "-Ctarget-cpu=native"
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
if (-not $cpu) { $cpu = "unknown" }

try
{
    $ramGiB = [math]::Round((Get-CimInstance Win32_ComputerSystem -ErrorAction Stop).TotalPhysicalMemory / 1GB, 1)
    $ram = "$ramGiB GiB"
} catch
{
    $ram = "unknown"
}

$rustcVersion = (& rustc --version 2>$null)
if (-not $rustcVersion) { $rustcVersion = "unknown" }
$cargoVersion = (& cargo --version 2>$null)
if (-not $cargoVersion) { $cargoVersion = "unknown" }
$gitCommit = (& git rev-parse --short HEAD 2>$null)
if (-not $gitCommit) { $gitCommit = "unknown" }
$gitStatus = (& git status --porcelain 2>$null)
$worktreeState = if ($LASTEXITCODE -ne 0) { 'unknown' } elseif ($gitStatus) { 'dirty' } else { 'clean' }

$configurations = @(
    @{ Label = "no-features"; Features = @("--no-default-features") },
    @{ Label = "jiff"; Features = @("--no-default-features", "--features", "jiff") },
    @{ Label = "time"; Features = @("--no-default-features", "--features", "time") },
    @{ Label = "chrono"; Features = @("--no-default-features", "--features", "chrono") },
    @{ Label = "parquet"; Features = @("--no-default-features", "--features", "parquet") }
)
$configurations = @($ConfigurationOrder | ForEach-Object {
    $label = $_
    $configuration = $configurations | Where-Object Label -eq $label
    if ($AllocationProfile)
    {
        $features = if ($label -eq 'no-features') { 'bench-alloc' } else { "$label,bench-alloc" }
        $configuration.Features = @('--no-default-features', '--features', $features)
    }
    $configuration
})

$outputPaths = @(foreach ($repetition in 1..$Repetitions)
{
    if ($Repetitions -eq 1) { $OutputPath }
    else
    {
        $directory = Split-Path -Parent $OutputPath
        if (-not $directory) { $directory = '.' }
        $stem = [System.IO.Path]::GetFileNameWithoutExtension($OutputPath)
        $extension = [System.IO.Path]::GetExtension($OutputPath)
        $path = Join-Path $directory ("$stem.run-{0:D3}$extension" -f $repetition)
        if (Test-Path -LiteralPath $path) { throw "Repetition report already exists: $path" }
        $path
    }
})

foreach ($repetition in 1..$Repetitions)
{
$OutputPath = $outputPaths[$repetition - 1]
$runDate = Get-Date -Format "yyyy-MM-dd HH:mm:ss zzz"

$outputDirectory = Split-Path -Parent $OutputPath
if (-not $outputDirectory) { $outputDirectory = "." }
New-Item -ItemType Directory -Path $outputDirectory -Force | Out-Null
$temporaryPath = Join-Path $outputDirectory (".benches." + [guid]::NewGuid())

try
{
    $writer = [System.IO.StreamWriter]::new($temporaryPath, $false, [System.Text.UTF8Encoding]::new($false))
    try
    {
        $writer.WriteLine("# Benchmarks: $platform")
        $writer.WriteLine()
        $writer.WriteLine("## Benchmark environment")
        $writer.WriteLine()
        $writer.WriteLine('- Platform: `' + $platform + '`')
        $writer.WriteLine('- Run date: `' + $runDate + '`')
        $writer.WriteLine('- OS: `' + $os + '`')
        $writer.WriteLine('- CPU: `' + $cpu + '`')
        $writer.WriteLine('- RAM: `' + $ram + '`')
        $writer.WriteLine('- Toolchain: `' + $rustcVersion + '`')
        $writer.WriteLine('- Cargo: `' + $cargoVersion + '`')
        $writer.WriteLine('- Git commit: `' + $gitCommit + '`')
        $writer.WriteLine('- Git worktree: `' + $worktreeState + '`')
        $writer.WriteLine('- RUSTFLAGS: `' + $env:RUSTFLAGS + '`')
        $writer.WriteLine('- Configuration order: `' + ($ConfigurationOrder -join ',') + '`')
        $writer.WriteLine("- Repetition: $repetition / $Repetitions")
        $writer.WriteLine('- Benchmark arguments: `' + ($BenchArgs -join ' ') + '`')
        $writer.WriteLine('- Allocation profiling: `' + $(if ($AllocationProfile) { 'bench-alloc (instrumented timing)' } else { 'disabled' }) + '`')
        foreach ($name in @('DIVAN_SAMPLE_COUNT', 'DIVAN_SAMPLE_SIZE', 'DIVAN_MIN_TIME', 'DIVAN_MAX_TIME', 'DIVAN_SKIP_EXT_TIME', 'DIVAN_BYTES_FORMAT', 'CF_BENCH_RECORDS', 'CARGO_TARGET_DIR'))
        {
            $value = [System.Environment]::GetEnvironmentVariable($name)
            $writer.WriteLine('- ' + $name + ': `' + $(if ($value) { $value } else { 'unset; see .cargo/config.toml and workload defaults' }) + '`')
        }

        foreach ($configuration in $configurations)
        {
            $writer.WriteLine()
            $writer.WriteLine("## Configuration: $($configuration.Label)")
            foreach ($benchmark in @(
                @{ Name = "brwv"; Description = "validated parsers" },
                @{ Name = "brwu"; Description = "unvalidated parsers" }
            ))
            {
                $writer.WriteLine()
                $writer.WriteLine('### `' + $benchmark.Name + '` (' + $benchmark.Description + ')')
                $writer.WriteLine()
                $writer.WriteLine('```txt')
                $arguments = @("bench", "-q") + $configuration.Features + @("--bench", $benchmark.Name)
                if ($BenchArgs.Count -gt 0) { $arguments += @('--') + $BenchArgs }
                $result = & cargo @arguments 2>&1
                $exitCode = $LASTEXITCODE
                $result | ForEach-Object {
                    $line = $_.ToString()
                    Write-Host $line
                    $writer.WriteLine($line)
                }
                if ($exitCode -ne 0)
                {
                    throw "Configuration '$($configuration.Label)', benchmark target '$($benchmark.Name)' failed with exit code $exitCode."
                }
                $writer.WriteLine('```')
            }
        }
    } finally
    {
        $writer.Dispose()
    }

    if ($Repetitions -eq 1 -and [System.IO.File]::Exists($OutputPath))
    {
        $backupPath = Join-Path $outputDirectory (".benches-backup." + [guid]::NewGuid())
        try
        {
            [System.IO.File]::Replace($temporaryPath, $OutputPath, $backupPath)
        } finally
        {
            try
            {
                [System.IO.File]::Delete($backupPath)
            } catch
            {
                Write-Warning "Could not remove benchmark backup '$backupPath': $($_.Exception.Message)"
            }
        }
    } else
    {
        [System.IO.File]::Move($temporaryPath, $OutputPath)
    }
    Write-Host "Wrote benchmark report to $OutputPath"
} finally
{
    Remove-Item -Path $temporaryPath -Force -ErrorAction SilentlyContinue
}
}
