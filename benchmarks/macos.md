# Benchmarks: macos

## Benchmark environment

- Platform: `macos`
- Run date: `2026-09-13 16:05:43 +0200`
- OS: `Darwin 25.6.0 arm64`
- CPU: `Apple M1 Pro`
- RAM: `32.0 GiB`
- Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`
- Cargo: `cargo 1.98.0 (797e8a9bc 2026-08-05)`
- Git commit: `8b6e410`
- Git worktree: `dirty`
- RUSTFLAGS: `-Ctarget-cpu=native`
- Configuration order: `no-features,jiff,time,chrono,parquet`
- Repetition: 1 / 1
- Benchmark arguments: ``
- Allocation profiling: `disabled`
- DIVAN_SAMPLE_COUNT: `unset; see .cargo/config.toml and workload defaults`
- DIVAN_SAMPLE_SIZE: `unset; see .cargo/config.toml and workload defaults`
- DIVAN_MIN_TIME: `unset; workload default`
- DIVAN_MAX_TIME: `unset; workload default`
- DIVAN_SKIP_EXT_TIME: `unset; workload default`
- DIVAN_BYTES_FORMAT: `unset; see .cargo/config.toml`
- CF_BENCH_RECORDS: `unset; workload default`
- CARGO_TARGET_DIR: `unset`

## Configuration: no-features

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       328.7 ns      │ 435.3 ns      │ 329.5 ns      │ 330.6 ns      │ 1000    │ 1000000
│  ├─ Line B                       314 ns        │ 413.3 ns      │ 315.5 ns      │ 321.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    639.8 ns      │ 758.7 ns      │ 640.9 ns      │ 643.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.952 µs      │ 2.145 µs      │ 1.959 µs      │ 1.969 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.958 µs      │ 2.231 µs      │ 1.967 µs      │ 1.976 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       381.1 ns      │ 477.7 ns      │ 381.6 ns      │ 383.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       365.2 ns      │ 417.6 ns      │ 365.5 ns      │ 367 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                    742.7 ns      │ 904.2 ns      │ 744.2 ns      │ 754.6 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.241 µs      │ 2.529 µs      │ 2.255 µs      │ 2.262 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.243 µs      │ 2.486 µs      │ 2.262 µs      │ 2.269 µs      │ 1000    │ 1000000
╰─ representative                                │               │               │               │         │
   ├─ filtering                                  │               │               │               │         │
   │  ╰─ simple                                  │               │               │               │         │
   │     ├─ eager                  (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert       (ignored)     │               │               │               │         │
   ├─ full_record                                │               │               │               │         │
   │  ├─ raw                       (ignored)     │               │               │               │         │
   │  ╰─ simple                    (ignored)     │               │               │               │         │
   ╰─ lifecycle                                  │               │               │               │         │
      ├─ borrowed_raw                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ borrowed_simple                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ owned_simple                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ referential_raw                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ╰─ referential_simple                      │               │               │               │         │
         ├─ clone                  (ignored)     │               │               │               │         │
         ├─ construct              (ignored)     │               │               │               │         │
         ╰─ destroy                (ignored)     │               │               │               │         │

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     325.8 ns      │ 374.5 ns      │ 326.3 ns      │ 327.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     318.3 ns      │ 424.4 ns      │ 318.8 ns      │ 324 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                  641.8 ns      │ 753.3 ns      │ 643.8 ns      │ 646.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.971 µs      │ 2.233 µs      │ 1.995 µs      │ 2.001 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     339.9 ns      │ 383.4 ns      │ 342 ns        │ 343 ns        │ 1000    │ 1000000
│  ├─ Line B                     332.5 ns      │ 382.1 ns      │ 333.1 ns      │ 334.7 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  668 ns        │ 768.1 ns      │ 676.5 ns      │ 681.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.064 µs      │ 2.279 µs      │ 2.08 µs       │ 2.087 µs      │ 1000    │ 1000000
╰─ representative                              │               │               │               │         │
   ├─ filtering                                │               │               │               │         │
   │  ╰─ simple                                │               │               │               │         │
   │     ├─ eager                (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert     (ignored)     │               │               │               │         │
   ├─ full_record                              │               │               │               │         │
   │  ├─ raw                     (ignored)     │               │               │               │         │
   │  ╰─ simple                  (ignored)     │               │               │               │         │
   ╰─ lifecycle                                │               │               │               │         │
      ├─ borrowed_raw                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ borrowed_simple                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ owned_simple                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ referential_raw                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ╰─ referential_simple                    │               │               │               │         │
         ├─ clone                (ignored)     │               │               │               │         │
         ├─ construct            (ignored)     │               │               │               │         │
         ╰─ destroy              (ignored)     │               │               │               │         │

```

