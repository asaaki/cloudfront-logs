# Benchmarks: macos

## Benchmark environment

- Platform: `macos`
- Run date: `2026-09-09 22:34:13 +0200`
- OS: `Darwin 25.6.0 arm64`
- CPU: `Apple M1 Pro`
- RAM: `32.0 GiB`
- Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`
- Cargo: `cargo 1.98.0 (797e8a9bc 2026-08-05)`
- Git commit: `2e3795d`
- RUSTFLAGS: `-Ctarget-cpu=native`

## Configuration: no-features

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       326.6 ns      │ 456.2 ns      │ 328.8 ns      │ 330.3 ns      │ 1000    │ 1000000
│  ├─ Line B                       319.8 ns      │ 450 ns        │ 320.3 ns      │ 322.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    643.5 ns      │ 786.4 ns      │ 643.8 ns      │ 648.4 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.96 µs       │ 2.117 µs      │ 1.97 µs       │ 1.974 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.963 µs      │ 2.123 µs      │ 1.971 µs      │ 1.974 µs      │ 1000    │ 1000000
╰─ 01 ValidatedSimpleLogline                     │               │               │               │         │
   ├─ Line A                       349.9 ns      │ 457.8 ns      │ 350.3 ns      │ 351 ns        │ 1000    │ 1000000
   ├─ Line B                       343.9 ns      │ 458.9 ns      │ 344.3 ns      │ 345.1 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    690.9 ns      │ 810.1 ns      │ 691.8 ns      │ 693.3 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    2.111 µs      │ 2.242 µs      │ 2.123 µs      │ 2.126 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  2.115 µs      │ 2.266 µs      │ 2.128 µs      │ 2.13 µs       │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     320 ns        │ 353.7 ns      │ 323.5 ns      │ 323.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     312.8 ns      │ 348.5 ns      │ 316.9 ns      │ 316.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  629.9 ns      │ 741.3 ns      │ 636 ns        │ 637 ns        │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.951 µs      │ 2.084 µs      │ 1.966 µs      │ 1.969 µs      │ 1000    │ 1000000
╰─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
   ├─ Line A                     340.2 ns      │ 449.3 ns      │ 342.5 ns      │ 342.9 ns      │ 1000    │ 1000000
   ├─ Line B                     333.9 ns      │ 427.7 ns      │ 334.7 ns      │ 335.5 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  667.7 ns      │ 779.7 ns      │ 669.4 ns      │ 670.8 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  2.066 µs      │ 59.51 µs      │ 2.08 µs       │ 2.228 µs      │ 1000    │ 1000000

```

## Configuration: jiff

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       328.8 ns      │ 426.6 ns      │ 329.2 ns      │ 330.6 ns      │ 1000    │ 1000000
│  ├─ Line B                       314.5 ns      │ 414.9 ns      │ 314.9 ns      │ 315.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    640.5 ns      │ 757.4 ns      │ 641.5 ns      │ 643.3 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.954 µs      │ 2.113 µs      │ 1.964 µs      │ 1.967 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.96 µs       │ 2.121 µs      │ 1.969 µs      │ 1.974 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       354.3 ns      │ 468.8 ns      │ 354.8 ns      │ 355.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       338.1 ns      │ 440.6 ns      │ 338.5 ns      │ 339.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    691.3 ns      │ 807 ns        │ 691.9 ns      │ 692.6 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.116 µs      │ 2.246 µs      │ 2.126 µs      │ 2.13 µs       │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.119 µs      │ 2.279 µs      │ 2.131 µs      │ 2.135 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                      │               │               │               │         │
   ├─ Line A                       408 ns        │ 527.7 ns      │ 408.7 ns      │ 410.7 ns      │ 1000    │ 1000000
   ├─ Line B                       391.3 ns      │ 503.4 ns      │ 392 ns        │ 393.1 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    805.8 ns      │ 900.5 ns      │ 808.8 ns      │ 810.5 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    2.464 µs      │ 6.896 µs      │ 2.479 µs      │ 2.507 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  2.467 µs      │ 2.627 µs      │ 2.483 µs      │ 2.488 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     317 ns        │ 431.7 ns      │ 320.4 ns      │ 321 ns        │ 1000    │ 1000000
