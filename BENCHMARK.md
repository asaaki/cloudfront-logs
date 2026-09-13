# Benchmarks

Current benchmark reports:

- [Windows](benchmarks/windows.md)
- [macOS](benchmarks/macos.md)
- [WSL](benchmarks/wsl.md)

Each report includes the hardware, toolchain, Git commit, build flags, and results for every feature configuration.

## Running benchmarks

Run `just bench` to execute both benchmark targets across five configurations: `no-features`, `jiff`, `time`, `chrono`, and `parquet`. Each configuration disables default features and enables only its named feature, if any.

The targets defined in [Cargo.toml](Cargo.toml) are:

- [`brwv`](benches/borrowed-real-world-validated.rs): validated parsers.
- [`brwu`](benches/borrowed-real-world-unvalidated.rs): unvalidated parsers.

Both suites cover raw, simple, and typed parsing. Parquet benchmarks run in the `parquet` configuration. The original v2 workloads run by default. The new representative groups are opt-in with Divan's `--ignored` option (`--include-ignored` runs both groups).

The command writes `benchmarks/<platform>.md`, detecting Windows, macOS, WSL, or Linux automatically. Windows requires PowerShell 7 (`pwsh`). Unless `RUSTFLAGS` is already set, the runner uses `-Ctarget-cpu=native`.

## Interpreting results

Methodology v2 passes the seven extracted field values through `divan::black_box` so the optimizer must preserve them. Earlier reports consumed only the field count; rerun all platforms before comparing the revised benchmark. A change in timings across these methodologies is not evidence of a parser regression.

Compare validated and unvalidated parsers using matching input labels. Both suites include `Line A`, `Line B`, `Lines A+B`, and `Sample File (no comments)` with the same six records. The validated suite additionally measures `Sample File (with comments)`, which includes two comment lines. Times are per benchmark invocation: one line, two lines, or the entire sample, respectively.

The inputs are small, repeated, in-memory fixtures. These benchmarks measure parsing and field extraction, not file I/O, decompression, or Parquet encoding and writing.

For platform comparisons, use the same source revision, Rust toolchain, and build flags. Repeat runs on an otherwise idle machine and retain each report using the runner's output option (`--output` in `bin/benches.sh`, `-OutputPath` in `bin/benches.ps1`). Use medians and check whether differences persist across runs. CPU affinity and power settings are not controlled by the runner; keep them consistent and record any manual settings alongside results.

These are synthetic benchmarks. Results depend on hardware, toolchain, feature configuration, and CPU frequency scaling. Check each report's environment before comparing results across platforms or runs.

## Representative workloads (v1)

[`benches/corpus.rs`](benches/corpus.rs) generates public, synthetic records; no log fixture is downloaded or checked in. Dates, times, statuses, byte counts, paths, and request IDs vary deterministically. `MixedHot` contains 100 records. `MixedLarge` defaults to 32,700 records (tens of MiB), controlled by `CF_BENCH_RECORDS`. Increase this count to explore larger working sets; it does not guarantee a cold cache or that the corpus exceeds a particular CPU's cache. Generation and input setup happen outside the timed region.

The focused 100-record profiles are `Ipv4`, `Ipv6`, `Forwarded` (one, two, and three addresses, including sockets and leading-zero IPv4), `UnknownResults` (all three result fields), `LongFields` (path, agent, query), and `OptionalValues` (populated referer, query, cookie, encryption and range fields). `Mixed` cycles through those six profiles. These are controlled cases, not an estimate of production traffic frequencies. Some owned fields have stricter types than their borrowed equivalents; corpus inputs stay in their shared accepted domain.

