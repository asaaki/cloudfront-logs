# Benchmarks: wsl

## Benchmark environment

- Platform: `wsl`
- Run date: `2026-09-13 15:54:24 +0200`
- OS: `Linux 6.18.33.2-microsoft-standard-WSL2 x86_64 GNU/Linux`
- CPU: `AMD Ryzen 9 7950X3D 16-Core Processor`
- RAM: `31.3 GiB`
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
Timer precision: 10 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       144.8 ns      │ 479.4 ns      │ 148.3 ns      │ 151.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       144.2 ns      │ 262 ns        │ 148.3 ns      │ 152.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    282.5 ns      │ 448.4 ns      │ 291.3 ns      │ 296.8 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    838.9 ns      │ 2.025 µs      │ 859.8 ns      │ 867 ns        │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  846.8 ns      │ 1.09 µs       │ 881.2 ns      │ 890.2 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       189 ns        │ 327.4 ns      │ 195.9 ns      │ 199 ns        │ 1000    │ 1000000
│  ├─ Line B                       186.9 ns      │ 319.4 ns      │ 194 ns        │ 197.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    371 ns        │ 628.4 ns      │ 387.3 ns      │ 393.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.138 µs      │ 1.872 µs      │ 1.182 µs      │ 1.194 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.148 µs      │ 1.657 µs      │ 1.188 µs      │ 1.2 µs        │ 1000    │ 1000000
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
Timer precision: 10 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     137.7 ns      │ 433.4 ns      │ 140.4 ns      │ 142.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     137 ns        │ 189.2 ns      │ 139.6 ns      │ 142.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  266.8 ns      │ 348.7 ns      │ 275.5 ns      │ 278.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  800.6 ns      │ 2.072 µs      │ 826.5 ns      │ 838.3 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     176.9 ns      │ 339.9 ns      │ 183.8 ns      │ 188.1 ns      │ 1000    │ 1000000
│  ├─ Line B                     177.9 ns      │ 271 ns        │ 184.4 ns      │ 187.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  355.4 ns      │ 612 ns        │ 366.2 ns      │ 371.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.094 µs      │ 1.509 µs      │ 1.131 µs      │ 1.138 µs      │ 1000    │ 1000000
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
Timer precision: 19 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       144.9 ns      │ 612.3 ns      │ 147.8 ns      │ 150.8 ns      │ 1000    │ 1000000
│  ├─ Line B                       144.2 ns      │ 243.2 ns      │ 148 ns        │ 151.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    284.2 ns      │ 389.8 ns      │ 292.6 ns      │ 294.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    841.1 ns      │ 1.102 µs      │ 864 ns        │ 866.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  845.3 ns      │ 1.213 µs      │ 872.1 ns      │ 878.3 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       186 ns        │ 1.939 µs      │ 192.8 ns      │ 199.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       188.4 ns      │ 302.7 ns      │ 193.7 ns      │ 196.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    372.3 ns      │ 626.7 ns      │ 382.8 ns      │ 386.7 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.116 µs      │ 4.829 µs      │ 1.154 µs      │ 1.161 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.118 µs      │ 1.386 µs      │ 1.168 µs      │ 1.172 µs      │ 1000    │ 1000000
├─ 02 ValidatedTypedLogline                      │               │               │               │         │
│  ├─ Line A                       192.4 ns      │ 305.4 ns      │ 196.9 ns      │ 199.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       195 ns        │ 279.2 ns      │ 199.4 ns      │ 201.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    382.1 ns      │ 636.6 ns      │ 393.8 ns      │ 397.3 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.16 µs       │ 1.865 µs      │ 1.209 µs      │ 1.22 µs       │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.156 µs      │ 1.744 µs      │ 1.219 µs      │ 1.232 µs      │ 1000    │ 1000000
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
Timer precision: 9 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     140.9 ns      │ 677.2 ns      │ 152.2 ns      │ 153.6 ns      │ 1000    │ 1000000
│  ├─ Line B                     134.2 ns      │ 259.8 ns      │ 138.2 ns      │ 140.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  269 ns        │ 411.9 ns      │ 292.6 ns      │ 296.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  794.8 ns      │ 1.034 µs      │ 824.2 ns      │ 835.8 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     206.8 ns      │ 497.6 ns      │ 215.1 ns      │ 217.9 ns      │ 1000    │ 1000000
│  ├─ Line B                     185.5 ns      │ 277.4 ns      │ 190.7 ns      │ 192.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  392.7 ns      │ 1.116 µs      │ 409.2 ns      │ 415.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.116 µs      │ 1.834 µs      │ 1.155 µs      │ 1.161 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
│  ├─ Line A                     212 ns        │ 1.681 µs      │ 219.1 ns      │ 222.6 ns      │ 1000    │ 1000000
│  ├─ Line B                     188.9 ns      │ 317.4 ns      │ 196.9 ns      │ 201.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  397.3 ns      │ 551.1 ns      │ 413.2 ns      │ 417.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.159 µs      │ 1.716 µs      │ 1.197 µs      │ 1.206 µs      │ 1000    │ 1000000
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
Timer precision: 10 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       146.2 ns      │ 615.7 ns      │ 149.1 ns      │ 152.3 ns      │ 1000    │ 1000000
│  ├─ Line B                       145.5 ns      │ 265.2 ns      │ 148 ns        │ 152.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    284.6 ns      │ 414.3 ns      │ 290.4 ns      │ 294.4 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    846.3 ns      │ 1.224 µs      │ 871.3 ns      │ 880.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  848.3 ns      │ 1.102 µs      │ 870.8 ns      │ 873.2 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       188 ns        │ 552.8 ns      │ 194.3 ns      │ 198 ns        │ 1000    │ 1000000
│  ├─ Line B                       191.1 ns      │ 323.3 ns      │ 197.2 ns      │ 202.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    373.4 ns      │ 557.3 ns      │ 386.8 ns      │ 392.7 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.133 µs      │ 1.638 µs      │ 1.169 µs      │ 1.176 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.144 µs      │ 1.901 µs      │ 1.184 µs      │ 1.193 µs      │ 1000    │ 1000000
├─ 02 ValidatedTypedLogline                      │               │               │               │         │
│  ├─ Line A                       196.9 ns      │ 350.1 ns      │ 203 ns        │ 208.4 ns      │ 1000    │ 1000000
│  ├─ Line B                       196.1 ns      │ 342.4 ns      │ 201.1 ns      │ 204.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    383.8 ns      │ 647.8 ns      │ 397 ns        │ 403.2 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.157 µs      │ 1.551 µs      │ 1.205 µs      │ 1.21 µs       │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.168 µs      │ 1.818 µs      │ 1.214 µs      │ 1.221 µs      │ 1000    │ 1000000
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
Timer precision: 10 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     136.4 ns      │ 633.7 ns      │ 138.8 ns      │ 141.2 ns      │ 1000    │ 1000000
│  ├─ Line B                     144 ns        │ 229.2 ns      │ 147.3 ns      │ 149.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  273.8 ns      │ 393.7 ns      │ 280.8 ns      │ 284 ns        │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  796.8 ns      │ 2.578 µs      │ 816.3 ns      │ 821.7 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     177 ns        │ 295 ns        │ 182.7 ns      │ 185.5 ns      │ 1000    │ 1000000
│  ├─ Line B                     177.7 ns      │ 261.9 ns      │ 182.9 ns      │ 186.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  353.9 ns      │ 583.1 ns      │ 365.4 ns      │ 370.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.079 µs      │ 1.642 µs      │ 1.122 µs      │ 1.124 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
│  ├─ Line A                     181 ns        │ 250.1 ns      │ 195 ns        │ 195.8 ns      │ 1000    │ 1000000
│  ├─ Line B                     181.5 ns      │ 244.1 ns      │ 189 ns        │ 191.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  362 ns        │ 579.6 ns      │ 376.4 ns      │ 382.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.13 µs       │ 1.539 µs      │ 1.167 µs      │ 1.178 µs      │ 1000    │ 1000000
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
Timer precision: 9 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       144.5 ns      │ 613.4 ns      │ 147.4 ns      │ 150 ns        │ 1000    │ 1000000
│  ├─ Line B                       144.2 ns      │ 198.6 ns      │ 147.8 ns      │ 150.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    281.1 ns      │ 412.1 ns      │ 289.9 ns      │ 295.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    842.6 ns      │ 2.454 µs      │ 863.2 ns      │ 868.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  838.9 ns      │ 1.094 µs      │ 870.7 ns      │ 877.6 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       188.4 ns      │ 1.987 µs      │ 192.7 ns      │ 197.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       188.9 ns      │ 302.2 ns      │ 194.2 ns      │ 197.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    370.3 ns      │ 591.7 ns      │ 382.5 ns      │ 385 ns        │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.14 µs       │ 1.786 µs      │ 1.186 µs      │ 1.192 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.16 µs       │ 1.758 µs      │ 1.207 µs      │ 1.211 µs      │ 1000    │ 1000000
├─ 02 ValidatedTypedLogline                      │               │               │               │         │
│  ├─ Line A                       192.6 ns      │ 312.9 ns      │ 197.8 ns      │ 201.1 ns      │ 1000    │ 1000000
│  ├─ Line B                       193.4 ns      │ 315.9 ns      │ 199 ns        │ 201.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    382.9 ns      │ 712 ns        │ 397.5 ns      │ 400.6 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.179 µs      │ 1.319 µs      │ 1.221 µs      │ 1.224 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.18 µs       │ 1.386 µs      │ 1.222 µs      │ 1.226 µs      │ 1000    │ 1000000
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
Timer precision: 9 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     145.5 ns      │ 657.7 ns      │ 148.4 ns      │ 150.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     137.6 ns      │ 2.508 µs      │ 140.2 ns      │ 144.7 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  277.2 ns      │ 391.2 ns      │ 282.4 ns      │ 285.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  801.5 ns      │ 1.203 µs      │ 823.6 ns      │ 840.6 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     179.7 ns      │ 251.5 ns      │ 183.4 ns      │ 185.8 ns      │ 1000    │ 1000000
│  ├─ Line B                     184.9 ns      │ 256.1 ns      │ 189.2 ns      │ 191.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  361.9 ns      │ 536.7 ns      │ 374.1 ns      │ 379.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.082 µs      │ 1.639 µs      │ 1.126 µs      │ 1.138 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
│  ├─ Line A                     183.7 ns      │ 1.935 µs      │ 190.3 ns      │ 195.2 ns      │ 1000    │ 1000000
│  ├─ Line B                     194.1 ns      │ 336.1 ns      │ 199.1 ns      │ 203.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  377 ns        │ 575.4 ns      │ 387.3 ns      │ 391.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.117 µs      │ 1.241 µs      │ 1.15 µs       │ 1.152 µs      │ 1000    │ 1000000
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
Timer precision: 10 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       145.5 ns      │ 602.3 ns      │ 148 ns        │ 152.6 ns      │ 1000    │ 1000000
│  ├─ Line B                       143.8 ns      │ 342.8 ns      │ 147.9 ns      │ 153.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    283 ns        │ 405.1 ns      │ 290.4 ns      │ 294.2 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    840.2 ns      │ 1.177 µs      │ 871.1 ns      │ 874.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  853.5 ns      │ 2.008 µs      │ 886.6 ns      │ 911.4 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       186.8 ns      │ 346.4 ns      │ 200.4 ns      │ 213.8 ns      │ 1000    │ 1000000
│  ├─ Line B                       188.3 ns      │ 320.3 ns      │ 193.8 ns      │ 197.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    373.8 ns      │ 599.7 ns      │ 387.7 ns      │ 390.7 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.14 µs       │ 1.676 µs      │ 1.182 µs      │ 1.189 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.134 µs      │ 1.763 µs      │ 1.191 µs      │ 1.205 µs      │ 1000    │ 1000000
├─ 03 ValidatedParquetLogline                    │               │               │               │         │
│  ├─ Line A                       168.4 ns      │ 310.6 ns      │ 173.9 ns      │ 178.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       167.9 ns      │ 292 ns        │ 173.1 ns      │ 178.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    327.9 ns      │ 464.3 ns      │ 339.2 ns      │ 344.6 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    999.2 ns      │ 1.529 µs      │ 1.034 µs      │ 1.043 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  989 ns        │ 1.487 µs      │ 1.035 µs      │ 1.044 µs      │ 1000    │ 1000000
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
Timer precision: 9 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     135.2 ns      │ 606 ns        │ 138.2 ns      │ 140.5 ns      │ 1000    │ 1000000
│  ├─ Line B                     135.1 ns      │ 185.3 ns      │ 138.6 ns      │ 140.7 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  263.3 ns      │ 437 ns        │ 270.8 ns      │ 273.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  790.5 ns      │ 962.8 ns      │ 812 ns        │ 815.4 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     182.1 ns      │ 559.1 ns      │ 188.7 ns      │ 193 ns        │ 1000    │ 1000000
│  ├─ Line B                     180.9 ns      │ 408 ns        │ 187.3 ns      │ 191.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  362.9 ns      │ 446.4 ns      │ 377.4 ns      │ 379.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.114 µs      │ 1.269 µs      │ 1.153 µs      │ 1.155 µs      │ 1000    │ 1000000
├─ 03 UnvalidatedParquetLogline                │               │               │               │         │
│  ├─ Line A                     156.5 ns      │ 365.6 ns      │ 162.7 ns      │ 166.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     154.8 ns      │ 250 ns        │ 164.4 ns      │ 168 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                  306.6 ns      │ 426.7 ns      │ 319.6 ns      │ 324.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  911.6 ns      │ 1.195 µs      │ 940.8 ns      │ 944.4 ns      │ 1000    │ 1000000
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
