# Benchmarks: wsl

## Benchmark environment

- Platform: `wsl`
- Run date: `2026-09-09 22:23:53 +0200`
- OS: `Linux 6.18.33.2-microsoft-standard-WSL2 x86_64 GNU/Linux`
- CPU: `AMD Ryzen 9 7950X3D 16-Core Processor`
- RAM: `31.3 GiB`
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
Timer precision: 100 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       149.3 ns      │ 699 ns        │ 158 ns        │ 164.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       146.7 ns      │ 208.1 ns      │ 153.2 ns      │ 156.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    291.7 ns      │ 421.3 ns      │ 307 ns        │ 309.8 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    873 ns        │ 1.161 µs      │ 908.4 ns      │ 911.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  875 ns        │ 1.204 µs      │ 914 ns        │ 918 ns        │ 1000    │ 1000000
╰─ 01 ValidatedSimpleLogline                     │               │               │               │         │
   ├─ Line A                       181.1 ns      │ 373.1 ns      │ 189.3 ns      │ 191.9 ns      │ 1000    │ 1000000
   ├─ Line B                       181.1 ns      │ 254.3 ns      │ 187.5 ns      │ 190.4 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    358.5 ns      │ 1.184 µs      │ 372.5 ns      │ 377.5 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    1.109 µs      │ 1.512 µs      │ 1.149 µs      │ 1.16 µs       │ 1000    │ 1000000
   ╰─ Sample File (with comments)  1.109 µs      │ 1.639 µs      │ 1.148 µs      │ 1.156 µs      │ 1000    │ 1000000

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
│  ├─ Line A                     139.9 ns      │ 638.4 ns      │ 144.3 ns      │ 148.5 ns      │ 1000    │ 1000000
│  ├─ Line B                     143.8 ns      │ 257.1 ns      │ 196.9 ns      │ 196.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  270.1 ns      │ 337.6 ns      │ 282.2 ns      │ 284.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  812.2 ns      │ 1.208 µs      │ 842.9 ns      │ 846.2 ns      │ 1000    │ 1000000
╰─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
   ├─ Line A                     171.8 ns      │ 678.7 ns      │ 180.8 ns      │ 189 ns        │ 1000    │ 1000000
   ├─ Line B                     174.7 ns      │ 355.5 ns      │ 182.3 ns      │ 187 ns        │ 1000    │ 1000000
   ├─ Lines A+B                  342.4 ns      │ 571.1 ns      │ 359.1 ns      │ 364.4 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.065 µs      │ 1.535 µs      │ 1.102 µs      │ 1.108 µs      │ 1000    │ 1000000

```

## Configuration: jiff

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 100 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       146.5 ns      │ 774.5 ns      │ 149.9 ns      │ 155.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       146.4 ns      │ 320.9 ns      │ 150.8 ns      │ 154.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    286.1 ns      │ 535.4 ns      │ 295.1 ns      │ 301 ns        │ 1000    │ 1000000
│  ├─ Sample File (no comments)    852.8 ns      │ 1.203 µs      │ 871.6 ns      │ 876.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  856.3 ns      │ 1.215 µs      │ 878.3 ns      │ 885.1 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       181.4 ns      │ 663.7 ns      │ 188.8 ns      │ 193.7 ns      │ 1000    │ 1000000
│  ├─ Line B                       178.4 ns      │ 389.4 ns      │ 189.1 ns      │ 191.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    362.9 ns      │ 589 ns        │ 371.3 ns      │ 375.5 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.081 µs      │ 1.748 µs      │ 1.113 µs      │ 1.118 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.09 µs       │ 1.643 µs      │ 1.118 µs      │ 1.123 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                      │               │               │               │         │
   ├─ Line A                       209 ns        │ 516.4 ns      │ 215.9 ns      │ 218.7 ns      │ 1000    │ 1000000
   ├─ Line B                       210.2 ns      │ 482.9 ns      │ 217.4 ns      │ 223.1 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    419.5 ns      │ 662.9 ns      │ 435.5 ns      │ 440.8 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    1.247 µs      │ 1.721 µs      │ 1.291 µs      │ 1.299 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  1.26 µs       │ 1.901 µs      │ 1.291 µs      │ 1.298 µs      │ 1000    │ 1000000

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
│  ├─ Line A                     140.6 ns      │ 1.202 µs      │ 144.1 ns      │ 148.2 ns      │ 1000    │ 1000000
│  ├─ Line B                     138.3 ns      │ 194.2 ns      │ 143.4 ns      │ 145.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  267 ns        │ 380.3 ns      │ 276.1 ns      │ 278.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  798.3 ns      │ 1.089 µs      │ 821.4 ns      │ 826.1 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     173.4 ns      │ 546.4 ns      │ 180.7 ns      │ 183.5 ns      │ 1000    │ 1000000
│  ├─ Line B                     173 ns        │ 324.8 ns      │ 179.9 ns      │ 184.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  341.4 ns      │ 538.1 ns      │ 356.6 ns      │ 360.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.031 µs      │ 1.457 µs      │ 1.063 µs      │ 1.067 µs      │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     199.3 ns      │ 388.5 ns      │ 207.3 ns      │ 211.6 ns      │ 1000    │ 1000000
   ├─ Line B                     199.2 ns      │ 531.6 ns      │ 206.7 ns      │ 211.7 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  394.1 ns      │ 657.6 ns      │ 414.3 ns      │ 418.8 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.203 µs      │ 1.877 µs      │ 1.24 µs       │ 1.25 µs       │ 1000    │ 1000000

```

