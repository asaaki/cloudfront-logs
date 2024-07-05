#![allow(non_snake_case)]

mod utilities;
use utilities::*;

fn main() {
    println!("*** Comparing different parsers for AWS CloudFront logs ***\n");
    println!("Parses lines and extracts a few fields, slightly unordered,");
    println!("this should simulate close to real-world usages.");
    divan::main();
}

#[divan::bench(name = "00 ValidatedRawLogline", args = ARGS)]
fn ValidatedRawLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        ValidatedRawLogline::try_from(line).ok().map(|item| {
            let result = &[
                Data::S0(item.date),
                Data::S0(item.time),
                Data::S0(item.c_ip),
                Data::S0(item.c_port),
                Data::S0(item.cs_uri_stem),
                Data::S0(item.sc_content_len),
                Data::S0(item.sc_bytes),
            ];
            result.len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}

#[divan::bench(name = "01 ValidatedSimpleLogline", args = ARGS)]
fn ValidatedSimpleLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        ValidatedSimpleLogline::try_from(line).ok().map(|item| {
            let result = &[
                Data::S0(item.date),
                Data::S0(item.time),
                Data::I(item.c_ip),
                Data::M(item.c_port),
                Data::S0(item.cs_uri_stem),
                Data::ON(item.sc_content_len),
                Data::N(item.sc_bytes),
            ];
            result.len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}

#[divan::bench(name = "02 ValidatedChronoLogline", args = ARGS)]
fn ValidatedChronoLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        ValidatedChronoLogline::try_from(line).ok().map(|item| {
            let result = &[
                Data::ND(item.date),
                Data::NT(item.time),
                Data::I(item.c_ip),
                Data::M(item.c_port),
                Data::S0(item.cs_uri_stem),
                Data::ON(item.sc_content_len),
                Data::N(item.sc_bytes),
            ];
            result.len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}

#[divan::bench(name = "03 ValidatedTimeLogline", args = ARGS)]
fn ValidatedTimeLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        ValidatedTimeLogline::try_from(line).ok().map(|item| {
            let result = &[
                Data::D(item.date),
                Data::T(item.time),
                Data::I(item.c_ip),
                Data::M(item.c_port),
                Data::S0(item.cs_uri_stem),
                Data::ON(item.sc_content_len),
                Data::N(item.sc_bytes),
            ];
            result.len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}

#[divan::bench(name = "04 ValidatedParquetLogline", args = ARGS)]
fn ValidatedParquetLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        ValidatedParquetLogline::try_from(line).ok().map(|item| {
            let result = &[
                Data::ND(item.date),
                Data::S0(item.time),
                Data::S0(item.c_ip),
                Data::M(item.c_port),
                Data::S0(item.cs_uri_stem),
                Data::ON(item.sc_content_len),
                Data::N(item.sc_bytes),
            ];
            result.len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}
