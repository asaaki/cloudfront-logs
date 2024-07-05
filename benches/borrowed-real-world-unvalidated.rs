#![allow(non_snake_case)]

mod utilities;
use utilities::*;

fn main() {
    println!("*** Comparing different parsers for AWS CloudFront logs ***\n");
    println!("Parses lines and extracts a few fields, slightly unordered,");
    println!("this should simulate close to real-world usages.");
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
        Some(result.len())
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
            result.len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}

#[divan::bench(name = "02 UnvalidatedChronoLogline", args = ARGS_NO_COMMENTS)]
fn UnvalidatedChronoLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        UnvalidatedChronoLogline::try_from(line).ok().map(|item| {
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

#[divan::bench(name = "03 UnvalidatedTimeLogline", args = ARGS_NO_COMMENTS)]
fn UnvalidatedTimeLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        UnvalidatedTimeLogline::try_from(line).ok().map(|item| {
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

#[divan::bench(name = "04 UnvalidatedParquetLogline", args = ARGS_NO_COMMENTS)]
fn UnvalidatedParquetLogline(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        UnvalidatedParquetLogline::try_from(line).ok().map(|item| {
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
