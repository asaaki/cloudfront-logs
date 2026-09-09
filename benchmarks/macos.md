# Benchmarks: macos

## Benchmark environment

- Platform: `macos`
- Run date: `2026-09-08 17:49:25 +0200`
- OS: `Darwin 25.6.0 arm64`
- CPU: `Apple M1 Pro`
- RAM: `32.0 GiB`
- Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`
- Cargo: `cargo 1.98.0 (797e8a9bc 2026-08-05)`
- Git commit: `790c05c`
- RUSTFLAGS: `-Ctarget-cpu=native`

## Configuration: no-features

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwv                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                   │               │               │               │         │
│  ├─ Line A                  326.3 ns      │ 403.9 ns      │ 327.6 ns      │ 328.4 ns      │ 1000    │ 1000000
│  ├─ Line B                  319.3 ns      │ 354.6 ns      │ 319.6 ns      │ 321.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B               643.4 ns      │ 4.991 µs      │ 644.5 ns      │ 656 ns        │ 1000    │ 1000000
│  ╰─ Sample File             1.968 µs      │ 2.176 µs      │ 1.983 µs      │ 1.989 µs      │ 1000    │ 1000000
╰─ 01 ValidatedSimpleLogline                │               │               │               │         │
   ├─ Line A                  349.8 ns      │ 450.6 ns      │ 350.3 ns      │ 351.9 ns      │ 1000    │ 1000000
   ├─ Line B                  342.9 ns      │ 454.6 ns      │ 343.3 ns      │ 345.3 ns      │ 1000    │ 1000000
   ├─ Lines A+B               691.3 ns      │ 807.7 ns      │ 691.8 ns      │ 694.1 ns      │ 1000    │ 1000000
   ╰─ Sample File             2.117 µs      │ 4.268 µs      │ 2.129 µs      │ 2.144 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     320 ns        │ 434 ns        │ 323.5 ns      │ 325.9 ns      │ 1000    │ 1000000
│  ├─ Line B                     312.9 ns      │ 420.2 ns      │ 317.7 ns      │ 319.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  630.1 ns      │ 7.778 µs      │ 637.3 ns      │ 653.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.947 µs      │ 2.169 µs      │ 1.97 µs       │ 1.975 µs      │ 1000    │ 1000000
╰─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
   ├─ Line A                     339.2 ns      │ 437.7 ns      │ 341.2 ns      │ 342.5 ns      │ 1000    │ 1000000
   ├─ Line B                     332.8 ns      │ 444.4 ns      │ 334.6 ns      │ 335.3 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  666.8 ns      │ 792.7 ns      │ 669.1 ns      │ 671.9 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  2.062 µs      │ 4.599 µs      │ 2.077 µs      │ 2.09 µs       │ 1000    │ 1000000

```

## Configuration: jiff

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwv                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                   │               │               │               │         │
│  ├─ Line A                  324.3 ns      │ 376 ns        │ 324.6 ns      │ 325.3 ns      │ 1000    │ 1000000
│  ├─ Line B                  311.3 ns      │ 345 ns        │ 311.6 ns      │ 312.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B               633.2 ns      │ 745.1 ns      │ 633.8 ns      │ 636.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File             1.93 µs       │ 2.126 µs      │ 1.938 µs      │ 1.944 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                │               │               │               │         │
│  ├─ Line A                  348.6 ns      │ 498.5 ns      │ 349 ns        │ 350.2 ns      │ 1000    │ 1000000
│  ├─ Line B                  333.1 ns      │ 437.7 ns      │ 333.6 ns      │ 337.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B               680.2 ns      │ 2.304 µs      │ 680.8 ns      │ 692.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File             2.108 µs      │ 2.401 µs      │ 2.121 µs      │ 2.134 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                 │               │               │               │         │
   ├─ Line A                  399.3 ns      │ 514.1 ns      │ 399.7 ns      │ 401.5 ns      │ 1000    │ 1000000
   ├─ Line B                  382.5 ns      │ 464.8 ns      │ 382.9 ns      │ 384.4 ns      │ 1000    │ 1000000
   ├─ Lines A+B               788.6 ns      │ 2.881 µs      │ 792.5 ns      │ 807.2 ns      │ 1000    │ 1000000
   ╰─ Sample File             2.452 µs      │ 4.707 µs      │ 2.487 µs      │ 2.497 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     312.4 ns      │ 414.4 ns      │ 313.7 ns      │ 315.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     306.3 ns      │ 414.1 ns      │ 307.9 ns      │ 308.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  613.9 ns      │ 726.6 ns      │ 615 ns        │ 617.3 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.867 µs      │ 2.062 µs      │ 1.889 µs      │ 1.898 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     334.4 ns      │ 447.2 ns      │ 336.2 ns      │ 339.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     327.6 ns      │ 432.5 ns      │ 330.1 ns      │ 334.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  657.4 ns      │ 2.219 µs      │ 661.7 ns      │ 675.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.03 µs       │ 2.215 µs      │ 2.063 µs      │ 2.072 µs      │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     383 ns        │ 507.3 ns      │ 383.5 ns      │ 388.8 ns      │ 1000    │ 1000000
   ├─ Line B                     375.9 ns      │ 508.1 ns      │ 376.4 ns      │ 381 ns        │ 1000    │ 1000000
   ├─ Lines A+B                  762.4 ns      │ 4.396 µs      │ 768.6 ns      │ 787 ns        │ 1000    │ 1000000
   ╰─ Sample File (no comments)  2.327 µs      │ 2.548 µs      │ 2.347 µs      │ 2.36 µs       │ 1000    │ 1000000