| Group | Timed work | Unit and interpretation |
| --- | --- | --- |
| `representative::full_record` | Parse, observe all source and derived fields, destroy each record | One corpus scan. Raw/simple/typed/Parquet representations. Includes conditional allocations and destruction, plus observer overhead. |
| `representative::filtering` | Eager full conversion, or raw parsing + manual typed status filter + conversion of accepted records | One `MixedLarge` scan at 0%, 1%, 10%, 50%, or 100% selection. Simple and typed variants. Both observe every field of accepted records. |
| `representative::lifecycle::*::construct` | Construct one record from `&str` | One record, with output destruction deferred by Divan. Referential construction includes copying the input into its owner. |
| `representative::lifecycle::*::clone` | Clone a prepared record | One clone; input construction and destruction of source/clone are outside timing. Referential cloning currently reparses. |
| `representative::lifecycle::*::destroy` | Destroy a prepared record | One record; construction is outside timing. Borrowed raw is a no-destructor control. |

Lifecycle variants cover borrowed and referential raw, borrowed/owned/referential simple, borrowed/referential typed, and all three Parquet representations where available. Lifecycle inputs cycle through 100 varied records per profile. This isolates local ownership operations; it does not measure fan-out, retained graphs, or file ingestion. Byte throughput is reported for construction and full/filter scans; clone and drop report record throughput because they do not scan a byte stream.

Full consumption black-boxes references to all 33 named source fields, including owned enum payloads and forwarded-address vectors, and then the complete record. The whole-record barrier also observes additional derived fields such as Parquet's `datetime`. It does not format or serialize values. The seven-field v2 workload remains unchanged and separate. Timings across these observers are different workloads, not evidence of a parser speedup or regression.

Filtering uses `sc_status < 200 + rate`. A deterministic permutation yields exactly the named rate when the corpus size is a multiple of 100, including the default. With another size, it approximates that rate. Selected profiles can differ between rates; this is not a model of an independent production predicate. Rejected records in `raw_then_convert` receive structural and status validation only; eager parsing validates every converted field. The corpus is valid, so this is a performance comparison with different validation scope, not proof that unobserved fields are valid.

## Focused runs and independent repetitions

Always specify sample count and size for representative runs: `.cargo/config.toml` sets both to 1000, and runtime environment settings override benchmark attributes. For a corpus scan, use sample size 1 initially. For short lifecycle operations, increase the sample size after smoke checks. `--list` shows the exact filter names.

PowerShell 7:

```powershell
# Compile both targets for each independent feature configuration.
foreach ($feature in @('', 'jiff', 'time', 'chrono', 'parquet')) {
    $features = @('--no-default-features')
    if ($feature) { $features += @('--features', $feature) }
    cargo bench @features --bench brwv --bench brwu --no-run
}

# Smoke: opt-in workloads, small corpus, separate disposable output.
$env:CF_BENCH_RECORDS = '100'
./bin/benches.ps1 -OutputPath tmp/representative-smoke.md -BenchArgs @('--ignored', '--sample-count', '2', '--sample-size', '1')
Remove-Item Env:CF_BENCH_RECORDS

# Retained measurement sessions in a chosen order; use a new basename each time.
./bin/benches.ps1 -OutputPath benchmarks/runs/full-20260909.md -Repetitions 5 -ConfigurationOrder chrono,time,jiff,no-features,parquet -BenchArgs @('representative::full_record', '--ignored', '--sample-count', '50', '--sample-size', '1')

# Allocations: separate instrumented run, no inferred timing comparison.
./bin/benches.ps1 -OutputPath benchmarks/runs/alloc-20260909.md -ConfigurationOrder jiff -AllocationProfile -BenchArgs @('representative::lifecycle', '--ignored', '--sample-count', '1000', '--sample-size', '1')
```

POSIX shell (Linux, WSL, macOS):

```sh
for feature in no-features jiff time chrono parquet; do
    if [ "$feature" = no-features ]; then
        cargo bench --no-default-features --bench brwv --bench brwu --no-run
    else
        cargo bench --no-default-features --features "$feature" --bench brwv --bench brwu --no-run
    fi
done

CF_BENCH_RECORDS=100 ./bin/benches.sh --output tmp/representative-smoke.md -- --ignored --sample-count 2 --sample-size 1
./bin/benches.sh --output benchmarks/runs/full-20260909.md --repetitions 5 --config-order chrono,time,jiff,no-features,parquet -- representative::full_record --ignored --sample-count 50 --sample-size 1
./bin/benches.sh --output benchmarks/runs/alloc-20260909.md --config-order jiff --alloc -- representative::lifecycle --ignored --sample-count 1000 --sample-size 1
```

