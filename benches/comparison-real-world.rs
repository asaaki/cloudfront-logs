mod utilities;
use utilities::*;

fn main() {
    println!("*** Comparing different parsers for AWS CloudFront logs ***\n");
    println!("Parses lines and extracts a few fields, slightly unordered,");
    println!("this should simulate close to real-world usages.");
    divan::main();
}

#[divan::bench(name = "00 CheckedRawLogLine", args = ARGS)]
fn raw_line_checked(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        CheckedRawLogLine::try_from(line).ok().map(|item| {
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

#[divan::bench(name = "10 CheckedRawLogLineView", args = ARGS)]
fn raw_view_checked(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        CheckedRawLogLineView::new(line).ok().map(|item| {
            let result = &[
                Data::S0(item.date()),
                Data::S0(item.time()),
                Data::S0(item.c_ip()),
                Data::S0(item.c_port()),
                Data::S0(item.cs_uri_stem()),
                Data::S0(item.sc_content_len()),
                Data::S0(item.sc_bytes()),
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

#[divan::bench(name = "11 SmartRawLogLineView", args = ARGS)]
fn raw_view_smart(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        SmartRawLogLineView::new(line).ok().map(|item| {
            let result = &[
                Data::S0(item.date()),
                Data::S0(item.time()),
                Data::S0(item.c_ip()),
                Data::S0(item.c_port()),
                Data::S0(item.cs_uri_stem()),
                Data::S0(item.sc_content_len()),
                Data::S0(item.sc_bytes()),
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

#[divan::bench(name = "20 SimpleLogLine", args = ARGS)]
fn owned_simple(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        SimpleLogLine::try_from(line).ok().map(|item| {
            let result = &[
                Data::S(item.date),
                Data::S(item.time),
                Data::I(item.c_ip),
                Data::M(item.c_port),
                Data::S(item.cs_uri_stem),
                Data::N(item.sc_content_len),
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

#[divan::bench(name = "21 TypedLogLine", args = ARGS)]
fn owned_typed(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        TypedLogLine::try_from(line).ok().map(|item| {
            let result = &[
                Data::D(item.date),
                Data::T(item.time),
                Data::I(item.c_ip),
                Data::M(item.c_port),
                Data::S(item.cs_uri_stem),
                Data::N(item.sc_content_len),
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
