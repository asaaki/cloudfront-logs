#![cfg(any(feature = "chrono", feature = "parquet"))]

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};

fn legacy(date: &str, time: &str) -> Result<NaiveDateTime, &'static str> {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| "date invalid")?;
    let time = NaiveTime::parse_from_str(time, "%H:%M:%S").map_err(|_| "time invalid")?;
    Ok(NaiveDateTime::new(date, time))
}

fn line(date: &str, time: &str) -> String {
    format!(
        "{date}\t{time}\tLAX1\t392\t192.0.2.100\tGET\td.example\t/\t200\t-\tagent\t-\t-\tHit\tid\td.example\thttps\t23\t0.001\t-\tTLSv1.2\tcipher\tHit\tHTTP/2.0\t-\t-\t11040\t0.001\tHit\ttext/html\t78\t-\t-"
    )
}

const CASES: &[(&str, &str)] = &[
    ("2024-02-29", "23:59:59"),
    ("2000-02-29", "00:00:00"),
    ("1900-02-29", "12:00:00"),
    ("0000-01-01", "00:00:00"),
    ("9999-12-31", "23:59:60"),
    ("2016-12-31", "12:34:60"),
    ("2024-2-9", "1:2:3"),
    ("+10000-01-01", "23:59:60"),
    ("-0001-01-01", "00:00:00"),
    ("+262142-12-31", "12:34:56"),
    ("+262143-01-01", "12:34:56"),
    ("2024/02/29", "12:34:56"),
    ("２０２４-02-29", "12:34:56"),
    ("2024-02-29x", "12:34:56"),
    ("2024-02-29", "12:34:56.123"),
    ("2024-02-29", "12:34:56x"),
    ("2024-02-29", "１２:34:56"),
    ("2024-02-29", "24:00:00"),
    ("2024-02-29", "12:60:00"),
    ("2024-02-29", "12:00:61"),
    (" 2024-02-29", " 12:34:56"),
    ("2024-02-29 ", "12:34:56 "),
    ("invalid", "invalid"),
];

#[test]
fn characterize_chrono_compatibility() {
    assert_eq!(
        legacy("2024-2-9", "1:2:3").unwrap().to_string(),
        "2024-02-09 01:02:03"
    );
    assert!(legacy("+10000-01-01", "23:59:60").is_ok());
    let leap = legacy("2016-12-31", "12:34:60").unwrap();
    assert_eq!(leap.second(), 59);
    assert_eq!(leap.nanosecond(), 1_000_000_000);
    assert_eq!(legacy("2024-02-29", "12:34:56.123"), Err("time invalid"));
}

#[cfg(feature = "chrono")]
#[test]
fn typed_record_routes_match_original_parser() {
    use cloudfront_logs::{borrowed, referential};
    for &(date, time) in CASES {
        let line = line(date, time);
        let expected = legacy(date, time);
        assert_eq!(
            borrowed::ValidatedTypedLogline::try_from(line.as_str()).map(|r| r.datetime()),
            expected,
            "{date} {time}"
        );
        assert_eq!(
            borrowed::UnvalidatedTypedLogline::try_from(line.as_str()).map(|r| r.datetime()),
            expected
        );
        let raw = borrowed::ValidatedRawLogline::try_from(line.as_str()).unwrap();
        assert_eq!(
            borrowed::ValidatedTypedLogline::try_from(raw).map(|r| r.datetime()),
            expected
        );
        assert_eq!(
            referential::OwningValidatedTypedLogline::try_from(line.as_str())
                .map(|r| r.view().datetime()),
            expected
        );
    }
}

#[cfg(feature = "parquet")]
#[test]
fn parquet_record_routes_match_original_parser() {
    use cloudfront_logs::{borrowed, owned, referential};
    for &(date, time) in CASES {
        let line = line(date, time);
        let expected = legacy(date, time);
        assert_eq!(
            borrowed::ValidatedParquetLogline::try_from(line.as_str()).map(|r| r.datetime),
            expected,
            "{date} {time}"
        );
        assert_eq!(
            borrowed::UnvalidatedParquetLogline::try_from(line.as_str()).map(|r| r.datetime),
            expected
        );
        assert_eq!(
            owned::ValidatedParquetLogline::try_from(line.as_str()).map(|r| r.datetime),
            expected
        );
        assert_eq!(
            owned::UnvalidatedParquetLogline::try_from(line.as_str()).map(|r| r.datetime),
            expected
        );
        let raw = || borrowed::ValidatedRawLogline::try_from(line.as_str()).unwrap();
        let raw_unvalidated = || borrowed::UnvalidatedRawLogline::from(line.as_str());
        assert_eq!(
            borrowed::ValidatedParquetLogline::try_from(raw()).map(|r| r.datetime),
            expected
        );
        assert_eq!(
            borrowed::UnvalidatedParquetLogline::try_from(raw()).map(|r| r.datetime),
            expected
        );
        assert_eq!(
            borrowed::UnvalidatedParquetLogline::try_from(raw_unvalidated()).map(|r| r.datetime),
            expected
        );
        assert_eq!(
            owned::ValidatedParquetLogline::try_from(raw()).map(|r| r.datetime),
            expected
        );
        assert_eq!(
            owned::UnvalidatedParquetLogline::try_from(raw()).map(|r| r.datetime),
            expected
        );
        assert_eq!(
            owned::UnvalidatedParquetLogline::try_from(raw_unvalidated()).map(|r| r.datetime),
            expected
        );
        // Malformed referential construction is covered by the separate error-handling fix.
        if expected.is_ok() {
            assert_eq!(
                referential::ValidatedParquetLogline::try_from(line.as_str())
                    .map(|r| r.view().datetime),
                expected
            );
            assert_eq!(
                referential::UnvalidatedParquetLogline::try_from(line.as_str())
                    .map(|r| r.view().datetime),
                expected
            );
        }
    }
}