Repetitions create `name.run-001.md`, `name.run-002.md`, and so on. Existing repetition paths cause failure before benchmarks start. Each complete repetition is published separately; if a later run fails, earlier completed runs remain. Single-report replacement still occurs only after every requested target/configuration succeeds. Custom arguments, profiling, repetitions, and sample/corpus environment overrides require an explicit output path so smoke runs cannot implicitly replace platform reports.

The order can be any nonempty, duplicate-free subset of the five configurations. It is recorded with the repetition index, timestamp, Git worktree state, allocator mode, arguments, and relevant environment overrides. Reverse or rotate the order under another output basename to check drift; ordering is never randomized silently. No affinity or power setting is changed. Precompile before a measurement session and avoid competing builds. Independent reports expose run-to-run variation; neither repetitions nor Divan's within-run medians alone establish statistical significance.

To compare original v2 data, use the existing names, for example `cargo bench --no-default-features --features jiff --bench brwv -- 'Sample File \(no comments\)' --sample-count 50 --sample-size 1000`. This selects only the original six-record workload. Use matching features, build flags and revisions on each side.

## Allocation and memory interpretation

Use sample size 1 when interpreting peak live allocations. Divan divides its
peak-live value by sample size, even when one iteration frees its data before
the next iteration starts. Larger batches can therefore understate the peak of
one operation. Allocation-event totals still normalize across the batch. The
minimum and maximum columns follow timing samples, not independent allocation
extrema. Use the mean allocation columns across varied records and retain the
raw output. Process memory requires a separate measurement.

`--alloc` / `-AllocationProfile` adds only the `bench-alloc` feature alongside the requested backend. It installs Divan's built-in `AllocProfiler` in the benchmark executables using safe Rust; the library's allocator and unsafe-code policy do not change. Direct Cargo invocation can use `--features jiff,bench-alloc`. Both targets compile with each backend, with and without this instrumentation.

Divan reports allocation counts, allocated bytes, deallocations and the measured peak allocation footprint. Construction and clone results are already per record. Full/filter values are per scan: divide allocation counts and bytes by 100 or `CF_BENCH_RECORDS`, respectively. Throughput counters report input records and input bytes per second (excluding line separators). Filtering throughput counts every examined record, not just accepted records. The allocation profiler affects timing; compare instrumented runs with instrumented runs and use uninstrumented runs for latency comparisons. Freed bytes, net live bytes, cumulative allocated bytes, and operating-system resident memory answer different questions.

Measure process peak memory separately from timings, using the compiled executable rather than `cargo` so compiler memory is excluded. On Linux/WSL, use `/usr/bin/time -v <benchmark-executable> --bench representative::full_record::simple::MixedLarge --ignored --sample-count 30 --sample-size 1 --min-time 0.2`; on macOS use `/usr/bin/time -l` with the same arguments. On Windows, start that executable using `System.Diagnostics.Process` with `CreateNoWindow = true`, drain redirected output asynchronously, and sample `PeakWorkingSet64` while it runs. Repeat with `MixedHot` and a larger `CF_BENCH_RECORDS` in separate processes. Keep the executable, allocator, and sample settings fixed. Direct executable invocation needs `--bench`; Cargo supplies that flag when using `cargo bench`.

These process peaks include the generated corpus, runtime, Divan buffers, allocator retention and parsed records. They are not peak memory of the parser alone. Short-lived Windows processes may exit between samples. Report the sampling interval and unavailable platform measurements. None of these benchmarks measure file reads, decompression, Parquet encoding/writing, or production end-to-end throughput.
