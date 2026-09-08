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
echo %*>>"%CARGO_ARGUMENTS_FILE%"
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
    $text = Get-Content $output -Raw

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
    Write-Host 'PowerShell benchmark writer tests passed.'
} finally
{
    $env:PATH = $originalPath
    Remove-Item Env:FAIL_CARGO_AT -ErrorAction SilentlyContinue
    Remove-Item Env:CARGO_ARGUMENTS_FILE -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force $temp -ErrorAction SilentlyContinue
}
