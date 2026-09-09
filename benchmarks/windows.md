# Benchmarks: windows

## Benchmark environment

- Platform: `windows`
- Run date: `2026-09-09 22:26:37 +02:00`
- OS: `Microsoft Windows 11 Pro`
- CPU: `AMD Ryzen 9 7950X3D 16-Core Processor`
- RAM: `63.7 GiB`
- Toolchain: `rustc 1.98.1 (48a229cea 2026-09-01)`
- Cargo: `cargo 1.98.1 (797e8a9bc 2026-08-05)`
- Git commit: `2e3795d`
- RUSTFLAGS: `-Ctarget-cpu=native`

## Configuration: no-features

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
Timer precision: 100 ns
│  ├─ Line A                       130 ns        │ 160.4 ns      │ 132.8 ns      │ 133.6 ns      │ 1000    │ 1000000
│  ├─ Line B                       130.1 ns      │ 172.9 ns      │ 132.6 ns      │ 133.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    254.4 ns      │ 387.7 ns      │ 265 ns        │ 266.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    795.2 ns      │ 1.127 µs      │ 821.6 ns      │ 824.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  790.8 ns      │ 934.1 ns      │ 811.7 ns      │ 812.9 ns      │ 1000    │ 1000000
╰─ 01 ValidatedSimpleLogline                     │               │               │               │         │
   ├─ Line A                       169 ns        │ 228.9 ns      │ 174.1 ns      │ 174.7 ns      │ 1000    │ 1000000
   ├─ Line B                       170.3 ns      │ 211 ns        │ 176.9 ns      │ 177.2 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    336.6 ns      │ 554.8 ns      │ 348 ns        │ 349.6 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    1.049 µs      │ 1.77 µs       │ 1.066 µs      │ 1.071 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  1.049 µs      │ 1.782 µs      │ 1.069 µs      │ 1.08 µs       │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     134.8 ns      │ 172.5 ns      │ 139.2 ns      │ 139.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     134.8 ns      │ 170 ns        │ 137.6 ns      │ 138.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  265.5 ns      │ 367.4 ns      │ 272.3 ns      │ 273.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  830.9 ns      │ 947.3 ns      │ 848.6 ns      │ 850.2 ns      │ 1000    │ 1000000
╰─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
   ├─ Line A                     165.5 ns      │ 210.8 ns      │ 169.8 ns      │ 170.9 ns      │ 1000    │ 1000000
   ├─ Line B                     164.9 ns      │ 218 ns        │ 170.9 ns      │ 171.9 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  325 ns        │ 388.5 ns      │ 337.9 ns      │ 339.1 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.015 µs      │ 1.306 µs      │ 1.036 µs      │ 1.039 µs      │ 1000    │ 1000000

```

## Configuration: jiff

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
Timer precision: 100 ns
│  ├─ Line A                       129 ns        │ 164.6 ns      │ 131.7 ns      │ 132.8 ns      │ 1000    │ 1000000
│  ├─ Line B                       129.6 ns      │ 190.9 ns      │ 133.3 ns      │ 134.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    255.4 ns      │ 360.4 ns      │ 261 ns        │ 262.2 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    779.6 ns      │ 932.6 ns      │ 804.6 ns      │ 806 ns        │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  779.5 ns      │ 944.8 ns      │ 804.2 ns      │ 805.4 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       168.5 ns      │ 239.5 ns      │ 172.7 ns      │ 174.1 ns      │ 1000    │ 1000000
│  ├─ Line B                       170.5 ns      │ 239.1 ns      │ 176.5 ns      │ 177.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    335.5 ns      │ 394 ns        │ 343.2 ns      │ 344.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.024 µs      │ 1.535 µs      │ 1.048 µs      │ 1.058 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.034 µs      │ 1.575 µs      │ 1.052 µs      │ 1.056 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                      │               │               │               │         │
   ├─ Line A                       236.1 ns      │ 323.3 ns      │ 239.9 ns      │ 241.9 ns      │ 1000    │ 1000000
   ├─ Line B                       238.6 ns      │ 433.3 ns      │ 242.5 ns      │ 244.7 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    464.8 ns      │ 595 ns        │ 473.8 ns      │ 475.6 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    1.43 µs       │ 2.562 µs      │ 1.455 µs      │ 1.465 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  1.434 µs      │ 2.248 µs      │ 1.461 µs      │ 1.466 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     124.8 ns      │ 150.8 ns      │ 126.7 ns      │ 127.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     123 ns        │ 156.2 ns      │ 125 ns        │ 125.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  244.4 ns      │ 315.3 ns      │ 250.1 ns      │ 251.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  726.2 ns      │ 1.029 µs      │ 737.8 ns      │ 740.2 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     170.6 ns      │ 230.9 ns      │ 175.6 ns      │ 176.5 ns      │ 1000    │ 1000000
│  ├─ Line B                     169.9 ns      │ 270.4 ns      │ 176 ns        │ 177.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  332 ns        │ 483.4 ns      │ 348.4 ns      │ 349.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.01 µs       │ 1.431 µs      │ 1.048 µs      │ 1.05 µs       │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     226.6 ns      │ 273.7 ns      │ 230.5 ns      │ 231.1 ns      │ 1000    │ 1000000
   ├─ Line B                     227.8 ns      │ 328.5 ns      │ 231.2 ns      │ 232.2 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  450.6 ns      │ 624.8 ns      │ 457.5 ns      │ 458.8 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.364 µs      │ 2.054 µs      │ 1.398 µs      │ 1.401 µs      │ 1000    │ 1000000

```