## Configuration: time

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 99 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       146.2 ns      │ 728.1 ns      │ 150.1 ns      │ 153.9 ns      │ 1000    │ 1000000
│  ├─ Line B                       146.4 ns      │ 218.6 ns      │ 150.1 ns      │ 152.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    286.8 ns      │ 423.4 ns      │ 294.2 ns      │ 297.9 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    849.3 ns      │ 1.133 µs      │ 881.3 ns      │ 890.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  849 ns        │ 1.387 µs      │ 878.4 ns      │ 923 ns        │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       177.8 ns      │ 684.9 ns      │ 183.4 ns      │ 186.6 ns      │ 1000    │ 1000000
│  ├─ Line B                       177 ns        │ 277.1 ns      │ 184.2 ns      │ 186.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    353.4 ns      │ 582 ns        │ 366.9 ns      │ 371.2 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.08 µs       │ 1.753 µs      │ 1.105 µs      │ 1.11 µs       │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.078 µs      │ 1.763 µs      │ 1.109 µs      │ 1.116 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                      │               │               │               │         │
   ├─ Line A                       237.9 ns      │ 431.3 ns      │ 246.8 ns      │ 249.4 ns      │ 1000    │ 1000000
   ├─ Line B                       237.6 ns      │ 375.1 ns      │ 246 ns        │ 249.6 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    472.6 ns      │ 686.6 ns      │ 493.4 ns      │ 497.6 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    1.446 µs      │ 2.14 µs       │ 1.49 µs       │ 1.497 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  1.454 µs      │ 1.945 µs      │ 1.492 µs      │ 1.498 µs      │ 1000    │ 1000000

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
│  ├─ Line A                     139.6 ns      │ 633.4 ns      │ 142.2 ns      │ 144.6 ns      │ 1000    │ 1000000
│  ├─ Line B                     137.2 ns      │ 291.7 ns      │ 142.8 ns      │ 145.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  269.6 ns      │ 447.1 ns      │ 277.5 ns      │ 283.3 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  814.5 ns      │ 1.081 µs      │ 836.9 ns      │ 843.5 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     174.5 ns      │ 613.7 ns      │ 181.1 ns      │ 183.2 ns      │ 1000    │ 1000000
│  ├─ Line B                     173.2 ns      │ 253.7 ns      │ 179 ns        │ 180.7 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  343.2 ns      │ 555.3 ns      │ 355.5 ns      │ 359.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.046 µs      │ 1.381 µs      │ 1.082 µs      │ 1.09 µs       │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     221.6 ns      │ 431.1 ns      │ 240.2 ns      │ 242.7 ns      │ 1000    │ 1000000
   ├─ Line B                     224.7 ns      │ 317.8 ns      │ 236.6 ns      │ 238.4 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  450.1 ns      │ 625.1 ns      │ 471.3 ns      │ 474.3 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.385 µs      │ 1.751 µs      │ 1.442 µs      │ 1.447 µs      │ 1000    │ 1000000

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
│  ├─ Line A                       147.7 ns      │ 443.5 ns      │ 152 ns        │ 156.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       146 ns        │ 345.7 ns      │ 150.5 ns      │ 155.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    286.3 ns      │ 560.9 ns      │ 297.3 ns      │ 303.2 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    850.5 ns      │ 1.116 µs      │ 876.9 ns      │ 885.6 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  856.6 ns      │ 1.236 µs      │ 877 ns        │ 881.4 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       179.5 ns      │ 563.3 ns      │ 188.4 ns      │ 192.3 ns      │ 1000    │ 1000000
│  ├─ Line B                       179.3 ns      │ 285.9 ns      │ 189.2 ns      │ 192 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                    357.8 ns      │ 641.8 ns      │ 371.2 ns      │ 374.8 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.092 µs      │ 1.484 µs      │ 1.119 µs      │ 1.125 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.085 µs      │ 1.225 µs      │ 1.118 µs      │ 1.123 µs      │ 1000    │ 1000000
╰─ 02 ValidatedTypedLogline                      │               │               │               │         │
   ├─ Line A                       357.9 ns      │ 498.7 ns      │ 369.3 ns      │ 371.7 ns      │ 1000    │ 1000000
   ├─ Line B                       358.6 ns      │ 712.2 ns      │ 372.1 ns      │ 377.8 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    714.8 ns      │ 1.054 µs      │ 740.7 ns      │ 747.8 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    2.17 µs       │ 3 µs          │ 2.214 µs      │ 2.237 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  2.177 µs      │ 2.552 µs      │ 2.222 µs      │ 2.24 µs       │ 1000    │ 1000000

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
│  ├─ Line A                     137.8 ns      │ 615.8 ns      │ 141 ns        │ 143.1 ns      │ 1000    │ 1000000
│  ├─ Line B                     136.2 ns      │ 192.4 ns      │ 139.2 ns      │ 141.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  269.7 ns      │ 353.8 ns      │ 277.2 ns      │ 279.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  808.9 ns      │ 1.106 µs      │ 833.7 ns      │ 838.8 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     173.3 ns      │ 897.2 ns      │ 181 ns        │ 184 ns        │ 1000    │ 1000000
│  ├─ Line B                     174.5 ns      │ 288.2 ns      │ 181.6 ns      │ 184.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  342.5 ns      │ 449 ns        │ 355.3 ns      │ 357.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.041 µs      │ 1.408 µs      │ 1.073 µs      │ 1.078 µs      │ 1000    │ 1000000
╰─ 02 UnvalidatedTypedLogline                  │               │               │               │         │
   ├─ Line A                     307.5 ns      │ 455.2 ns      │ 318.3 ns      │ 321.1 ns      │ 1000    │ 1000000
   ├─ Line B                     309.2 ns      │ 487.8 ns      │ 320.5 ns      │ 324.1 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  615.1 ns      │ 922.9 ns      │ 634.2 ns      │ 637.3 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.874 µs      │ 2.163 µs      │ 1.918 µs      │ 1.924 µs      │ 1000    │ 1000000

