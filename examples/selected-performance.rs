//! Synthetic CPU-only comparison; no file I/O or corpus construction is timed.
//! Run: cargo run --release --no-default-features --example selected-performance -- 10000 5 100
//! Arguments: records (multiple of 100), samples, minimum milliseconds per sample.
#[allow(dead_code)]
#[path = "../benches/corpus.rs"]
mod corpus;

use cloudfront_logs::borrowed::{ValidatedRawLogline, ValidatedSimpleLogline};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
enum Method {
    Selected,
    Manual,
    Eager,
}

#[derive(Clone, Copy, Debug)]
enum Workload {
    Bytes,
    BytesDuration,
    RetainedFull,
}

#[derive(Debug, Default, PartialEq)]
struct Totals {
    retained: u64,
    bytes: u128,
    nanoseconds: u128,
}

fn consume(
    lines: &[String],
    rate: u16,
    method: Method,
    workload: Workload,
) -> Result<Totals, &'static str> {
    let mut totals = Totals::default();
    for line in lines {
        let line = black_box(line.as_str());
        let (bytes, duration) = match method {
            Method::Eager => {
                let record = ValidatedSimpleLogline::try_from(line)?;
                // Observe the complete eager record, including rejected records.
                black_box(&record);
                if record.sc_status >= 200 + rate {
                    continue;
                }
                (record.sc_bytes, record.time_taken)
            }
            Method::Selected | Method::Manual => {
                let raw = ValidatedRawLogline::try_from(line)?;
                let status = match method {
                    Method::Selected => raw.parse_sc_status()?,
                    Method::Manual => raw
                        .sc_status
                        .parse::<u16>()
                        .map_err(|_error| "sc_status invalid")?,
                    Method::Eager => unreachable!(),
                };
                if status >= 200 + rate {
                    continue;
                }
                if matches!(workload, Workload::RetainedFull) {
                    let record = ValidatedSimpleLogline::try_from(raw)?;
                    black_box(&record);
                    (record.sc_bytes, record.time_taken)
                } else {
                    let bytes = match method {
                        Method::Selected => raw.parse_sc_bytes()?,
                        Method::Manual => raw
                            .sc_bytes
                            .parse::<u64>()
                            .map_err(|_error| "sc_bytes invalid")?,
                        Method::Eager => unreachable!(),
                    };
                    let duration = if matches!(workload, Workload::BytesDuration) {
                        match method {
                            Method::Selected => raw.parse_time_taken()?,
                            Method::Manual => {
                                let seconds = raw
                                    .time_taken
                                    .parse::<f64>()
                                    .map_err(|_error| "time_taken invalid")?;
                                Duration::try_from_secs_f64(seconds)
                                    .map_err(|_error| "time_taken invalid")?
                            }
                            Method::Eager => unreachable!(),
                        }
                    } else {
                        Duration::ZERO
                    };
                    (bytes, duration)
                }
            }
        };
        totals.retained += 1;
        totals.bytes += u128::from(bytes);
        if !matches!(workload, Workload::Bytes) {
            totals.nanoseconds += duration.as_nanos();
        }
    }
    Ok(totals)
}

fn measure(
    lines: &[String],
    rate: u16,
    method: Method,
    workload: Workload,
    minimum: Duration,
) -> Result<f64, &'static str> {
    let started = Instant::now();
    let mut passes = 0u64;
    loop {
        black_box(consume(
            black_box(lines),
            black_box(rate),
            method,
            workload,
        )?);
        passes += 1;
        if started.elapsed() >= minimum {
            break;
        }
    }
    Ok(started.elapsed().as_secs_f64() * 1e9 / (passes as f64 * lines.len() as f64))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let records: usize = args.get(1).map_or(Ok(10000), |arg| arg.parse())?;
    let samples: usize = args.get(2).map_or(Ok(5), |arg| arg.parse())?;
    let milliseconds: u64 = args.get(3).map_or(Ok(100), |arg| arg.parse())?;
    if records == 0 || !records.is_multiple_of(100) || samples == 0 || milliseconds == 0 {
        return Err(
            "records must be a positive multiple of 100; samples and milliseconds must be positive"
                .into(),
        );
    }
    let corpus = corpus::generate(corpus::Profile::Mixed, records);
    println!(
        "records={records}, bytes={}, samples={samples}, minimum_ms={milliseconds}",
        corpus.bytes
    );
    println!(
        "Mixed synthetic corpus; parsing, output consumption, and destruction included; generation excluded."
    );
    println!("Order rotates Selected/Manual/Eager by sample; retention order=0,1,10,50,100.");
    println!(
        "workload,retention_percent,method,median_ns_per_record,min_ns_per_record,max_ns_per_record,records_per_second,bytes_per_second"
    );
    let methods = [Method::Selected, Method::Manual, Method::Eager];
    for workload in [
        Workload::Bytes,
        Workload::BytesDuration,
        Workload::RetainedFull,
    ] {
        for rate in [0, 1, 10, 50, 100] {
            // Compare observable results and warm every path before timing.
            let expected = consume(&corpus.lines, rate, Method::Eager, workload)?;
            assert_eq!(
                expected.retained as usize,
                records / 100 * usize::from(rate)
            );
            for method in methods {
                assert_eq!(consume(&corpus.lines, rate, method, workload)?, expected);
            }
            let mut times = [Vec::new(), Vec::new(), Vec::new()];
            for sample in 0..samples {
                for offset in 0..methods.len() {
                    let index = (sample + offset) % methods.len();
                    times[index].push(measure(
                        &corpus.lines,
                        rate,
                        methods[index],
                        workload,
                        Duration::from_millis(milliseconds),
                    )?);
                }
            }
            for (method, mut times) in methods.into_iter().zip(times) {
                times.sort_by(f64::total_cmp);
                let median = times[times.len() / 2];
                let records_per_second = 1e9 / median;
                let bytes_per_second = records_per_second * corpus.bytes as f64 / records as f64;
                println!(
                    "{workload:?},{rate},{method:?},{median:.3},{:.3},{:.3},{records_per_second:.0},{bytes_per_second:.0}",
                    times[0],
                    times[times.len() - 1]
                );
            }
        }
    }
    Ok(())
}