## Configuration: time

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
Timer precision: 100 ns
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       128.7 ns      │ 198.6 ns      │ 131.1 ns      │ 132.5 ns      │ 1000    │ 1000000
│  ├─ Line B                       128.1 ns      │ 159.8 ns      │ 130.2 ns      │ 131.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    251.4 ns      │ 429.2 ns      │ 256.5 ns      │ 258.1 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    760.9 ns      │ 856.8 ns      │ 781.6 ns      │ 783.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  770.1 ns      │ 1.327 µs      │ 788.7 ns      │ 791.1 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       165.6 ns      │ 216.8 ns      │ 171.1 ns      │ 172.5 ns      │ 1000    │ 1000000
│  ├─ Line B                       167.4 ns      │ 255.5 ns      │ 175.1 ns      │ 176.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    330 ns        │ 420 ns        │ 341.6 ns      │ 342.9 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.019 µs      │ 1.752 µs      │ 1.046 µs      │ 1.05 µs       │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.025 µs      │ 1.705 µs      │ 1.057 µs      │ 1.064 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                      │               │               │               │         │
   ├─ Line A                       224.9 ns      │ 339.1 ns      │ 234.9 ns      │ 237.2 ns      │ 1000    │ 1000000
   ├─ Line B                       229.1 ns      │ 378.7 ns      │ 240.5 ns      │ 242.7 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    466.4 ns      │ 666.1 ns      │ 484 ns        │ 487.3 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    1.42 µs       │ 2.212 µs      │ 1.464 µs      │ 1.467 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  1.424 µs      │ 1.878 µs      │ 1.463 µs      │ 1.468 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     125.3 ns      │ 188.5 ns      │ 128.3 ns      │ 129.1 ns      │ 1000    │ 1000000
│  ├─ Line B                     122.9 ns      │ 158.2 ns      │ 124.9 ns      │ 125.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  240.3 ns      │ 292 ns        │ 246.3 ns      │ 246.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  746.6 ns      │ 853.7 ns      │ 755.9 ns      │ 757.4 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     164.8 ns      │ 197.8 ns      │ 168.7 ns      │ 169.1 ns      │ 1000    │ 1000000
│  ├─ Line B                     165.2 ns      │ 230.1 ns      │ 169.1 ns      │ 169.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  324.3 ns      │ 567.9 ns      │ 335.1 ns      │ 342.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  985.8 ns      │ 1.509 µs      │ 1.007 µs      │ 1.01 µs       │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     235.4 ns      │ 350.3 ns      │ 241.9 ns      │ 243.1 ns      │ 1000    │ 1000000
   ├─ Line B                     235.6 ns      │ 336 ns        │ 242.3 ns      │ 243.9 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  468 ns        │ 795.5 ns      │ 479.7 ns      │ 490.1 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.415 µs      │ 2.342 µs      │ 1.459 µs      │ 1.471 µs      │ 1000    │ 1000000