## Configuration: jiff

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       329.2 ns      │ 384.4 ns      │ 329.8 ns      │ 331 ns        │ 1000    │ 1000000
│  ├─ Line B                       313.5 ns      │ 379.2 ns      │ 314.8 ns      │ 315.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    639.4 ns      │ 736 ns        │ 640.6 ns      │ 643 ns        │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.948 µs      │ 2.143 µs      │ 1.957 µs      │ 1.966 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.956 µs      │ 2.192 µs      │ 1.967 µs      │ 1.973 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       355 ns        │ 464 ns        │ 355.5 ns      │ 361.5 ns      │ 1000    │ 1000000
│  ├─ Line B                       337.9 ns      │ 437 ns        │ 338.3 ns      │ 339.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    690 ns        │ 783.2 ns      │ 693 ns        │ 695.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.127 µs      │ 3.166 µs      │ 2.139 µs      │ 2.145 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.13 µs       │ 2.42 µs       │ 2.147 µs      │ 2.156 µs      │ 1000    │ 1000000
├─ 02 ValidatedTypedLogline                      │               │               │               │         │
│  ├─ Line A                       356.3 ns      │ 442.6 ns      │ 356.8 ns      │ 358.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       340 ns        │ 420 ns        │ 340.4 ns      │ 344.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    694.3 ns      │ 794.9 ns      │ 696.2 ns      │ 698.7 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.135 µs      │ 2.337 µs      │ 2.149 µs      │ 2.157 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.139 µs      │ 2.364 µs      │ 2.154 µs      │ 2.16 µs       │ 1000    │ 1000000
╰─ representative                                │               │               │               │         │
   ├─ filtering                                  │               │               │               │         │
   │  ├─ simple                                  │               │               │               │         │
   │  │  ├─ eager                  (ignored)     │               │               │               │         │
   │  │  ╰─ raw_then_convert       (ignored)     │               │               │               │         │
   │  ╰─ typed                                   │               │               │               │         │
   │     ├─ eager                  (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert       (ignored)     │               │               │               │         │
   ├─ full_record                                │               │               │               │         │
   │  ├─ raw                       (ignored)     │               │               │               │         │
   │  ├─ simple                    (ignored)     │               │               │               │         │
   │  ╰─ typed                     (ignored)     │               │               │               │         │
   ╰─ lifecycle                                  │               │               │               │         │
      ├─ borrowed_raw                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ borrowed_simple                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ borrowed_typed                          │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ owned_simple                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ referential_raw                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ referential_simple                      │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ╰─ referential_typed                       │               │               │               │         │
         ├─ clone                  (ignored)     │               │               │               │         │
         ├─ construct              (ignored)     │               │               │               │         │
         ╰─ destroy                (ignored)     │               │               │               │         │

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     341.5 ns      │ 394 ns        │ 344.4 ns      │ 344.8 ns      │ 1000    │ 1000000
│  ├─ Line B                     335.1 ns      │ 384.1 ns      │ 338.2 ns      │ 339 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                  668.2 ns      │ 752.5 ns      │ 669.1 ns      │ 674.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.973 µs      │ 2.249 µs      │ 1.991 µs      │ 1.998 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     359.2 ns      │ 410.7 ns      │ 359.9 ns      │ 361.2 ns      │ 1000    │ 1000000
│  ├─ Line B                     351.4 ns      │ 448.1 ns      │ 352 ns        │ 353 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                  688.4 ns      │ 793.6 ns      │ 690.8 ns      │ 692.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.062 µs      │ 3.554 µs      │ 2.082 µs      │ 2.092 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
│  ├─ Line A                     361.2 ns      │ 442 ns        │ 362.9 ns      │ 369.1 ns      │ 1000    │ 1000000
│  ├─ Line B                     353.2 ns      │ 403.6 ns      │ 354.4 ns      │ 355.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  692 ns        │ 1.151 µs      │ 693.9 ns      │ 699.3 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.072 µs      │ 2.301 µs      │ 2.081 µs      │ 2.09 µs       │ 1000    │ 1000000
╰─ representative                              │               │               │               │         │
   ├─ filtering                                │               │               │               │         │
   │  ├─ simple                                │               │               │               │         │
   │  │  ├─ eager                (ignored)     │               │               │               │         │
   │  │  ╰─ raw_then_convert     (ignored)     │               │               │               │         │
   │  ╰─ typed                                 │               │               │               │         │
   │     ├─ eager                (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert     (ignored)     │               │               │               │         │
   ├─ full_record                              │               │               │               │         │
   │  ├─ raw                     (ignored)     │               │               │               │         │
   │  ├─ simple                  (ignored)     │               │               │               │         │
   │  ╰─ typed                   (ignored)     │               │               │               │         │
   ╰─ lifecycle                                │               │               │               │         │
      ├─ borrowed_raw                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ borrowed_simple                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ borrowed_typed                        │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ owned_simple                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ referential_raw                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ referential_simple                    │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ╰─ referential_typed                     │               │               │               │         │
         ├─ clone                (ignored)     │               │               │               │         │
         ├─ construct            (ignored)     │               │               │               │         │
         ╰─ destroy              (ignored)     │               │               │               │         │

```

## Configuration: time

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       328.6 ns      │ 414 ns        │ 329.3 ns      │ 335.4 ns      │ 1000    │ 1000000
│  ├─ Line B                       313.3 ns      │ 366.6 ns      │ 313.6 ns      │ 314.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    639.1 ns      │ 769.6 ns      │ 639.9 ns      │ 642 ns        │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.952 µs      │ 2.209 µs      │ 1.963 µs      │ 1.97 µs       │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.957 µs      │ 2.206 µs      │ 1.964 µs      │ 1.973 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       354.5 ns      │ 434.6 ns      │ 355 ns        │ 356.1 ns      │ 1000    │ 1000000
│  ├─ Line B                       337.1 ns      │ 460.4 ns      │ 337.5 ns      │ 338.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    688.5 ns      │ 751 ns        │ 691.8 ns      │ 693.9 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.118 µs      │ 2.407 µs      │ 2.136 µs      │ 2.144 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.125 µs      │ 4.841 µs      │ 2.142 µs      │ 2.158 µs      │ 1000    │ 1000000
├─ 02 ValidatedTypedLogline                      │               │               │               │         │
│  ├─ Line A                       355.6 ns      │ 407 ns        │ 356.1 ns      │ 357.3 ns      │ 1000    │ 1000000
│  ├─ Line B                       340.1 ns      │ 418.6 ns      │ 340.4 ns      │ 341.7 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    693.7 ns      │ 913.4 ns      │ 695.4 ns      │ 698.6 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.133 µs      │ 2.386 µs      │ 2.153 µs      │ 2.16 µs       │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.139 µs      │ 2.397 µs      │ 2.153 µs      │ 2.161 µs      │ 1000    │ 1000000
╰─ representative                                │               │               │               │         │
   ├─ filtering                                  │               │               │               │         │
   │  ├─ simple                                  │               │               │               │         │
   │  │  ├─ eager                  (ignored)     │               │               │               │         │
   │  │  ╰─ raw_then_convert       (ignored)     │               │               │               │         │
   │  ╰─ typed                                   │               │               │               │         │
   │     ├─ eager                  (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert       (ignored)     │               │               │               │         │
   ├─ full_record                                │               │               │               │         │
   │  ├─ raw                       (ignored)     │               │               │               │         │
   │  ├─ simple                    (ignored)     │               │               │               │         │
   │  ╰─ typed                     (ignored)     │               │               │               │         │
   ╰─ lifecycle                                  │               │               │               │         │
      ├─ borrowed_raw                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ borrowed_simple                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ borrowed_typed                          │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ owned_simple                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ referential_raw                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ referential_simple                      │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ╰─ referential_typed                       │               │               │               │         │
         ├─ clone                  (ignored)     │               │               │               │         │
         ├─ construct              (ignored)     │               │               │               │         │
         ╰─ destroy                (ignored)     │               │               │               │         │

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     341.7 ns      │ 457 ns        │ 345.1 ns      │ 346.4 ns      │ 1000    │ 1000000
│  ├─ Line B                     335 ns        │ 410.5 ns      │ 337.8 ns      │ 338.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  668 ns        │ 780.9 ns      │ 668.9 ns      │ 671.3 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.972 µs      │ 2.834 µs      │ 1.991 µs      │ 2.005 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     358.8 ns      │ 677.2 ns      │ 359.5 ns      │ 361 ns        │ 1000    │ 1000000
│  ├─ Line B                     351.1 ns      │ 448.6 ns      │ 352 ns        │ 359.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  686.7 ns      │ 795.4 ns      │ 689.1 ns      │ 690.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.06 µs       │ 2.243 µs      │ 2.078 µs      │ 2.081 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
│  ├─ Line A                     360.8 ns      │ 416.5 ns      │ 361.3 ns      │ 362.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     353 ns        │ 435.5 ns      │ 353.4 ns      │ 354.7 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  690 ns        │ 808.6 ns      │ 692.2 ns      │ 698.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.069 µs      │ 2.273 µs      │ 2.081 µs      │ 2.088 µs      │ 1000    │ 1000000
╰─ representative                              │               │               │               │         │
   ├─ filtering                                │               │               │               │         │
   │  ├─ simple                                │               │               │               │         │
   │  │  ├─ eager                (ignored)     │               │               │               │         │
   │  │  ╰─ raw_then_convert     (ignored)     │               │               │               │         │
   │  ╰─ typed                                 │               │               │               │         │
   │     ├─ eager                (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert     (ignored)     │               │               │               │         │
   ├─ full_record                              │               │               │               │         │
   │  ├─ raw                     (ignored)     │               │               │               │         │
   │  ├─ simple                  (ignored)     │               │               │               │         │
   │  ╰─ typed                   (ignored)     │               │               │               │         │
   ╰─ lifecycle                                │               │               │               │         │
      ├─ borrowed_raw                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ borrowed_simple                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ borrowed_typed                        │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ owned_simple                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ referential_raw                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ referential_simple                    │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ╰─ referential_typed                     │               │               │               │         │
         ├─ clone                (ignored)     │               │               │               │         │
         ├─ construct            (ignored)     │               │               │               │         │
         ╰─ destroy              (ignored)     │               │               │               │         │

```

## Configuration: chrono

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       328.7 ns      │ 370.2 ns      │ 329.3 ns      │ 330.5 ns      │ 1000    │ 1000000
│  ├─ Line B                       313.3 ns      │ 387 ns        │ 313.5 ns      │ 314.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    639.1 ns      │ 746.6 ns      │ 640 ns        │ 648.6 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.951 µs      │ 2.147 µs      │ 1.959 µs      │ 1.967 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.958 µs      │ 2.185 µs      │ 1.97 µs       │ 1.975 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       355.5 ns      │ 412.6 ns      │ 356 ns        │ 357.5 ns      │ 1000    │ 1000000
│  ├─ Line B                       337.2 ns      │ 395.9 ns      │ 337.5 ns      │ 338.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    689.5 ns      │ 769.1 ns      │ 692.7 ns      │ 694.8 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.115 µs      │ 2.367 µs      │ 2.142 µs      │ 2.148 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.121 µs      │ 2.383 µs      │ 2.144 µs      │ 2.152 µs      │ 1000    │ 1000000
├─ 02 ValidatedTypedLogline                      │               │               │               │         │
│  ├─ Line A                       357.6 ns      │ 456.7 ns      │ 358.1 ns      │ 362.9 ns      │ 1000    │ 1000000
│  ├─ Line B                       339.5 ns      │ 410.5 ns      │ 339.8 ns      │ 340.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    694.9 ns      │ 767.8 ns      │ 696 ns        │ 697.9 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.138 µs      │ 2.599 µs      │ 2.15 µs       │ 2.157 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.147 µs      │ 2.29 µs       │ 2.16 µs       │ 2.163 µs      │ 1000    │ 1000000
╰─ representative                                │               │               │               │         │
   ├─ filtering                                  │               │               │               │         │
   │  ├─ simple                                  │               │               │               │         │
   │  │  ├─ eager                  (ignored)     │               │               │               │         │
   │  │  ╰─ raw_then_convert       (ignored)     │               │               │               │         │
   │  ╰─ typed                                   │               │               │               │         │
   │     ├─ eager                  (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert       (ignored)     │               │               │               │         │
   ├─ full_record                                │               │               │               │         │
   │  ├─ raw                       (ignored)     │               │               │               │         │
   │  ├─ simple                    (ignored)     │               │               │               │         │
   │  ╰─ typed                     (ignored)     │               │               │               │         │
   ╰─ lifecycle                                  │               │               │               │         │
      ├─ borrowed_raw                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ borrowed_simple                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ borrowed_typed                          │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ owned_simple                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ referential_raw                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ referential_simple                      │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ╰─ referential_typed                       │               │               │               │         │
         ├─ clone                  (ignored)     │               │               │               │         │
         ├─ construct              (ignored)     │               │               │               │         │
         ╰─ destroy                (ignored)     │               │               │               │         │

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     342.7 ns      │ 391.7 ns      │ 345.5 ns      │ 346.5 ns      │ 1000    │ 1000000
│  ├─ Line B                     335.3 ns      │ 450.7 ns      │ 338.5 ns      │ 339.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  668.7 ns      │ 788.6 ns      │ 669.6 ns      │ 676.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.97 µs       │ 2.209 µs      │ 1.998 µs      │ 2.005 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     359.3 ns      │ 415 ns        │ 359.9 ns      │ 361 ns        │ 1000    │ 1000000
│  ├─ Line B                     352.1 ns      │ 402.3 ns      │ 352.6 ns      │ 353.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  687.2 ns      │ 788.5 ns      │ 688.3 ns      │ 690.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.067 µs      │ 2.357 µs      │ 2.086 µs      │ 2.096 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
│  ├─ Line A                     360.5 ns      │ 469.3 ns      │ 361 ns        │ 364.1 ns      │ 1000    │ 1000000
│  ├─ Line B                     352.9 ns      │ 450.2 ns      │ 353.3 ns      │ 354.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  690.8 ns      │ 794.5 ns      │ 695.2 ns      │ 698.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.082 µs      │ 2.22 µs       │ 2.091 µs      │ 2.095 µs      │ 1000    │ 1000000
╰─ representative                              │               │               │               │         │
   ├─ filtering                                │               │               │               │         │
   │  ├─ simple                                │               │               │               │         │
   │  │  ├─ eager                (ignored)     │               │               │               │         │
   │  │  ╰─ raw_then_convert     (ignored)     │               │               │               │         │
   │  ╰─ typed                                 │               │               │               │         │
   │     ├─ eager                (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert     (ignored)     │               │               │               │         │
   ├─ full_record                              │               │               │               │         │
   │  ├─ raw                     (ignored)     │               │               │               │         │
   │  ├─ simple                  (ignored)     │               │               │               │         │
   │  ╰─ typed                   (ignored)     │               │               │               │         │
   ╰─ lifecycle                                │               │               │               │         │
      ├─ borrowed_raw                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ borrowed_simple                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ borrowed_typed                        │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ owned_simple                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ referential_raw                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ referential_simple                    │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ╰─ referential_typed                     │               │               │               │         │
         ├─ clone                (ignored)     │               │               │               │         │
         ├─ construct            (ignored)     │               │               │               │         │
         ╰─ destroy              (ignored)     │               │               │               │         │

```

## Configuration: parquet

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       328.5 ns      │ 423.7 ns      │ 329.6 ns      │ 330.8 ns      │ 1000    │ 1000000
│  ├─ Line B                       314.2 ns      │ 439 ns        │ 314.9 ns      │ 317.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    639.7 ns      │ 1.387 µs      │ 640.7 ns      │ 649.3 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.951 µs      │ 2.192 µs      │ 1.961 µs      │ 1.97 µs       │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.956 µs      │ 2.169 µs      │ 1.964 µs      │ 1.973 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       354.5 ns      │ 438.9 ns      │ 354.8 ns      │ 355.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       339.1 ns      │ 390.1 ns      │ 339.4 ns      │ 340.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    690.2 ns      │ 765.7 ns      │ 691.2 ns      │ 693.9 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.115 µs      │ 2.402 µs      │ 2.13 µs       │ 2.142 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.119 µs      │ 2.378 µs      │ 2.137 µs      │ 2.144 µs      │ 1000    │ 1000000
├─ 03 ValidatedParquetLogline                    │               │               │               │         │
│  ├─ Line A                       346.5 ns      │ 423.9 ns      │ 346.8 ns      │ 348.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       330.4 ns      │ 376.1 ns      │ 330.7 ns      │ 332 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                    674.2 ns      │ 850.9 ns      │ 674.8 ns      │ 681.2 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.067 µs      │ 2.348 µs      │ 2.078 µs      │ 2.085 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.07 µs       │ 2.331 µs      │ 2.083 µs      │ 2.089 µs      │ 1000    │ 1000000
╰─ representative                                │               │               │               │         │
   ├─ filtering                                  │               │               │               │         │
   │  ╰─ simple                                  │               │               │               │         │
   │     ├─ eager                  (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert       (ignored)     │               │               │               │         │
   ├─ full_record                                │               │               │               │         │
   │  ├─ parquet                   (ignored)     │               │               │               │         │
   │  ├─ raw                       (ignored)     │               │               │               │         │
   │  ╰─ simple                    (ignored)     │               │               │               │         │
   ╰─ lifecycle                                  │               │               │               │         │
      ├─ borrowed_parquet                        │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ borrowed_raw                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ borrowed_simple                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ owned_parquet                           │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ owned_simple                            │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ referential_parquet                     │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ├─ referential_raw                         │               │               │               │         │
      │  ├─ clone                  (ignored)     │               │               │               │         │
      │  ├─ construct              (ignored)     │               │               │               │         │
      │  ╰─ destroy                (ignored)     │               │               │               │         │
      ╰─ referential_simple                      │               │               │               │         │
         ├─ clone                  (ignored)     │               │               │               │         │
         ├─ construct              (ignored)     │               │               │               │         │
         ╰─ destroy                (ignored)     │               │               │               │         │

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Representative v1: synthetic corpus, 100-record hot batches; 32700-record MixedLarge batches.
Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record.
Allocation profiler: disabled. No file I/O, decompression, or encoding.
Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     376.8 ns      │ 455.8 ns      │ 377.4 ns      │ 382.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     318.1 ns      │ 440.8 ns      │ 318.4 ns      │ 319.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  693.6 ns      │ 791.5 ns      │ 695.8 ns      │ 697.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.026 µs      │ 3 µs          │ 2.051 µs      │ 2.062 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     390.8 ns      │ 429 ns        │ 391.7 ns      │ 392.9 ns      │ 1000    │ 1000000
│  ├─ Line B                     333 ns        │ 388.1 ns      │ 334.3 ns      │ 336.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  721.7 ns      │ 837.2 ns      │ 724.1 ns      │ 730.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.117 µs      │ 2.343 µs      │ 2.136 µs      │ 2.143 µs      │ 1000    │ 1000000
├─ 03 UnvalidatedParquetLogline                │               │               │               │         │
│  ├─ Line A                     379 ns        │ 428.5 ns      │ 386.3 ns      │ 387.2 ns      │ 1000    │ 1000000
│  ├─ Line B                     324.9 ns      │ 439.1 ns      │ 335.2 ns      │ 335.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  700.9 ns      │ 833.3 ns      │ 710 ns        │ 715.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.048 µs      │ 2.338 µs      │ 2.069 µs      │ 2.078 µs      │ 1000    │ 1000000
╰─ representative                              │               │               │               │         │
   ├─ filtering                                │               │               │               │         │
   │  ╰─ simple                                │               │               │               │         │
   │     ├─ eager                (ignored)     │               │               │               │         │
   │     ╰─ raw_then_convert     (ignored)     │               │               │               │         │
   ├─ full_record                              │               │               │               │         │
   │  ├─ parquet                 (ignored)     │               │               │               │         │
   │  ├─ raw                     (ignored)     │               │               │               │         │
   │  ╰─ simple                  (ignored)     │               │               │               │         │
   ╰─ lifecycle                                │               │               │               │         │
      ├─ borrowed_parquet                      │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ borrowed_raw                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ borrowed_simple                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ owned_parquet                         │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ owned_simple                          │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ referential_parquet                   │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ├─ referential_raw                       │               │               │               │         │
      │  ├─ clone                (ignored)     │               │               │               │         │
      │  ├─ construct            (ignored)     │               │               │               │         │
      │  ╰─ destroy              (ignored)     │               │               │               │         │
      ╰─ referential_simple                    │               │               │               │         │
         ├─ clone                (ignored)     │               │               │               │         │
         ├─ construct            (ignored)     │               │               │               │         │
         ╰─ destroy              (ignored)     │               │               │               │         │

```
