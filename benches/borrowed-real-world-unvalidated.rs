#![allow(non_snake_case)]
#![forbid(unsafe_code)]

mod corpus;
mod representative;

#[cfg(feature = "parquet")]
use cloudfront_logs::{
    UnvalidatedParquetLogline as CorpusParquet,
    owned::UnvalidatedParquetLogline as CorpusOwnedParquet,
    referential::UnvalidatedParquetLogline as CorpusReferentialParquet,
};
use cloudfront_logs::{
    UnvalidatedRawLogline as CorpusRaw, UnvalidatedSimpleLogline as CorpusSimple,
    owned::UnvalidatedSimpleLogline as CorpusOwned,
    referential::UnvalidatedRawLogline as CorpusReferentialRaw,
    referential::UnvalidatedSimpleLogline as CorpusReferential,
};
#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
use cloudfront_logs::{
    UnvalidatedTypedLogline as CorpusTyped,
    referential::UnvalidatedTypedLogline as CorpusReferentialTyped,
};

#[cfg(feature = "bench-alloc")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

mod utilities;
use utilities::*;

fn main() {
    println!("*** Comparing different parsers for AWS CloudFront logs ***\n");
    println!("Parses lines and extracts a few fields, slightly unordered,");
    println!("this should simulate close to real-world usages.");
    println!("Methodology v2: extracted values are black-boxed; compare matching input labels.");
    representative::describe();
    divan::main();
}

#[divan::bench(name = "00 UnvalidatedRawLogline", args = ARGS_NO_COMMENTS)]
fn UnvalidatedRawLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        let item = UnvalidatedRawLogline::from(line);
        let result = &[
            Data::S0(item.date),
            Data::S0(item.time),
            Data::S0(item.c_ip),
            Data::S0(item.c_port),
            Data::S0(item.cs_uri_stem),
            Data::S0(item.sc_content_len),
            Data::S0(item.sc_bytes),
        ];
        Some(divan::black_box(result).len())
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}

#[divan::bench(name = "01 UnvalidatedSimpleLogline", args = ARGS_NO_COMMENTS)]
fn UnvalidatedSimpleLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        UnvalidatedSimpleLogline::try_from(line).ok().map(|item| {
            let result = &[
                Data::S0(item.date),
                Data::S0(item.time),
                Data::I(item.c_ip),
                Data::M(item.c_port),
                Data::S0(item.cs_uri_stem),
                Data::ON(item.sc_content_len),
                Data::N(item.sc_bytes),
            ];
            divan::black_box(result).len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
#[divan::bench(name = "02 UnvalidatedTypedLogline", args = ARGS_NO_COMMENTS)]
fn UnvalidatedTypedLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        UnvalidatedTypedLogline::try_from(line).ok().map(|item| {
            let result = &[
                Data::D(item.date),
                Data::T(item.time),
                Data::I(item.c_ip),
                Data::M(item.c_port),
                Data::S0(item.cs_uri_stem),
                Data::ON(item.sc_content_len),
                Data::N(item.sc_bytes),
            ];
            divan::black_box(result).len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}

#[cfg(feature = "parquet")]
#[divan::bench(name = "03 UnvalidatedParquetLogline", args = ARGS_NO_COMMENTS)]
fn UnvalidatedParquetLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        UnvalidatedParquetLogline::try_from(line).ok().map(|item| {
            let result = &[
                Data::PD(item.date),
                Data::S0(item.time),
                Data::S0(item.c_ip),
                Data::M(item.c_port),
                Data::S0(item.cs_uri_stem),
                Data::ON(item.sc_content_len),
                Data::N(item.sc_bytes),
            ];
            divan::black_box(result).len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}
