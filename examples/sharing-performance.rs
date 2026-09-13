//! Compare reparsing clones with sharing complete immutable referential records.
//! Set SHARING_PROFILE=mixed|ipv4|forwarded|unknown|long; default: mixed.
//! Use --sample-count/--sample-size to override repository benchmark defaults.
#![forbid(unsafe_code)]

#[allow(dead_code)]
#[path = "../benches/corpus.rs"]
mod corpus;

use cloudfront_logs::referential;
use std::{cell::Cell, hint::black_box, sync::Arc};

#[cfg(feature = "bench-alloc")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

trait Record: Clone {
    fn parse(line: &str) -> Self;
    fn observe(&self);
}

macro_rules! record {
    ($type:ty) => {
        impl Record for $type {
            fn parse(line: &str) -> Self {
                Self::try_from(line).unwrap()
            }
            fn observe(&self) {
                black_box(self.view());
            }
        }
    };
}

record!(referential::ValidatedRawLogline);
record!(referential::ValidatedSimpleLogline);
#[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
record!(referential::ValidatedTypedLogline);
#[cfg(feature = "parquet")]
record!(referential::ValidatedParquetLogline);

impl<R: Record> Record for Arc<R> {
    fn parse(line: &str) -> Self {
        Arc::new(R::parse(line))
    }
    fn observe(&self) {
        self.as_ref().observe();
    }
}

fn inputs() -> corpus::Corpus {
    use corpus::Profile;
    let profile = match std::env::var("SHARING_PROFILE")
        .as_deref()
        .unwrap_or("mixed")
    {
        "mixed" => Profile::Mixed,
        "ipv4" => Profile::Ipv4,
        "forwarded" => Profile::Forwarded,
        "unknown" => Profile::UnknownResults,
        "long" => Profile::LongFields,
        _ => panic!("SHARING_PROFILE must be mixed, ipv4, forwarded, unknown, or long"),
    };
    corpus::generate(profile, 1024)
}

fn next<'a, T>(items: &'a [T], cursor: &Cell<usize>) -> &'a T {
    let index = cursor.get();
    cursor.set((index + 1) % items.len());
    &items[index]
}

fn bundle<R: Record, const N: usize>(line: &str) -> (R, [R; N]) {
    let original = R::parse(line);
    let copies = std::array::from_fn(|_| original.clone());
    (original, copies)
}

fn lifecycle<R: Record, const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let inputs = inputs();
    let cursor = Cell::new(0);
    bencher
        .with_inputs(|| next(&inputs.lines, &cursor).as_str())
        .bench_local_values(|line| {
            let (original, copies) = bundle::<R, N>(black_box(line));
            original.observe();
            // Every copy stays alive simultaneously; drop the original first.
            black_box(&copies);
            drop(original);
            drop(copies);
        });
}

fn clone_only<R: Record, const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let inputs = inputs();
    let records: Vec<R> = inputs.lines.iter().map(|line| R::parse(line)).collect();
    let cursor = Cell::new(0);
    bencher
        .with_inputs(|| next(&records, &cursor))
        .bench_local_values(|original| {
            let copies: [R; N] = std::array::from_fn(|_| original.clone());
            black_box(&copies);
            drop(copies);
        });
}

fn destruction<R: Record, const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let inputs = inputs();
    let cursor = Cell::new(0);
    bencher
        .with_inputs(|| bundle::<R, N>(next(&inputs.lines, &cursor)))
        .bench_local_values(|(original, copies)| {
            black_box(&copies);
            drop(original);
            drop(copies);
        });
}

macro_rules! workloads {
    ($module:ident, $record:ty) => {
        mod $module {
            use super::*;

            #[divan::bench(consts = [0, 1, 8, 32])]
            fn lifecycle_direct<const N: usize>(bencher: divan::Bencher<'_, '_>) {
                lifecycle::<$record, N>(bencher);
            }
            #[divan::bench(consts = [0, 1, 8, 32])]
            fn lifecycle_shared<const N: usize>(bencher: divan::Bencher<'_, '_>) {
                lifecycle::<Arc<$record>, N>(bencher);
            }
            #[divan::bench(consts = [1, 8, 32])]
            fn clone_direct<const N: usize>(bencher: divan::Bencher<'_, '_>) {
                clone_only::<$record, N>(bencher);
            }
            #[divan::bench(consts = [1, 8, 32])]
            fn clone_shared<const N: usize>(bencher: divan::Bencher<'_, '_>) {
                clone_only::<Arc<$record>, N>(bencher);
            }
            #[divan::bench(consts = [0, 1, 8, 32])]
            fn destruction_direct<const N: usize>(bencher: divan::Bencher<'_, '_>) {
                destruction::<$record, N>(bencher);
            }
            #[divan::bench(consts = [0, 1, 8, 32])]
            fn destruction_shared<const N: usize>(bencher: divan::Bencher<'_, '_>) {
                destruction::<Arc<$record>, N>(bencher);
            }
        }
    };
}

workloads!(raw, referential::ValidatedRawLogline);
workloads!(simple, referential::ValidatedSimpleLogline);
#[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
workloads!(typed, referential::ValidatedTypedLogline);
#[cfg(feature = "parquet")]
workloads!(parquet, referential::ValidatedParquetLogline);

fn main() {
    println!(
        "Sharing profile: {} (1024 varied records); timings per original record, copies held simultaneously",
        std::env::var("SHARING_PROFILE")
            .as_deref()
            .unwrap_or("mixed")
    );
    divan::main();
}
