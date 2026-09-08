# Benchmarks

This document tracks the current benchmark targets defined in `Cargo.toml`:

- `brwv` -> `benches/borrowed-real-world-validated.rs`
- `brwu` -> `benches/borrowed-real-world-unvalidated.rs`

## Commands

```powershell
$env:RUSTFLAGS='-Ctarget-cpu=native'
cargo bench -q --no-default-features --features chrono --bench brwv
cargo bench -q --no-default-features --features chrono --bench brwu
```

The validated and unvalidated suites each include one raw, one simple, and one typed benchmark.
Parquet benchmarks are included when the `parquet` feature is enabled.

Current timing tables are intentionally omitted. Benchmark results depend on hardware, toolchain
version, selected date/time backend, and CPU frequency scaling.
