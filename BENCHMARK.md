# Benchmarks

Current benchmark reports:

- [Windows](benchmarks/windows.md)
- [macOS](benchmarks/macos.md)
- [WSL](benchmarks/wsl.md)

Each report includes the hardware, toolchain, Git commit, build flags, and results for every feature configuration.

## Findings from 2026-09-09

These findings use methodology v2 and median times for `Sample File (no comments)`. The median is the middle measured value. Each comparison uses results from the same platform:

- The default date/time library, `jiff`, takes the least time for validated typed parsing on all three platforms. `time` takes approximately 1% longer on Windows, 15% longer on WSL, and 3% longer on macOS.
- Validation adds approximately 4–5% to the time for typed parsing with `jiff` on each platform. Other parsers differ, so removing validation does not always reduce time.
- On the M1 Pro, typed parsing with `jiff` takes 26% longer than raw parsing in the same configuration. The increases are 81% on Windows and 48% on WSL. These percentages describe the additional cost of typed parsing, not absolute CPU speed.
- Validated typed parsing with `chrono` takes approximately 29% longer than `jiff` on Windows, 71% longer on WSL, and 27% longer on macOS. On WSL, validated Chrono parsing also takes 15% longer than unvalidated Chrono parsing. The cause of this difference is unknown.

These results support `jiff` as the default for this workload. Each platform has one measurement session. Small differences require more measurements before a performance decision.

## Running benchmarks

Run `just bench` to execute both benchmark targets across five configurations: `no-features`, `jiff`, `time`, `chrono`, and `parquet`. Each configuration disables default features and enables only its named feature, if any.

The targets defined in [Cargo.toml](Cargo.toml) are:

- [`brwv`](benches/borrowed-real-world-validated.rs): validated parsers.
- [`brwu`](benches/borrowed-real-world-unvalidated.rs): unvalidated parsers.

Both suites cover raw, simple, and typed parsing. Parquet benchmarks run in the `parquet` configuration.

The command writes `benchmarks/<platform>.md`, detecting Windows, macOS, WSL, or Linux automatically. Windows requires PowerShell 7 (`pwsh`). Unless `RUSTFLAGS` is already set, the runner uses `-Ctarget-cpu=native`.

## Interpreting results

Methodology v2 passes the seven extracted field values through `divan::black_box` so the optimizer must preserve them. Earlier reports consumed only the field count; rerun all platforms before comparing the revised benchmark. A change in timings across these methodologies is not evidence of a parser regression.

Compare validated and unvalidated parsers using matching input labels. Both suites include `Line A`, `Line B`, `Lines A+B`, and `Sample File (no comments)` with the same six records. The validated suite additionally measures `Sample File (with comments)`, which includes two comment lines. Times are per benchmark invocation: one line, two lines, or the entire sample, respectively.

The inputs are small, repeated, in-memory fixtures. These benchmarks measure parsing and field extraction, not file I/O, decompression, or Parquet encoding and writing.

For platform comparisons, use the same source revision, Rust toolchain, and build flags. Repeat runs on an otherwise idle machine and retain each report using the runner's output option (`--output` in `bin/benches.sh`, `-OutputPath` in `bin/benches.ps1`). Use medians and check whether differences persist across runs. CPU affinity and power settings are not controlled by the runner; keep them consistent and record any manual settings alongside results.

These are synthetic benchmarks. Results depend on hardware, toolchain, feature configuration, and CPU frequency scaling. Check each report's environment before comparing results across platforms or runs.
