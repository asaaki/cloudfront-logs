//! Filter and aggregate named raw fields; fully parse only retained records.
//! Run: cargo run --example selected-fields -- path/to/cloudfront.log
use cloudfront_logs::borrowed::{ValidatedRawLogline, ValidatedSimpleLogline};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: selected-fields <log-file>")?;
    let mut reader = BufReader::new(File::open(path)?);
    let mut buffer = String::new();
    let mut line_number = 0u64;
    let mut error_responses = 0u64;
    let mut response_bytes = 0u128;
    let mut slow_error_responses = 0u64;

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }
        line_number += 1;
        let line = buffer.trim_end_matches(['\r', '\n']);
        if line.starts_with('#') {
            continue;
        }
        let parse_error = |message| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("line {line_number}: {message}"),
            )
        };
        let raw = ValidatedRawLogline::try_from(line).map_err(parse_error)?;
        if raw.parse_sc_status().map_err(parse_error)? < 400 {
            continue;
        }
        error_responses += 1;
        response_bytes += u128::from(raw.parse_sc_bytes().map_err(parse_error)?);

        // This is the point at which all structured fields are validated.
        // Earlier rejection does not validate the fields that were not observed.
        let record = ValidatedSimpleLogline::try_from(raw).map_err(parse_error)?;
        if record.time_taken.as_secs_f64() >= 1.0 {
            slow_error_responses += 1;
        }
    }
    println!(
        "error responses={error_responses}, response bytes={response_bytes}, errors taking at least one second={slow_error_responses}"
    );
    Ok(())
}