│  ├─ Line B                     310.8 ns      │ 427.2 ns      │ 314 ns        │ 314.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  627.7 ns      │ 733.5 ns      │ 632.9 ns      │ 633.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.94 µs       │ 2.118 µs      │ 1.966 µs      │ 1.967 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     340.2 ns      │ 425.4 ns      │ 342.2 ns      │ 342.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     334.1 ns      │ 435.6 ns      │ 334.9 ns      │ 336.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  667.4 ns      │ 861.7 ns      │ 670.4 ns      │ 671.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.057 µs      │ 2.211 µs      │ 2.07 µs       │ 2.076 µs      │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     388.1 ns      │ 488.2 ns      │ 388.6 ns      │ 390 ns        │ 1000    │ 1000000
   ├─ Line B                     381 ns        │ 484.7 ns      │ 381.7 ns      │ 382.5 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  772.9 ns      │ 895.2 ns      │ 778.3 ns      │ 780.5 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  2.36 µs       │ 3.215 µs      │ 2.367 µs      │ 2.375 µs      │ 1000    │ 1000000

```

## Configuration: time

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       328.5 ns      │ 421.2 ns      │ 329 ns        │ 329.5 ns      │ 1000    │ 1000000
│  ├─ Line B                       313.7 ns      │ 403.2 ns      │ 314.8 ns      │ 315.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    640.5 ns      │ 746.5 ns      │ 641.5 ns      │ 642.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.955 µs      │ 2.1 µs        │ 1.96 µs       │ 1.966 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.96 µs       │ 2.098 µs      │ 1.964 µs      │ 1.969 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       354.3 ns      │ 414.7 ns      │ 354.7 ns      │ 355.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       339.3 ns      │ 427.5 ns      │ 339.7 ns      │ 340.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    692 ns        │ 792.2 ns      │ 692.6 ns      │ 693.9 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.118 µs      │ 2.382 µs      │ 2.131 µs      │ 2.134 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.121 µs      │ 2.278 µs      │ 2.132 µs      │ 2.137 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                      │               │               │               │         │
   ├─ Line A                       419.8 ns      │ 508.2 ns      │ 421.7 ns      │ 422.1 ns      │ 1000    │ 1000000
   ├─ Line B                       404.5 ns      │ 561.3 ns      │ 405.6 ns      │ 406.6 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    831.1 ns      │ 961 ns        │ 837.8 ns      │ 839.5 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    2.548 µs      │ 2.713 µs      │ 2.563 µs      │ 2.567 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  2.554 µs      │ 2.74 µs       │ 2.568 µs      │ 2.571 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     318 ns        │ 363.8 ns      │ 321.3 ns      │ 321.8 ns      │ 1000    │ 1000000
│  ├─ Line B                     309.7 ns      │ 344.9 ns      │ 313.9 ns      │ 313.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  627.7 ns      │ 787.9 ns      │ 633 ns        │ 634 ns        │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.94 µs       │ 2.096 µs      │ 1.961 µs      │ 1.965 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     339.9 ns      │ 442.2 ns      │ 341.8 ns      │ 342.4 ns      │ 1000    │ 1000000
│  ├─ Line B                     333 ns        │ 375.9 ns      │ 333.8 ns      │ 335.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  666.7 ns      │ 765.8 ns      │ 669.5 ns      │ 670.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.053 µs      │ 2.218 µs      │ 2.069 µs      │ 2.072 µs      │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     403.8 ns      │ 516.1 ns      │ 405.7 ns      │ 406.2 ns      │ 1000    │ 1000000
   ├─ Line B                     396.9 ns      │ 550.8 ns      │ 398.3 ns      │ 399.2 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  808.3 ns      │ 924.4 ns      │ 814.5 ns      │ 816.1 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  2.465 µs      │ 2.608 µs      │ 2.48 µs       │ 2.483 µs      │ 1000    │ 1000000

```