```

## Configuration: chrono

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 100 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       129.8 ns      │ 164 ns        │ 133 ns        │ 133.8 ns      │ 1000    │ 1000000
│  ├─ Line B                       129.2 ns      │ 162 ns        │ 132.5 ns      │ 133.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    254.7 ns      │ 303.5 ns      │ 260.2 ns      │ 261.3 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    772.5 ns      │ 1.163 µs      │ 788.5 ns      │ 792.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  774.7 ns      │ 901.2 ns      │ 799.1 ns      │ 800.5 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       168.6 ns      │ 214.5 ns      │ 172.7 ns      │ 173.9 ns      │ 1000    │ 1000000
│  ├─ Line B                       169.6 ns      │ 245.8 ns      │ 176.1 ns      │ 177.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    336.1 ns      │ 410.5 ns      │ 345.1 ns      │ 346.4 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.02 µs       │ 1.583 µs      │ 1.042 µs      │ 1.046 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.024 µs      │ 1.727 µs      │ 1.046 µs      │ 1.049 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                      │               │               │               │         │
   ├─ Line A                       303.1 ns      │ 374.6 ns      │ 310.3 ns      │ 311.5 ns      │ 1000    │ 1000000
   ├─ Line B                       303.4 ns      │ 377.4 ns      │ 312.7 ns      │ 314.2 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    608.9 ns      │ 677.9 ns      │ 622.1 ns      │ 623.4 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    1.839 µs      │ 3.026 µs      │ 1.875 µs      │ 1.904 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  1.843 µs      │ 2.426 µs      │ 1.89 µs       │ 1.897 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     124.7 ns      │ 166.1 ns      │ 127 ns        │ 127.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     123.5 ns      │ 171.7 ns      │ 125.4 ns      │ 126.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  243.2 ns      │ 290.4 ns      │ 248.9 ns      │ 249.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  738.4 ns      │ 902.1 ns      │ 749.4 ns      │ 751.1 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     163.3 ns      │ 274.5 ns      │ 168.7 ns      │ 170.5 ns      │ 1000    │ 1000000
│  ├─ Line B                     163.8 ns      │ 198.6 ns      │ 168.8 ns      │ 169.7 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  322.9 ns      │ 538 ns        │ 333.1 ns      │ 335 ns        │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  980.7 ns      │ 1.39 µs       │ 1.015 µs      │ 1.018 µs      │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     310.9 ns      │ 511.7 ns      │ 319 ns        │ 321 ns        │ 1000    │ 1000000
   ├─ Line B                     306.6 ns      │ 490.1 ns      │ 321.7 ns      │ 322.8 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  615.2 ns      │ 777.9 ns      │ 647.3 ns      │ 649 ns        │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.873 µs      │ 2.176 µs      │ 1.933 µs      │ 1.939 µs      │ 1000    │ 1000000

```

## Configuration: parquet

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 100 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       129.5 ns      │ 173.2 ns      │ 133.7 ns      │ 134.8 ns      │ 1000    │ 1000000
│  ├─ Line B                       131.4 ns      │ 184.1 ns      │ 134.2 ns      │ 135.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    256.1 ns      │ 313.4 ns      │ 261.2 ns      │ 262.2 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    770.5 ns      │ 1.21 µs       │ 789.1 ns      │ 792.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  779 ns        │ 1.082 µs      │ 797.6 ns      │ 800.9 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       172.5 ns      │ 227.6 ns      │ 176.6 ns      │ 177.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       171 ns        │ 227.2 ns      │ 178.6 ns      │ 179.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    335.2 ns      │ 447.2 ns      │ 348.1 ns      │ 349.3 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.025 µs      │ 1.178 µs      │ 1.047 µs      │ 1.055 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.034 µs      │ 1.744 µs      │ 1.068 µs      │ 1.083 µs      │ 1000    │ 1000000
╰─ 03 ValidatedParquetLogline                    │               │               │               │         │
   ├─ Line A                       287.2 ns      │ 473.6 ns      │ 293.5 ns      │ 295.4 ns      │ 1000    │ 1000000
   ├─ Line B                       289 ns        │ 339.9 ns      │ 295.6 ns      │ 297 ns        │ 1000    │ 1000000
   ├─ Lines A+B                    577.1 ns      │ 826.1 ns      │ 588.7 ns      │ 591.6 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    1.726 µs      │ 2.758 µs      │ 1.753 µs      │ 1.761 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  1.737 µs      │ 2.34 µs       │ 1.767 µs      │ 1.775 µs      │ 1000    │ 1000000

```

### `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     124.2 ns      │ 178 ns        │ 134.2 ns      │ 134.6 ns      │ 1000    │ 1000000
│  ├─ Line B                     123.5 ns      │ 155.6 ns      │ 125.3 ns      │ 126 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                  243.6 ns      │ 301.4 ns      │ 248.2 ns      │ 249.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  737.5 ns      │ 1 µs          │ 753.3 ns      │ 757.3 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     163.3 ns      │ 226.3 ns      │ 171.4 ns      │ 171.3 ns      │ 1000    │ 1000000
│  ├─ Line B                     164.7 ns      │ 221.4 ns      │ 172.1 ns      │ 172.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  324.5 ns      │ 409.3 ns      │ 339.2 ns      │ 339.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  984.1 ns      │ 1.273 µs      │ 1.015 µs      │ 1.018 µs      │ 1000    │ 1000000
╰─ 03 UnvalidatedParquetLogline                │               │               │               │         │
   ├─ Line A                     278.2 ns      │ 356.4 ns      │ 287.5 ns      │ 288 ns        │ 1000    │ 1000000
   ├─ Line B                     278.4 ns      │ 390.8 ns      │ 286.8 ns      │ 287.7 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  555.6 ns      │ 613.5 ns      │ 570.2 ns      │ 570.7 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.685 µs      │ 1.806 µs      │ 1.714 µs      │ 1.715 µs      │ 1000    │ 1000000

```
