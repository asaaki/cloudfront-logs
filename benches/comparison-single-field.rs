mod utilities;
use utilities::*;

fn main() {
    println!("*** Comparing different parsers for AWS CloudFront logs ***\n");
    println!("Parses lines and extracts a single field.");
    divan::main();
}

#[divan::bench(name = "00 CheckedRawLogLine", args = ARGS)]
fn raw_line_checked(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        CheckedRawLogLine::try_from(line).ok().map(|item| {
            let result = &[Data::S0(item.sc_bytes)];
            result.len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}

#[divan::bench(name = "01 UnsafeRawLogLine", args = ARGS)]
fn raw_line_unsafe(inputs: Inputs) -> usize {
    fn parse(line: &str) -> Option<usize> {
        UnsafeRawLogLine::try_from(line).ok().map(|item| {
            let result = &[Data::S0(item.sc_bytes)];
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
            let result = &[Data::S0(item.sc_bytes())];
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
            let result = &[Data::S0(item.sc_bytes())];
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
            let result = &[Data::N(item.sc_bytes)];
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
            let result = &[Data::N(item.sc_bytes)];
            result.len()
        })
    }

    inputs
        .data()
        .iter()
        .map(|line| parse(divan::black_box(*line)).unwrap_or_default())
        .sum()
}
