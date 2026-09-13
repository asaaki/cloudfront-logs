//! Focused date/time experiment; run with --no-default-features --features chrono,parquet.
#![forbid(unsafe_code)]

#[cfg(feature = "chrono")]
mod experiment {
    use chrono::{NaiveDate, NaiveTime};
    use cloudfront_logs::{
        Validated,
        borrowed::structured::{DateTimeBackend, Logline, Selected},
    };
    use std::{hint::black_box, time::Instant};

    struct Legacy;
    impl DateTimeBackend for Legacy {
        type Date<'a> = NaiveDate;
        type Time<'a> = NaiveTime;
        fn parse<'a>(date: &'a str, time: &'a str) -> Result<(NaiveDate, NaiveTime), &'static str> {
            Ok((
                NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| "date invalid")?,
                NaiveTime::parse_from_str(time, "%H:%M:%S").map_err(|_| "time invalid")?,
            ))
        }
    }

    fn corpus(kind: &str) -> Vec<(String, String, String)> {
        let fallback = match kind {
            "exact" => false,
            "fallback" | "pure-fallback" => true,
            _ => panic!("corpus must be exact, fallback, or pure-fallback"),
        };
        (0..4096)
            .map(|i| {
                let year = 1970 + i % 130;
                let month = 1 + i * 7 % 12;
                let day = 1 + i * 13 % 28;
                // A single unpadded hour digit prevents every pure-fallback
                // time from matching the exact eight-byte fast path.
                let hour = i * 7 % if kind == "pure-fallback" { 10 } else { 24 };
                let minute = i * 17 % 60;
                let second = i * 23 % 60;
                let date = if fallback { format!("+{year:05}-{month}-{day}") } else { format!("{year:04}-{month:02}-{day:02}") };
                let time = if fallback { format!("{hour}:{minute}:{second}") } else { format!("{hour:02}:{minute:02}:{second:02}") };
                let line = format!("{date}\t{time}\tLAX1\t392\t192.0.2.100\tGET\td.example\t/index.html\t200\t-\tagent\t-\t-\tHit\tid\td.example\thttps\t23\t0.001\t-\tTLSv1.2\tcipher\tHit\tHTTP/2.0\t-\t-\t11040\t0.001\tHit\ttext/html\t78\t-\t-");
                (date, time, line)
            })
            .collect()
    }

    pub fn run() {
        let args: Vec<String> = std::env::args().collect();
        let mode = args.get(1).map(String::as_str).unwrap_or("typed");
        let kind = args.get(2).map(String::as_str).unwrap_or("exact");
        let rounds: usize = args.get(3).map(|arg| arg.parse().unwrap()).unwrap_or(1000);
        let corpus = corpus(kind);
        let batch = || {
            for (date, time, line) in &corpus {
                match mode {
                    "helper" => {
                        black_box(Selected::parse(black_box(date), black_box(time)).unwrap());
                    }
                    "legacy-helper" => {
                        black_box(Legacy::parse(black_box(date), black_box(time)).unwrap());
                    }
                    "typed" => {
                        let _ = black_box(
                            Logline::<Validated, Selected>::try_from(black_box(line.as_str()))
                                .unwrap(),
                        );
                    }
                    "legacy-typed" => {
                        let _ = black_box(
                            Logline::<Validated, Legacy>::try_from(black_box(line.as_str()))
                                .unwrap(),
                        );
                    }
                    #[cfg(feature = "parquet")]
                    "parquet" => {
                        let _ = black_box(
                            cloudfront_logs::ValidatedParquetLogline::try_from(black_box(
                                line.as_str(),
                            ))
                            .unwrap(),
                        );
                    }
                    _ => panic!(
                        "mode must be helper, legacy-helper, typed, legacy-typed, or parquet"
                    ),
                }
            }
        };
        for _ in 0..20 {
            batch();
        }
        let start = Instant::now();
        for _ in 0..rounds {
            batch();
        }
        let elapsed = start.elapsed();
        println!(
            "{mode},{kind},{rounds},{:.3}",
            elapsed.as_nanos() as f64 / (rounds * corpus.len()) as f64
        );
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn original_fallback_corpus_preserves_its_mixed_time_formats() {
            let corpus = corpus("fallback");
            assert_eq!(corpus.len(), 4096);
            assert!(corpus.iter().all(|(date, _, _)| date.starts_with('+')));
            assert_eq!(
                corpus.iter().filter(|(_, time, _)| time.len() == 8).count(),
                1605
            );
        }

        #[test]
        fn pure_fallback_always_misses_both_exact_format_paths() {
            let corpus = corpus("pure-fallback");
            assert_eq!(corpus.len(), 4096);
            for (date, time, line) in corpus {
                assert!(date.starts_with('+'), "date {date:?}");
                assert!(time.len() < 8, "time {time:?}");
                let expected = Legacy::parse(&date, &time).unwrap();
                assert_eq!(Selected::parse(&date, &time).unwrap(), expected);
                let record = Logline::<Validated, Selected>::try_from(line.as_str()).unwrap();
                assert_eq!((record.date, record.time), expected);
                #[cfg(feature = "parquet")]
                {
                    let record =
                        cloudfront_logs::ValidatedParquetLogline::try_from(line.as_str()).unwrap();
                    assert_eq!(
                        record.datetime,
                        chrono::NaiveDateTime::new(expected.0, expected.1)
                    );
                }
            }
        }
    }
}

fn main() {
    #[cfg(feature = "chrono")]
    experiment::run();
    #[cfg(not(feature = "chrono"))]
    eprintln!("enable --no-default-features --features chrono (and optionally parquet)");
}