```

## Configuration: parquet

### `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Methodology v2: extracted values are black-boxed; compare matching input labels.
Timer precision: 99 ns
brwv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                        │               │               │               │         │
│  ├─ Line A                       145.6 ns      │ 444.4 ns      │ 150.6 ns      │ 157.2 ns      │ 1000    │ 1000000
│  ├─ Line B                       144.3 ns      │ 317.9 ns      │ 148.7 ns      │ 153.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    286 ns        │ 546.4 ns      │ 294.5 ns      │ 299.8 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    848.4 ns      │ 1.046 µs      │ 873.6 ns      │ 880.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  853.4 ns      │ 1.115 µs      │ 875.4 ns      │ 883.4 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                     │               │               │               │         │
│  ├─ Line A                       183.5 ns      │ 413.4 ns      │ 189.5 ns      │ 191.4 ns      │ 1000    │ 1000000
│  ├─ Line B                       178.6 ns      │ 288 ns        │ 186.4 ns      │ 188.7 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                    359.9 ns      │ 579.6 ns      │ 374 ns        │ 376.3 ns      │ 1000    │ 1000000
│  ├─ Sample File (no comments)    1.078 µs      │ 1.888 µs      │ 1.11 µs       │ 1.122 µs      │ 1000    │ 1000000
│  ╰─ Sample File (with comments)  1.075 µs      │ 1.64 µs       │ 1.112 µs      │ 1.117 µs      │ 1000    │ 1000000
╰─ 03 ValidatedParquetLogline                    │               │               │               │         │
   ├─ Line A                       296.9 ns      │ 509.9 ns      │ 310 ns        │ 313.8 ns      │ 1000    │ 1000000
   ├─ Line B                       298.5 ns      │ 881.4 ns      │ 309 ns        │ 314.3 ns      │ 1000    │ 1000000
   ├─ Lines A+B                    593.5 ns      │ 871.5 ns      │ 613.8 ns      │ 616.9 ns      │ 1000    │ 1000000
   ├─ Sample File (no comments)    1.784 µs      │ 2.161 µs      │ 1.824 µs      │ 1.829 µs      │ 1000    │ 1000000
   ╰─ Sample File (with comments)  1.782 µs      │ 2.111 µs      │ 1.821 µs      │ 1.826 µs      │ 1000    │ 1000000

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
│  ├─ Line A                     136 ns        │ 521.4 ns      │ 138.8 ns      │ 141.5 ns      │ 1000    │ 1000000
│  ├─ Line B                     135.7 ns      │ 207.3 ns      │ 140.2 ns      │ 142.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  266.5 ns      │ 779.6 ns      │ 273.7 ns      │ 278.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  790.9 ns      │ 929.6 ns      │ 810.6 ns      │ 813.6 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     171 ns        │ 389 ns        │ 177.8 ns      │ 180.7 ns      │ 1000    │ 1000000
│  ├─ Line B                     170.5 ns      │ 286.5 ns      │ 177.5 ns      │ 179.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  338.9 ns      │ 897.6 ns      │ 353 ns        │ 356.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.031 µs      │ 1.703 µs      │ 1.063 µs      │ 1.067 µs      │ 1000    │ 1000000
╰─ 03 UnvalidatedParquetLogline                │               │               │               │         │
   ├─ Line A                     293 ns        │ 513.2 ns      │ 299.4 ns      │ 302.5 ns      │ 1000    │ 1000000
   ├─ Line B                     290.2 ns      │ 417.8 ns      │ 299 ns        │ 302 ns        │ 1000    │ 1000000
   ├─ Lines A+B                  579.8 ns      │ 775.3 ns      │ 598.1 ns      │ 601.1 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.751 µs      │ 2.629 µs      │ 1.786 µs      │ 1.792 µs      │ 1000    │ 1000000

```
