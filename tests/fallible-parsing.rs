use cloudfront_logs::{borrowed, referential, validate_line};

const LINE: &str = "2019-12-04\t21:02:31\tLAX1\t392\t192.0.2.100\tGET\td.example\t/\t200\t-\tagent\t-\t-\tHit\tid\td.example\thttps\t23\t0.001\t-\tTLSv1.2\tcipher\tHit\tHTTP/2.0\t-\t-\t11040\t0.001\tHit\ttext/html\t78\t-\t-";

fn replace_field(index: usize, value: &str) -> String {
    let mut fields: Vec<_> = LINE.split('\t').collect();
    fields[index] = value;
    fields.join("\t")
}

macro_rules! check_error {
    ($parser:path, $line:expr, $error:expr) => {
        assert_eq!($parser($line).unwrap_err(), $error);
    };
}

macro_rules! referential_error {
    ($parser:path, $line:expr, $error:expr) => {
        check_error!($parser, $line, $error);
        check_error!($parser, $line.to_owned(), $error);
        check_error!($parser, $line.to_owned().into_boxed_str(), $error);
        check_error!($parser, std::sync::Arc::<str>::from($line), $error);
    };
}

fn simple_error(line: &str, error: &str) {
    check_error!(borrowed::ValidatedSimpleLogline::try_from, line, error);
    referential_error!(referential::ValidatedSimpleLogline::try_from, line, error);
    check_error!(
        cloudfront_logs::owned::ValidatedSimpleLogline::try_from,
        line,
        error
    );
    check_error!(
        cloudfront_logs::owned::ValidatedSimpleLogline::try_from_with_raw,
        line,
        error
    );
    #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
    {
        check_error!(borrowed::ValidatedTypedLogline::try_from, line, error);
        referential_error!(referential::ValidatedTypedLogline::try_from, line, error);
    }
    // Structural errors belong to validated input constructors. On complete records,
    // also exercise semantic parsing after raw construction and validation conversion.
    if let Ok(raw) = borrowed::ValidatedRawLogline::try_from(line) {
        check_error!(borrowed::ValidatedSimpleLogline::try_from, raw, error);
        check_error!(
            cloudfront_logs::owned::ValidatedSimpleLogline::try_from,
            raw,
            error
        );
        let unvalidated = borrowed::UnvalidatedRawLogline::from(raw);
        check_error!(
            borrowed::UnvalidatedSimpleLogline::try_from,
            unvalidated,
            error
        );
        check_error!(
            cloudfront_logs::owned::UnvalidatedSimpleLogline::try_from,
            unvalidated,
            error
        );
        #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
        {
            check_error!(borrowed::ValidatedTypedLogline::try_from, raw, error);
            check_error!(
                borrowed::UnvalidatedTypedLogline::try_from,
                unvalidated,
                error
            );
        }
    }
}

#[test]
fn raw_preserves_all_empty_fields() {
    for index in 0..33 {
        let line = replace_field(index, "");
        assert!(validate_line(&line).is_ok());
        let parsed = borrowed::ValidatedRawLogline::try_from(line.as_str()).unwrap();
        let retained = referential::ValidatedRawLogline::try_from(line.as_str()).unwrap();
        assert_eq!(retained.view(), &parsed.into());
        if index == 0 {
            assert_eq!(parsed.date, "");
        }
        if index == 32 {
            assert_eq!(parsed.sc_range_end, "");
        }
    }
    let line = "\t".repeat(32);
    let parsed = borrowed::ValidatedRawLogline::try_from(line.as_str()).unwrap();
    assert_eq!(parsed.date, "");
    assert_eq!(parsed.sc_range_end, "");
}

#[test]
fn simple_preserves_empty_text_and_optional_text() {
    for index in [0, 1, 2, 5, 6, 7, 9, 10, 11, 12, 14, 15, 21, 24, 29] {
        let line = replace_field(index, "");
        assert!(borrowed::ValidatedSimpleLogline::try_from(line.as_str()).is_ok());
        assert!(referential::ValidatedSimpleLogline::try_from(line.as_str()).is_ok());
        assert!(cloudfront_logs::owned::ValidatedSimpleLogline::try_from(line.as_str()).is_ok());
    }
    let line = replace_field(9, "");
    assert_eq!(
        borrowed::ValidatedSimpleLogline::try_from(line.as_str())
            .unwrap()
            .cs_referer,
        Some("")
    );
}

#[test]
fn trailing_empty_numeric_field_is_an_error() {
    simple_error(&replace_field(32, ""), "sc_range_end invalid");
}

#[test]
fn invalid_numeric_fields_return_field_errors() {
    for (index, field) in [
        (3, "sc_bytes"),
        (8, "sc_status"),
        (17, "cs_bytes"),
        (25, "fle_encrypted_fields"),
        (26, "c_port"),
        (30, "sc_content_len"),
        (31, "sc_range_start"),
        (32, "sc_range_end"),
    ] {
        for value in ["", "invalid", "184467440737095516160"] {
            simple_error(&replace_field(index, value), &format!("{field} invalid"));
        }
    }
}

#[test]
fn invalid_durations_return_field_errors() {
    for (index, field) in [(18, "time_taken"), (27, "time_to_first_byte")] {
        for value in [
            "",
            "invalid",
            "-1",
            "NaN",
            "inf",
            "-inf",
            "1e300",
            "1e999",
            "18446744073709551616",
        ] {
            simple_error(&replace_field(index, value), &format!("{field} invalid"));
        }
    }
}

