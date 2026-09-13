$ErrorActionPreference = "Stop"

$temp = Join-Path ([System.IO.Path]::GetTempPath()) ("test-benches-" + [guid]::NewGuid())
$fakeBin = Join-Path $temp "bin"
New-Item -ItemType Directory -Path $fakeBin | Out-Null
$originalPath = $env:PATH

try
{
    @'
@echo off
if "%~1"=="--version" (
    echo cargo 1.2.3
    exit /b 0
)
(echo %*)>>"%CARGO_ARGUMENTS_FILE%"
for /f %%i in ('find /c /v "" ^< "%CARGO_ARGUMENTS_FILE%"') do set INVOCATION=%%i
if "%FAIL_CARGO_AT%"=="%INVOCATION%" exit /b 23
echo benchmark output: %*
'@ | Set-Content -Path (Join-Path $fakeBin "cargo.cmd") -Encoding ascii

    @'
@echo off
echo rustc 1.2.3
'@ | Set-Content -Path (Join-Path $fakeBin "rustc.cmd") -Encoding ascii

    @'
@echo off
if "%~1"=="rev-parse" if "%~2"=="--short" (
    echo abc1234
    exit /b 0
)
exit /b 1
'@ | Set-Content -Path (Join-Path $fakeBin "git.cmd") -Encoding ascii

    $env:PATH = $fakeBin + [System.IO.Path]::PathSeparator + $originalPath
    $env:PATHEXT = ".CMD;" + $env:PATHEXT
    $argumentsFile = Join-Path $temp "cargo-arguments.txt"
    $env:CARGO_ARGUMENTS_FILE = $argumentsFile
    $output = Join-Path $temp "windows.md"
    & "$PSScriptRoot/benches.ps1" -TestPlatform windows -OutputPath $output
    $text = (Get-Content $output -Raw).Replace("`r`n", "`n")

    if ($text -notmatch '- Platform: `windows`') { throw 'platform missing' }
    if ([regex]::Matches($text, '(?m)^## Configuration:').Count -ne 5) { throw 'matrix incomplete' }

    $expectedConfigurations = @('no-features', 'jiff', 'time', 'chrono', 'parquet')
    foreach ($configuration in $expectedConfigurations)
    {
        if ($text -notmatch "(?m)^## Configuration: $configuration$") { throw "configuration missing: $configuration" }
    }

    if ([regex]::Matches($text, '(?m)^### `brwv` \(validated parsers\)$').Count -ne 5) { throw 'brwv matrix incomplete' }
    if ([regex]::Matches($text, '(?m)^### `brwu` \(unvalidated parsers\)$').Count -ne 5) { throw 'brwu matrix incomplete' }

    $expectedArguments = @(
        'bench -q --no-default-features --bench brwv',
        'bench -q --no-default-features --bench brwu',
        'bench -q --no-default-features --features jiff --bench brwv',
        'bench -q --no-default-features --features jiff --bench brwu',
        'bench -q --no-default-features --features time --bench brwv',
        'bench -q --no-default-features --features time --bench brwu',
        'bench -q --no-default-features --features chrono --bench brwv',
        'bench -q --no-default-features --features chrono --bench brwu',
        'bench -q --no-default-features --features parquet --bench brwv',
        'bench -q --no-default-features --features parquet --bench brwu'
    )
    $actualArguments = @(Get-Content $argumentsFile)
    if ($actualArguments.Count -ne $expectedArguments.Count) { throw 'Cargo invocation count incorrect' }
    for ($index = 0; $index -lt $expectedArguments.Count; $index++)
    {
        if ($actualArguments[$index] -ne $expectedArguments[$index])
        {
            throw "Cargo invocation $index incorrect: $($actualArguments[$index])"
        }
    }

    Set-Content -Path $output -Value 'existing report' -Encoding utf8
    Remove-Item -Path $argumentsFile -ErrorAction SilentlyContinue
    $env:FAIL_CARGO_AT = '8'
    try
    {
        & "$PSScriptRoot/benches.ps1" -TestPlatform windows -OutputPath $output
        throw 'failed Cargo did not fail the writer'
    } catch
    {
        if ($_.Exception.Message -eq 'failed Cargo did not fail the writer') { throw }
        if ($_.Exception.Message -notmatch "configuration 'chrono'.*benchmark target 'brwu'")
        {
            throw "failure lacks benchmark context: $($_.Exception.Message)"
        }
    }

    if ((Get-Content $output -Raw).Trim() -ne 'existing report') { throw 'failed Cargo replaced existing output' }
    Remove-Item Env:FAIL_CARGO_AT
    Remove-Item -LiteralPath $argumentsFile
    $series = Join-Path $temp 'series.md'
    & "$PSScriptRoot/benches.ps1" -TestPlatform windows -OutputPath $series -Repetitions 2 -ConfigurationOrder chrono,no-features -BenchArgs @('--sample-count', '2', '--sample-size', '1') -AllocationProfile
    foreach ($repetition in 1..2)
    {
        $report = Join-Path $temp ("series.run-{0:D3}.md" -f $repetition)
        $reportText = Get-Content -LiteralPath $report -Raw
        if ($reportText -notmatch '- Configuration order: `chrono,no-features`') { throw 'configuration order missing' }
        if ($reportText -notmatch "- Repetition: $repetition / 2") { throw 'repetition missing' }
        if ($reportText -notmatch 'bench-alloc') { throw 'allocation profile not recorded' }
    }
    $seriesArguments = @(Get-Content -LiteralPath $argumentsFile)
    if ($seriesArguments.Count -ne 8) { throw 'repetition matrix incomplete' }
    if ($seriesArguments[0].Replace('"', '') -ne 'bench -q --no-default-features --features chrono,bench-alloc --bench brwv -- --sample-count 2 --sample-size 1') { throw "ordered/profiled arguments incorrect: $($seriesArguments[0])" }
    try
    {
        & "$PSScriptRoot/benches.ps1" -TestPlatform windows -OutputPath $series -Repetitions 2
        throw 'repetition collision accepted'
    } catch
    {
        if ($_.Exception.Message -eq 'repetition collision accepted') { throw }
        if ($_.Exception.Message -notmatch 'already exists') { throw }
    }
    if (@(Get-Content -LiteralPath $argumentsFile).Count -ne 8) { throw 'collision ran benchmarks before rejection' }

    Remove-Item -LiteralPath $argumentsFile
    $env:FAIL_CARGO_AT = '6'
    try
    {
        & "$PSScriptRoot/benches.ps1" -TestPlatform windows -OutputPath (Join-Path $temp 'partial.md') -Repetitions 2 -ConfigurationOrder no-features,jiff
        throw 'later repetition failure accepted'
    } catch
    {
        if ($_.Exception.Message -eq 'later repetition failure accepted') { throw }
        if ($_.Exception.Message -notmatch "configuration 'no-features'.*benchmark target 'brwu'") { throw }
    }
    if (-not (Test-Path -LiteralPath (Join-Path $temp 'partial.run-001.md'))) { throw 'completed repetition lost' }
    if (Test-Path -LiteralPath (Join-Path $temp 'partial.run-002.md')) { throw 'partial repetition published' }
    Remove-Item Env:FAIL_CARGO_AT
    foreach ($invalidOrder in @(@('jiff', 'jiff'), @('unknown')))
    {
        try
        {
            & "$PSScriptRoot/benches.ps1" -TestPlatform windows -OutputPath (Join-Path $temp 'invalid.md') -ConfigurationOrder $invalidOrder
            throw 'invalid configuration order accepted'
        } catch
        {
            if ($_.Exception.Message -eq 'invalid configuration order accepted') { throw }
        }
        if (Test-Path -LiteralPath (Join-Path $temp 'invalid.md')) { throw 'invalid order published a report' }
    }
    try
    {
        & "$PSScriptRoot/benches.ps1" -TestPlatform windows -BenchArgs @('--sample-count', '2')
        throw 'implicit smoke output accepted'
    } catch
    {
        if ($_.Exception.Message -eq 'implicit smoke output accepted') { throw }
        if ($_.Exception.Message -notmatch 'require -OutputPath') { throw }
    }
    # Fail the first fake benchmark if protection is absent, so this regression
    # test can never replace the real platform report.
    $env:CARGO_ARGUMENTS_FILE = Join-Path $temp 'subset-arguments.txt'
    $env:FAIL_CARGO_AT = '1'
    try
    {
        & "$PSScriptRoot/benches.ps1" -TestPlatform windows -ConfigurationOrder jiff
        throw 'implicit subset output accepted'
    } catch
    {
        if ($_.Exception.Message -notmatch 'require -OutputPath') { throw }
    }
    if (Test-Path -LiteralPath $env:CARGO_ARGUMENTS_FILE) { throw 'subset ran benchmarks before rejection' }
    Remove-Item Env:FAIL_CARGO_AT
    foreach ($name in @('DIVAN_MIN_TIME', 'DIVAN_MAX_TIME', 'DIVAN_SKIP_EXT_TIME'))
    {
        $savedValue = [Environment]::GetEnvironmentVariable($name)
        try
        {
            [Environment]::SetEnvironmentVariable($name, '1')
            $env:FAIL_CARGO_AT = '1'
            try
            {
                & "$PSScriptRoot/benches.ps1" -TestPlatform windows
                throw "implicit $name output accepted"
            } catch
            {
                if ($_.Exception.Message -notmatch 'require -OutputPath') { throw }
            }
            if (Test-Path -LiteralPath $env:CARGO_ARGUMENTS_FILE) { throw 'environment override ran benchmarks before rejection' }
            Remove-Item Env:FAIL_CARGO_AT
            $metadataOutput = Join-Path $temp "$name.md"
            & "$PSScriptRoot/benches.ps1" -TestPlatform windows -OutputPath $metadataOutput -ConfigurationOrder jiff
            if ((Get-Content -LiteralPath $metadataOutput -Raw) -notmatch "- ${name}: ``1``") { throw "$name metadata missing" }
            Remove-Item -LiteralPath $env:CARGO_ARGUMENTS_FILE
        } finally
        {
            [Environment]::SetEnvironmentVariable($name, $savedValue)
        }
    }
    Write-Host 'PowerShell benchmark writer tests passed.'
} finally
{
    $env:PATH = $originalPath
    Remove-Item Env:FAIL_CARGO_AT -ErrorAction SilentlyContinue
    Remove-Item Env:CARGO_ARGUMENTS_FILE -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force $temp -ErrorAction SilentlyContinue
}
