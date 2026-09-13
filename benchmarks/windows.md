# Benchmarks: windows

## Benchmark environment

- Platform: `windows`
- Run date: `2026-09-13 15:53:00 +02:00`
- OS: `Microsoft Windows 11 Pro`
- CPU: `AMD Ryzen 9 7950X3D 16-Core Processor`
- RAM: `63.7 GiB`
- Toolchain: `rustc 1.98.1 (48a229cea 2026-09-01)`
- Cargo: `cargo 1.98.1 (797e8a9bc 2026-08-05)`
- Git commit: `8b6e410`
- Git worktree: `dirty`
- RUSTFLAGS: `-Ctarget-cpu=native`
- Configuration order: `no-features,jiff,time,chrono,parquet`
- Repetition: 1 / 1
- Benchmark arguments: ``
- Allocation profiling: `disabled`
- DIVAN_SAMPLE_COUNT: `unset; see .cargo/config.toml and workload defaults`
- DIVAN_SAMPLE_SIZE: `unset; see .cargo/config.toml and workload defaults`
- DIVAN_MIN_TIME: `unset; see .cargo/config.toml and workload defaults`
- DIVAN_MAX_TIME: `unset; see .cargo/config.toml and workload defaults`
- DIVAN_SKIP_EXT_TIME: `unset; see .cargo/config.toml and workload defaults`
- DIVAN_BYTES_FORMAT: `unset; see .cargo/config.toml and workload defaults`
- CF_BENCH_RECORDS: `unset; see .cargo/config.toml and workload defaults`
- CARGO_TARGET_DIR: `T:\target`

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
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
Timer precision: 100 ns
│  ├─ Line A                       133 ns        │ 226.9 ns      │ 135.5 ns      │ 143 ns        │ 1000    │ 1000000
│  ├─ Line B                       139.8 ns      │ 169.1 ns      │ 142.4 ns      │ 142.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    264.1 ns      │ 347.3 ns      │ 269.8 ns      │ 271 ns        │ 1000    │ 1000000
│  ├─ Sample File (no comments)    770.3 ns      │ 947.1 ns      │ 785.8 ns      │ 790.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  780.2 ns      │ 965.3 ns      │ 802.3 ns      │ 805.5 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       179.9 ns      │ 1.982 µs      │ 183.9 ns      │ 188.1 ns      │ 1000    │ 1000000
│  ├─ Line B                       193.1 ns      │ 284.4 ns      │ 196.9 ns      │ 198.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    369.4 ns      │ 469.9 ns      │ 377.8 ns      │ 380.3 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.063 µs      │ 1.426 µs      │ 1.095 µs      │ 1.104 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.061 µs      │ 1.271 µs      │ 1.099 µs      │ 1.117 µs      │ 1000    │ 1000000
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
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     125.8 ns      │ 245.6 ns      │ 128.5 ns      │ 129 ns        │ 1000    │ 1000000
│  ├─ Line B                     123.9 ns      │ 157.4 ns      │ 125.7 ns      │ 126.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  242 ns        │ 284 ns        │ 246.3 ns      │ 246.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  717.2 ns      │ 825.6 ns      │ 737.4 ns      │ 738.1 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     185.5 ns      │ 256 ns        │ 190.8 ns      │ 191.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     172.5 ns      │ 270.8 ns      │ 176.2 ns      │ 176.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  354.7 ns      │ 394.7 ns      │ 363.3 ns      │ 364.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.025 µs      │ 1.583 µs      │ 1.061 µs      │ 1.061 µs      │ 1000    │ 1000000
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
Timer precision: 100 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       131.6 ns      │ 264.3 ns      │ 135.7 ns      │ 136.4 ns      │ 1000    │ 1000000
│  ├─ Line B                       132.7 ns      │ 165.5 ns      │ 135 ns        │ 135.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    259 ns        │ 386.3 ns      │ 264.7 ns      │ 265.6 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    770.5 ns      │ 996.3 ns      │ 787.2 ns      │ 790.3 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  763.4 ns      │ 943.7 ns      │ 784.1 ns      │ 787.5 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       173.6 ns      │ 2.124 µs      │ 187.7 ns      │ 188.8 ns      │ 1000    │ 1000000
│  ├─ Line B                       177.4 ns      │ 227.7 ns      │ 184.7 ns      │ 185 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                    347.6 ns      │ 397.1 ns      │ 367 ns        │ 367.3 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.042 µs      │ 1.506 µs      │ 1.079 µs      │ 1.08 µs       │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.043 µs      │ 1.558 µs      │ 1.084 µs      │ 1.085 µs      │ 1000    │ 1000000
├─ 02 ValidatedTypedLogline                      │               │               │               │         │
│  ├─ Line A                       179.6 ns      │ 217.9 ns      │ 188.9 ns      │ 189.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       183.9 ns      │ 225 ns        │ 191.8 ns      │ 192.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    359.7 ns      │ 433.4 ns      │ 379.5 ns      │ 379.2 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.085 µs      │ 1.434 µs      │ 1.118 µs      │ 1.121 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.087 µs      │ 1.837 µs      │ 1.124 µs      │ 1.131 µs      │ 1000    │ 1000000
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
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     124.4 ns      │ 396.2 ns      │ 127.8 ns      │ 128.4 ns      │ 1000    │ 1000000
│  ├─ Line B                     124.9 ns      │ 159.9 ns      │ 127.2 ns      │ 127.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  243.8 ns      │ 350.3 ns      │ 249.5 ns      │ 250.3 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  727.2 ns      │ 794.1 ns      │ 742.9 ns      │ 743.5 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     172.1 ns      │ 2.023 µs      │ 174.8 ns      │ 177.1 ns      │ 1000    │ 1000000
│  ├─ Line B                     170.3 ns      │ 214.6 ns      │ 173.9 ns      │ 174.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  333.4 ns      │ 419.5 ns      │ 341.1 ns      │ 341.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.002 µs      │ 1.454 µs      │ 1.039 µs      │ 1.039 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
│  ├─ Line A                     174.2 ns      │ 341.9 ns      │ 181.8 ns      │ 227.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     175.6 ns      │ 332.1 ns      │ 294.1 ns      │ 281.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  343.2 ns      │ 630.9 ns      │ 589.3 ns      │ 559.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.037 µs      │ 1.787 µs      │ 1.07 µs       │ 1.202 µs      │ 1000    │ 1000000
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
Timer precision: 100 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       140.7 ns      │ 246.5 ns      │ 144.1 ns      │ 144.5 ns      │ 1000    │ 1000000
│  ├─ Line B                       138.5 ns      │ 156.2 ns      │ 142.4 ns      │ 142.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    275.5 ns      │ 321.9 ns      │ 279.9 ns      │ 280.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    810.3 ns      │ 919.9 ns      │ 828.5 ns      │ 829.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  824.7 ns      │ 1.183 µs      │ 843.1 ns      │ 845.6 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       176.8 ns      │ 1.771 µs      │ 180.5 ns      │ 183.4 ns      │ 1000    │ 1000000
│  ├─ Line B                       187.8 ns      │ 223.7 ns      │ 192.3 ns      │ 192.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    362.4 ns      │ 562.7 ns      │ 377.5 ns      │ 377.8 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.071 µs      │ 1.972 µs      │ 1.102 µs      │ 1.136 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.086 µs      │ 1.816 µs      │ 1.123 µs      │ 1.136 µs      │ 1000    │ 1000000
├─ 02 ValidatedTypedLogline                      │               │               │               │         │
│  ├─ Line A                       183.6 ns      │ 405.9 ns      │ 192.6 ns      │ 231.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       186.1 ns      │ 375.2 ns      │ 317.6 ns      │ 288.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    375.3 ns      │ 883.5 ns      │ 394.9 ns      │ 472.8 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.123 µs      │ 2.098 µs      │ 1.155 µs      │ 1.387 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.13 µs       │ 2.883 µs      │ 1.923 µs      │ 1.732 µs      │ 1000    │ 1000000
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
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     124 ns        │ 258.6 ns      │ 126.5 ns      │ 127.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     124.6 ns      │ 159.8 ns      │ 127.5 ns      │ 127.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  243.5 ns      │ 358.9 ns      │ 249 ns        │ 249.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  720.4 ns      │ 826.6 ns      │ 737 ns        │ 738.4 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     169.8 ns      │ 1.793 µs      │ 174.1 ns      │ 176 ns        │ 1000    │ 1000000
│  ├─ Line B                     170.7 ns      │ 283.2 ns      │ 173.8 ns      │ 174.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  334.8 ns      │ 567.6 ns      │ 347.7 ns      │ 349 ns        │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.01 µs       │ 2.335 µs      │ 1.059 µs      │ 1.088 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
│  ├─ Line A                     182.5 ns      │ 478.7 ns      │ 302.3 ns      │ 272.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     184.4 ns      │ 485.6 ns      │ 194.8 ns      │ 215 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                  362.5 ns      │ 635.7 ns      │ 375.6 ns      │ 381.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.09 µs       │ 2.716 µs      │ 1.706 µs      │ 1.608 µs      │ 1000    │ 1000000
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
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
Timer precision: 100 ns
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       135.5 ns      │ 365.3 ns      │ 139.7 ns      │ 141.1 ns      │ 1000    │ 1000000
│  ├─ Line B                       131 ns        │ 216 ns        │ 135.2 ns      │ 136.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    262 ns        │ 367.4 ns      │ 269.1 ns      │ 270.6 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    773.6 ns      │ 917.7 ns      │ 790.8 ns      │ 792 ns        │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  772.1 ns      │ 1.122 µs      │ 792.6 ns      │ 794.7 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       179.4 ns      │ 1.945 µs      │ 184.4 ns      │ 186.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       176.5 ns      │ 215.3 ns      │ 181.4 ns      │ 181.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    357.2 ns      │ 578.2 ns      │ 369.6 ns      │ 370.9 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.079 µs      │ 2.739 µs      │ 1.791 µs      │ 1.753 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.074 µs      │ 2.663 µs      │ 1.774 µs      │ 1.546 µs      │ 1000    │ 1000000
├─ 02 ValidatedTypedLogline                      │               │               │               │         │
│  ├─ Line A                       196.8 ns      │ 525.3 ns      │ 332.5 ns      │ 297.9 ns      │ 1000    │ 1000000
│  ├─ Line B                       188.4 ns      │ 336.8 ns      │ 198.4 ns      │ 201.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    374.7 ns      │ 839.4 ns      │ 402.9 ns      │ 475.9 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.115 µs      │ 2.812 µs      │ 1.89 µs       │ 1.834 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.115 µs      │ 2.677 µs      │ 1.17 µs       │ 1.465 µs      │ 1000    │ 1000000
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
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     137.6 ns      │ 280.6 ns      │ 141.6 ns      │ 142.2 ns      │ 1000    │ 1000000
│  ├─ Line B                     134.3 ns      │ 204.1 ns      │ 136.9 ns      │ 137.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  261.1 ns      │ 324.7 ns      │ 267.8 ns      │ 268.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  770.1 ns      │ 1.021 µs      │ 790.2 ns      │ 792.8 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     171.7 ns      │ 1.604 µs      │ 175.8 ns      │ 178.2 ns      │ 1000    │ 1000000
│  ├─ Line B                     172 ns        │ 222.9 ns      │ 176.9 ns      │ 178.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  339.2 ns      │ 458.5 ns      │ 349.9 ns      │ 350.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.006 µs      │ 1.586 µs      │ 1.05 µs       │ 1.053 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
│  ├─ Line A                     175 ns        │ 407.5 ns      │ 179.6 ns      │ 185.9 ns      │ 1000    │ 1000000
│  ├─ Line B                     176.8 ns      │ 426.3 ns      │ 187 ns        │ 243.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  345.8 ns      │ 868.3 ns      │ 358 ns        │ 381.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.034 µs      │ 2.653 µs      │ 1.099 µs      │ 1.308 µs      │ 1000    │ 1000000
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
Timer precision: 100 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       128.4 ns      │ 261.9 ns      │ 131 ns        │ 131.6 ns      │ 1000    │ 1000000
│  ├─ Line B                       128.1 ns      │ 165.7 ns      │ 131.6 ns      │ 132.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    251.5 ns      │ 310.4 ns      │ 256.1 ns      │ 256.9 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    753.2 ns      │ 846.1 ns      │ 765.3 ns      │ 767.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  759 ns        │ 1.132 µs      │ 771.6 ns      │ 774 ns        │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       173.1 ns      │ 1.801 µs      │ 178.5 ns      │ 180.4 ns      │ 1000    │ 1000000
│  ├─ Line B                       177.3 ns      │ 202 ns        │ 181 ns        │ 181.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    347 ns        │ 405.1 ns      │ 361.1 ns      │ 361.2 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.034 µs      │ 1.958 µs      │ 1.085 µs      │ 1.159 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.049 µs      │ 2.143 µs      │ 1.089 µs      │ 1.125 µs      │ 1000    │ 1000000
├─ 03 ValidatedParquetLogline                    │               │               │               │         │
│  ├─ Line A                       160.8 ns      │ 399.1 ns      │ 168.1 ns      │ 194.5 ns      │ 1000    │ 1000000
│  ├─ Line B                       158.4 ns      │ 375.4 ns      │ 162.6 ns      │ 165.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    311.6 ns      │ 529 ns        │ 317 ns        │ 320.7 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    924.4 ns      │ 1.616 µs      │ 940.9 ns      │ 961 ns        │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  932.6 ns      │ 1.991 µs      │ 957.8 ns      │ 1.043 µs      │ 1000    │ 1000000
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
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     126.4 ns      │ 245.5 ns      │ 129.1 ns      │ 129.6 ns      │ 1000    │ 1000000
│  ├─ Line B                     125.6 ns      │ 171.3 ns      │ 127.6 ns      │ 128.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  246.1 ns      │ 327.7 ns      │ 251.4 ns      │ 252.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  714 ns        │ 1.036 µs      │ 727.1 ns      │ 729.8 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     167 ns        │ 257.7 ns      │ 170.1 ns      │ 170.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     167.5 ns      │ 200.8 ns      │ 170.6 ns      │ 171 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                  329.8 ns      │ 538.1 ns      │ 337.9 ns      │ 339.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  990.5 ns      │ 1.343 µs      │ 1.011 µs      │ 1.014 µs      │ 1000    │ 1000000
├─ 03 UnvalidatedParquetLogline                │               │               │               │         │
│  ├─ Line A                     150 ns        │ 1.443 µs      │ 154.4 ns      │ 157 ns        │ 1000    │ 1000000
│  ├─ Line B                     150.4 ns      │ 343.4 ns      │ 155.1 ns      │ 160.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  290.3 ns      │ 485.2 ns      │ 300.7 ns      │ 303.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  866 ns        │ 2.317 µs      │ 915.4 ns      │ 1.18 µs       │ 1000    │ 1000000
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