#[test]
fn raw_mutation_and_validation_state_conversion_still_check_durations() {
    for value in ["-1", "NaN", "inf", "1e300"] {
        let mut raw = borrowed::ValidatedRawLogline::try_from(LINE).unwrap();
        raw.time_taken = value;
        check_error!(
            borrowed::ValidatedSimpleLogline::try_from,
            raw,
            "time_taken invalid"
        );
        check_error!(
            cloudfront_logs::owned::ValidatedSimpleLogline::try_from,
            raw,
            "time_taken invalid"
        );
        #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
        check_error!(
            borrowed::ValidatedTypedLogline::try_from,
            raw,
            "time_taken invalid"
        );

        let mut raw = borrowed::UnvalidatedRawLogline::from(LINE);
        raw.time_to_first_byte = value;
        let raw: borrowed::ValidatedRawLogline<'_> = raw.into();
        check_error!(
            borrowed::ValidatedSimpleLogline::try_from,
            raw,
            "time_to_first_byte invalid"
        );
        check_error!(
            cloudfront_logs::owned::ValidatedSimpleLogline::try_from,
            raw,
            "time_to_first_byte invalid"
        );
    }
}

#[test]
fn wrong_field_counts_are_rejected_before_parsing() {
    for line in [
        LINE.rsplit_once('\t').unwrap().0.to_owned(),
        format!("{LINE}\textra"),
    ] {
        simple_error(&line, "Invalid log line (field count)");
        check_error!(
            borrowed::ValidatedRawLogline::try_from,
            line.as_str(),
            "Invalid log line (field count)"
        );
        check_error!(
            referential::ValidatedRawLogline::try_from,
            line.as_str(),
            "Invalid log line (field count)"
        );
    }
    simple_error("", "Invalid log line (empty)");
    simple_error("#Version: 1.0", "Invalid log line (comment)");
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
#[test]
fn invalid_dates_and_times_return_field_errors() {
    for (index, error, values) in [
        (0, "date invalid", ["", "2023-02-29", "invalid"]),
        (1, "time invalid", ["", "25:00:00", "invalid"]),
    ] {
        for value in values {
            let line = replace_field(index, value);
            check_error!(
                borrowed::ValidatedTypedLogline::try_from,
                line.as_str(),
                error
            );
            check_error!(
                referential::ValidatedTypedLogline::try_from,
                line.as_str(),
                error
            );
        }
    }
}

#[cfg(feature = "parquet")]
#[test]
fn referential_parquet_invalid_numeric_returns_error() {
    let line = replace_field(3, "invalid");
    check_error!(
        referential::ValidatedParquetLogline::try_from,
        line.as_str(),
        "sc_bytes invalid"
    );
}

#[test]
fn raw_trailing_empty_field_is_preserved() {
    let line = replace_field(32, "");
    assert_eq!(
        borrowed::ValidatedRawLogline::try_from(line.as_str())
            .unwrap()
            .sc_range_end,
        ""
    );
}

#[cfg(feature = "parquet")]
#[test]
fn parquet_construction_propagates_errors() {
    for (index, value, error) in [
        (0, "", "date invalid"),
        (0, "2023-02-29", "date invalid"),
        (1, "25:00:00", "time invalid"),
        (3, "invalid", "sc_bytes invalid"),
        (18, "invalid", "time_taken invalid"),
        (27, "invalid", "time_to_first_byte invalid"),
        (32, "", "sc_range_end invalid"),
    ] {
        let line = replace_field(index, value);
        check_error!(
            borrowed::ValidatedParquetLogline::try_from,
            line.as_str(),
            error
        );
        check_error!(
            cloudfront_logs::owned::ValidatedParquetLogline::try_from,
            line.as_str(),
            error
        );
        referential_error!(
            referential::ValidatedParquetLogline::try_from,
            line.as_str(),
            error
        );
        referential_error!(
            referential::UnvalidatedParquetLogline::try_from,
            line.as_str(),
            error
        );
    }
}

#[test]
fn valid_duration_rounding_and_negative_zero_are_preserved() {
    use std::time::Duration;
    for (value, expected) in [
        ("-0", Duration::ZERO),
        ("0.001", Duration::from_millis(1)),
        ("0.0000000001", Duration::ZERO),
        ("0.0000000009", Duration::from_nanos(1)),
        ("0.0000000019", Duration::from_nanos(2)),
        ("1.25", Duration::from_millis(1250)),
        ("1e3", Duration::from_secs(1000)),
    ] {
        let line = replace_field(18, value);
        assert_eq!(
            borrowed::ValidatedSimpleLogline::try_from(line.as_str())
                .unwrap()
                .time_taken,
            expected
        );
        assert_eq!(
            cloudfront_logs::owned::ValidatedSimpleLogline::try_from(line.as_str())
                .unwrap()
                .time_taken,
            expected
        );
        assert_eq!(
            referential::ValidatedSimpleLogline::try_from(line.as_str())
                .unwrap()
                .view()
                .time_taken,
            expected
        );
    }
}

#[cfg(feature = "parquet")]
#[test]
fn parquet_float_durations_keep_existing_semantics() {
    for value in ["-1", "NaN", "inf", "-inf", "1e300", "1e999"] {
        let line = replace_field(18, value);
        let borrowed = borrowed::ValidatedParquetLogline::try_from(line.as_str()).unwrap();
        let owned =
            cloudfront_logs::owned::ValidatedParquetLogline::try_from(line.as_str()).unwrap();
        let retained = referential::ValidatedParquetLogline::try_from(line.as_str()).unwrap();
        if value == "NaN" {
            assert!(borrowed.time_taken.is_nan());
            assert!(owned.time_taken.is_nan());
            assert!(retained.view().time_taken.is_nan());
        } else {
            let expected: f64 = value.parse().unwrap();
            assert_eq!(borrowed.time_taken, expected);
            assert_eq!(owned.time_taken, expected);
            assert_eq!(retained.view().time_taken, expected);
        }
    }
}