## Configuration: chrono

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       328.6 ns      │ 447.5 ns      │ 329.1 ns      │ 329.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       313.8 ns      │ 357.6 ns      │ 314.9 ns      │ 315.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    640.2 ns      │ 732.1 ns      │ 641 ns        │ 642.1 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.953 µs      │ 2.074 µs      │ 1.962 µs      │ 1.967 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.959 µs      │ 2.107 µs      │ 1.967 µs      │ 1.972 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       356 ns        │ 498 ns        │ 356.4 ns      │ 357.4 ns      │ 1000    │ 1000000
│  ├─ Line B                       338 ns        │ 421.7 ns      │ 338.5 ns      │ 338.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    692.8 ns      │ 799.5 ns      │ 693.7 ns      │ 694.8 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.121 µs      │ 2.259 µs      │ 2.131 µs      │ 2.136 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.126 µs      │ 2.26 µs       │ 2.138 µs      │ 2.141 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                      │               │               │               │         │
   ├─ Line A                       513.6 ns      │ 638 ns        │ 521.7 ns      │ 522.8 ns      │ 1000    │ 1000000
   ├─ Line B                       494.8 ns      │ 579.3 ns      │ 503.7 ns      │ 504.7 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    1.021 µs      │ 1.331 µs      │ 1.038 µs      │ 1.041 µs      │ 1000    │ 1000000
   ├─ Sample File (no comments)    3.13 µs       │ 3.286 µs      │ 3.155 µs      │ 3.16 µs       │ 1000    │ 1000000
   ╰─ Sample File (with comments)  3.14 µs       │ 3.352 µs      │ 3.175 µs      │ 3.176 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     318.3 ns      │ 418.9 ns      │ 321.6 ns      │ 322.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     310.5 ns      │ 424.1 ns      │ 314.3 ns      │ 315 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                  628.2 ns      │ 743.3 ns      │ 633.5 ns      │ 634.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.942 µs      │ 2.063 µs      │ 1.963 µs      │ 1.965 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     340.3 ns      │ 401.7 ns      │ 342.1 ns      │ 342.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     333.4 ns      │ 433.5 ns      │ 333.9 ns      │ 335.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  667.2 ns      │ 785.6 ns      │ 670 ns        │ 671.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.057 µs      │ 2.293 µs      │ 2.073 µs      │ 2.077 µs      │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     507.5 ns      │ 653.7 ns      │ 515.2 ns      │ 516.5 ns      │ 1000    │ 1000000
   ├─ Line B                     497.2 ns      │ 628.2 ns      │ 507.8 ns      │ 508.2 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  1.021 µs      │ 1.152 µs      │ 1.033 µs      │ 1.036 µs      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  3.12 µs       │ 3.279 µs      │ 3.143 µs      │ 3.148 µs      │ 1000    │ 1000000

```

## Configuration: parquet

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       328.6 ns      │ 361.1 ns      │ 329 ns        │ 329.4 ns      │ 1000    │ 1000000
│  ├─ Line B                       314 ns        │ 409.6 ns      │ 315.3 ns      │ 316 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                    640.5 ns      │ 765.5 ns      │ 641.6 ns      │ 642.8 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.954 µs      │ 2.114 µs      │ 1.961 µs      │ 1.965 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.961 µs      │ 2.111 µs      │ 1.966 µs      │ 1.972 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       354.2 ns      │ 450.2 ns      │ 354.7 ns      │ 355.5 ns      │ 1000    │ 1000000
│  ├─ Line B                       338.1 ns      │ 368.6 ns      │ 338.5 ns      │ 338.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    691.2 ns      │ 914.1 ns      │ 691.9 ns      │ 693.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    2.121 µs      │ 2.246 µs      │ 2.129 µs      │ 2.133 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  2.121 µs      │ 2.268 µs      │ 2.138 µs      │ 2.141 µs      │ 1000    │ 1000000
╰─ 03 ValidatedParquetLogline                    │               │               │               │         │
   ├─ Line A                       502 ns        │ 612.8 ns      │ 515.6 ns      │ 516.4 ns      │ 1000    │ 1000000
   ├─ Line B                       487.3 ns      │ 610.3 ns      │ 501.5 ns      │ 502.1 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    1.01 µs       │ 1.189 µs      │ 1.026 µs      │ 1.027 µs      │ 1000    │ 1000000
   ├─ Sample File (no comments)    3.083 µs      │ 6.977 µs      │ 3.144 µs      │ 3.159 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  3.086 µs      │ 3.256 µs      │ 3.134 µs      │ 3.136 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     317.2 ns      │ 416.7 ns      │ 321.1 ns      │ 321.4 ns      │ 1000    │ 1000000
│  ├─ Line B                     309.9 ns      │ 396.5 ns      │ 313.8 ns      │ 313.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  628.5 ns      │ 1.458 µs      │ 632.8 ns      │ 638 ns        │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.945 µs      │ 2.082 µs      │ 1.962 µs      │ 1.964 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     340.5 ns      │ 386.4 ns      │ 341.9 ns      │ 342.6 ns      │ 1000    │ 1000000
│  ├─ Line B                     333.6 ns      │ 417.7 ns      │ 334.2 ns      │ 335.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  667.5 ns      │ 774.9 ns      │ 670.3 ns      │ 671.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.056 µs      │ 2.208 µs      │ 2.069 µs      │ 2.072 µs      │ 1000    │ 1000000
╰─ 03 UnvalidatedParquetLogline                │               │               │               │         │
   ├─ Line A                     486.3 ns      │ 566.5 ns      │ 489.5 ns      │ 490.4 ns      │ 1000    │ 1000000
   ├─ Line B                     474.2 ns      │ 567.1 ns      │ 482.5 ns      │ 483.2 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  972.8 ns      │ 1.1 µs        │ 985 ns        │ 986.5 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  2.931 µs      │ 3.181 µs      │ 2.985 µs      │ 2.991 µs      │ 1000    │ 1000000

```