```

## Configuration: time

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwv                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                   │               │               │               │         │
│  ├─ Line A                  325.2 ns      │ 448.9 ns      │ 325.5 ns      │ 328.8 ns      │ 1000    │ 1000000
│  ├─ Line B                  311.5 ns      │ 2.221 µs      │ 311.8 ns      │ 322.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B               633.5 ns      │ 797.1 ns      │ 633.9 ns      │ 638.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File             1.931 µs      │ 2.108 µs      │ 1.945 µs      │ 1.952 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                │               │               │               │         │
│  ├─ Line A                  348.8 ns      │ 467.1 ns      │ 349.2 ns      │ 352.9 ns      │ 1000    │ 1000000
│  ├─ Line B                  333.3 ns      │ 436.8 ns      │ 333.7 ns      │ 337.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B               680.4 ns      │ 2.097 µs      │ 681.2 ns      │ 697.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File             2.11 µs       │ 3.139 µs      │ 2.132 µs      │ 2.142 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                 │               │               │               │         │
   ├─ Line A                  414.2 ns      │ 541.2 ns      │ 416 ns        │ 421 ns        │ 1000    │ 1000000
   ├─ Line B                  398.7 ns      │ 521.1 ns      │ 401 ns        │ 404.9 ns      │ 1000    │ 1000000
   ├─ Lines A+B               820.6 ns      │ 2.369 µs      │ 829.5 ns      │ 843.1 ns      │ 1000    │ 1000000
   ╰─ Sample File             2.563 µs      │ 2.855 µs      │ 2.606 µs      │ 2.614 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     312.7 ns      │ 9.075 µs      │ 314.3 ns      │ 337.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     306.6 ns      │ 415.2 ns      │ 307.7 ns      │ 310.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  614.2 ns      │ 733.2 ns      │ 615.8 ns      │ 620.3 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.868 µs      │ 2.029 µs      │ 1.89 µs       │ 1.897 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     334.5 ns      │ 445.6 ns      │ 336.3 ns      │ 338.5 ns      │ 1000    │ 1000000
│  ├─ Line B                     327.6 ns      │ 415.2 ns      │ 328.1 ns      │ 330.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  657.5 ns      │ 1.533 µs      │ 660.6 ns      │ 671.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.033 µs      │ 3.648 µs      │ 2.058 µs      │ 2.068 µs      │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     398.6 ns      │ 508.1 ns      │ 400.8 ns      │ 404 ns        │ 1000    │ 1000000
   ├─ Line B                     391.3 ns      │ 493.5 ns      │ 392.4 ns      │ 394.8 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  797.9 ns      │ 2.354 µs      │ 804.4 ns      │ 816.1 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  2.439 µs      │ 2.684 µs      │ 2.453 µs      │ 2.462 µs      │ 1000    │ 1000000

```

## Configuration: chrono

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwv                          fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                   │               │               │               │         │
│  ├─ Line A                  325.4 ns      │ 447.8 ns      │ 325.9 ns      │ 326.8 ns      │ 1000    │ 1000000
│  ├─ Line B                  311.8 ns      │ 391.4 ns      │ 312.2 ns      │ 313.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B               633.7 ns      │ 717.1 ns      │ 634.2 ns      │ 636.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File             1.933 µs      │ 3.632 µs      │ 1.949 µs      │ 1.966 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                │               │               │               │         │
│  ├─ Line A                  349 ns        │ 448.2 ns      │ 349.4 ns      │ 350.5 ns      │ 1000    │ 1000000
│  ├─ Line B                  333.4 ns      │ 621.7 ns      │ 334 ns        │ 339.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B               680.7 ns      │ 797.5 ns      │ 681.7 ns      │ 687 ns        │ 1000    │ 1000000
│  ╰─ Sample File             2.11 µs       │ 4.242 µs      │ 2.131 µs      │ 2.149 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                 │               │               │               │         │
   ├─ Line A                  508.1 ns      │ 623.7 ns      │ 513.9 ns      │ 518 ns        │ 1000    │ 1000000
   ├─ Line B                  487.4 ns      │ 611.7 ns      │ 498.4 ns      │ 503.1 ns      │ 1000    │ 1000000
   ├─ Lines A+B               1.003 µs      │ 1.161 µs      │ 1.028 µs      │ 1.031 µs      │ 1000    │ 1000000
   ╰─ Sample File             3.107 µs      │ 7.252 µs      │ 3.159 µs      │ 3.173 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     313 ns        │ 5.68 µs       │ 314.3 ns      │ 323.4 ns      │ 1000    │ 1000000
