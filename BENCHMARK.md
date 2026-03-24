# Benchmarks

This document tracks the current benchmark targets defined in `Cargo.toml`:

- `brwv` -> `benches/borrowed-real-world-validated.rs`
- `brwu` -> `benches/borrowed-real-world-unvalidated.rs`

## Benchmark environment

- Run date: `2026-02-13 13:43:07 +01:00`
- OS: `Microsoft Windows 11 Pro`
- CPU: `AMD Ryzen 9 7950X3D 16-Core Processor`
- RAM: `63.7 GiB`
- Toolchain: `rustc 1.93.1 (01f6ddf75 2026-02-11)`
- Cargo: `cargo 1.93.1 (083ac5135 2025-12-15)`

## Commands

```powershell
$env:RUSTFLAGS='-Ctarget-cpu=native'
cargo bench -q --all-features --bench brwv
cargo bench -q --all-features --bench brwu
```

## Results: `brwv` (validated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
brwv                           fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 ValidatedRawLogline                    │               │               │               │         │
Timer precision: 100 ns
│  ├─ Line A                   124.1 ns      │ 192.4 ns      │ 128.4 ns      │ 130.5 ns      │ 1000    │ 1000000
│  ├─ Line B                   126.7 ns      │ 163.3 ns      │ 130.9 ns      │ 131.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                249.2 ns      │ 369 ns        │ 255.4 ns      │ 257.3 ns      │ 1000    │ 1000000
│  ╰─ Sample File              751.4 ns      │ 959.9 ns      │ 779.8 ns      │ 781.1 ns      │ 1000    │ 1000000
├─ 01 ValidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                   168.4 ns      │ 221.2 ns      │ 174.3 ns      │ 174.9 ns      │ 1000    │ 1000000
│  ├─ Line B                   173.2 ns      │ 224.1 ns      │ 180 ns        │ 181.3 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                337.1 ns      │ 398.2 ns      │ 351.2 ns      │ 351.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File              1.046 µs      │ 1.153 µs      │ 1.091 µs      │ 1.092 µs      │ 1000    │ 1000000
├─ 02 ValidatedChronoLogline                 │               │               │               │         │
│  ├─ Line A                   290.1 ns      │ 338.7 ns      │ 300.9 ns      │ 301.9 ns      │ 1000    │ 1000000
│  ├─ Line B                   292.8 ns      │ 471.6 ns      │ 306.6 ns      │ 308.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                589.1 ns      │ 705.4 ns      │ 615.4 ns      │ 616.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File              1.766 µs      │ 3.009 µs      │ 1.868 µs      │ 1.866 µs      │ 1000    │ 1000000
├─ 03 ValidatedTimeLogline                   │               │               │               │         │
│  ├─ Line A                   224.4 ns      │ 341.4 ns      │ 229.4 ns      │ 231.7 ns      │ 1000    │ 1000000
│  ├─ Line B                   225.1 ns      │ 336.2 ns      │ 234.1 ns      │ 235.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                445.4 ns      │ 849.2 ns      │ 465.9 ns      │ 473.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File              1.351 µs      │ 1.628 µs      │ 1.437 µs      │ 1.436 µs      │ 1000    │ 1000000
╰─ 04 ValidatedParquetLogline                │               │               │               │         │
   ├─ Line A                   274.7 ns      │ 490.4 ns      │ 282.1 ns      │ 283.6 ns      │ 1000    │ 1000000
   ├─ Line B                   276.7 ns      │ 339 ns        │ 282.7 ns      │ 283.9 ns      │ 1000    │ 1000000
   ├─ Lines A+B                553.4 ns      │ 668.7 ns      │ 570.8 ns      │ 572.1 ns      │ 1000    │ 1000000
   ╰─ Sample File              1.622 µs      │ 3.32 µs       │ 1.722 µs      │ 1.743 µs      │ 1000    │ 1000000
```

## Results: `brwu` (unvalidated parsers)

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 100 ns
brwu                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 UnvalidatedRawLogline                    │               │               │               │         │
│  ├─ Line A                     119.5 ns      │ 187.4 ns      │ 122.7 ns      │ 126.8 ns      │ 1000    │ 1000000
│  ├─ Line B                     130.8 ns      │ 149.5 ns      │ 134.5 ns      │ 135 ns        │ 1000    │ 1000000
│  ├─ Lines A+B                  244.7 ns      │ 293.5 ns      │ 252.1 ns      │ 252.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  707.5 ns      │ 793.8 ns      │ 734.9 ns      │ 734.6 ns      │ 1000    │ 1000000
├─ 01 UnvalidatedSimpleLogline                 │               │               │               │         │
│  ├─ Line A                     164.3 ns      │ 214.9 ns      │ 171.1 ns      │ 172.2 ns      │ 1000    │ 1000000
│  ├─ Line B                     164.2 ns      │ 224.2 ns      │ 171.9 ns      │ 172.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  318.6 ns      │ 378.3 ns      │ 333.2 ns      │ 334.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  947.3 ns      │ 1.667 µs      │ 1.01 µs       │ 1.013 µs      │ 1000    │ 1000000
├─ 02 UnvalidatedChronoLogline                 │               │               │               │         │
│  ├─ Line A                     279.4 ns      │ 346.7 ns      │ 295.8 ns      │ 295.8 ns      │ 1000    │ 1000000
│  ├─ Line B                     295.1 ns      │ 358.7 ns      │ 305.3 ns      │ 306.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  575 ns        │ 947.9 ns      │ 603 ns        │ 604.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.718 µs      │ 3.785 µs      │ 1.82 µs       │ 1.85 µs       │ 1000    │ 1000000
├─ 03 UnvalidatedTimeLogline                   │               │               │               │         │
│  ├─ Line A                     215.3 ns      │ 277.1 ns      │ 223.7 ns      │ 224.9 ns      │ 1000    │ 1000000
│  ├─ Line B                     216.7 ns      │ 284.4 ns      │ 225.3 ns      │ 226.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B                  415.5 ns      │ 579.4 ns      │ 442.2 ns      │ 442.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File (no comments)  1.277 µs      │ 2.629 µs      │ 1.361 µs      │ 1.37 µs       │ 1000    │ 1000000
╰─ 04 UnvalidatedParquetLogline                │               │               │               │         │
   ├─ Line A                     267 ns        │ 467.6 ns      │ 275.2 ns      │ 279.6 ns      │ 1000    │ 1000000
   ├─ Line B                     261.2 ns      │ 379.8 ns      │ 273.9 ns      │ 275.1 ns      │ 1000    │ 1000000
   ├─ Lines A+B                  517.8 ns      │ 666.1 ns      │ 546.9 ns      │ 548.6 ns      │ 1000    │ 1000000
   ╰─ Sample File (no comments)  1.597 µs      │ 2.101 µs      │ 1.681 µs      │ 1.686 µs      │ 1000    │ 1000000
```

These numbers are synthetic and depend on hardware, toolchain version, and CPU frequency scaling.