│  ├─ Line B                     306.9 ns      │ 397.7 ns      │ 307.9 ns      │ 311.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  615 ns        │ 798.8 ns      │ 616.6 ns      │ 618.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.867 µs      │ 2.075 µs      │ 1.883 µs      │ 1.888 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     334.8 ns      │ 490.9 ns      │ 336.6 ns      │ 337.4 ns      │ 1000    │ 1000000
│  ├─ Line B                     328.7 ns      │ 436.9 ns      │ 329.1 ns      │ 331.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  658.3 ns      │ 4.599 µs      │ 660.7 ns      │ 672.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.033 µs      │ 2.237 µs      │ 2.054 µs      │ 2.057 µs      │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     497 ns        │ 553.9 ns      │ 501.1 ns      │ 502.1 ns      │ 1000    │ 1000000
   ├─ Line B                     481.3 ns      │ 600.9 ns      │ 494.8 ns      │ 495.8 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  978.7 ns      │ 3.128 µs      │ 1.006 µs      │ 1.013 µs      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  3.007 µs      │ 4.104 µs      │ 3.048 µs      │ 3.059 µs      │ 1000    │ 1000000

```

## Configuration: parquet

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwv                           fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                   324.2 ns      │ 425 ns        │ 324.6 ns      │ 326.3 ns      │ 1000    │ 1000000
│  ├─ Line B                   311.2 ns      │ 432.3 ns      │ 311.5 ns      │ 312.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                633.3 ns      │ 730 ns        │ 633.8 ns      │ 635.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File              1.933 µs      │ 18.32 µs      │ 1.946 µs      │ 2.025 µs      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                   349.3 ns      │ 651.9 ns      │ 350.2 ns      │ 361.5 ns      │ 1000    │ 1000000
│  ├─ Line B                   333.2 ns      │ 521.1 ns      │ 334 ns        │ 344.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                680.8 ns      │ 844.7 ns      │ 681.8 ns      │ 690.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File              2.109 µs      │ 8.149 µs      │ 2.136 µs      │ 2.19 µs       │ 1000    │ 1000000
╰─ 03 ValidatedParquetLogline                │               │               │               │         │
   ├─ Line A                   499.6 ns      │ 2.395 µs      │ 511.6 ns      │ 525.5 ns      │ 1000    │ 1000000
   ├─ Line B                   478.8 ns      │ 3.964 µs      │ 496.1 ns      │ 515.1 ns      │ 1000    │ 1000000
   ├─ Lines A+B                982.2 ns      │ 15.49 µs      │ 1.021 µs      │ 1.061 µs      │ 1000    │ 1000000
   ╰─ Sample File              3.071 µs      │ 8.139 µs      │ 3.144 µs      │ 3.196 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 41 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     313.4 ns      │ 478.5 ns      │ 314.8 ns      │ 318.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     308 ns        │ 443.5 ns      │ 308.9 ns      │ 311.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  615.8 ns      │ 771 ns        │ 617.5 ns      │ 621.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.868 µs      │ 10.31 µs      │ 1.913 µs      │ 1.994 µs      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     335.1 ns      │ 599.2 ns      │ 336.8 ns      │ 341.8 ns      │ 1000    │ 1000000
│  ├─ Line B                     328.2 ns      │ 535.9 ns      │ 330.4 ns      │ 341.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  658.1 ns      │ 849.1 ns      │ 663.1 ns      │ 680.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  2.031 µs      │ 6.422 µs      │ 2.07 µs       │ 2.104 µs      │ 1000    │ 1000000
╰─ 03 UnvalidatedParquetLogline                │               │               │               │         │
   ├─ Line A                     477.2 ns      │ 686 ns        │ 485.9 ns      │ 495.4 ns      │ 1000    │ 1000000
   ├─ Line B                     470.4 ns      │ 718.4 ns      │ 480 ns        │ 490.4 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  963.5 ns      │ 1.203 µs      │ 993.9 ns      │ 1.011 µs      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  2.93 µs       │ 7.394 µs      │ 2.975 µs      │ 2.989 µs      │ 1000    │ 1000000

```
