#![cfg(any(feature = "jiff", feature = "time"))]

use cloudfront_logs::borrowed::structured::{DateTimeBackend, Selected};

fn check_date(input: &str) {
    #[cfg(feature = "jiff")]
    let expected = input.parse::<jiff::civil::Date>();
    #[cfg(feature = "time")]
    let expected = time::Date::parse(
        input,
        time::macros::format_description!("[year]-[month]-[day]"),
    );
    assert_eq!(
        Selected::parse(input, "12:34:56").map(|(date, _)| date),
        expected.map_err(|_error| "date invalid"),
        "date {input:?}"
    );
}

fn check_time(input: &str) {
    #[cfg(feature = "jiff")]
    let expected = input.parse::<jiff::civil::Time>();
    #[cfg(feature = "time")]
    let expected = time::Time::parse(
        input,
        time::macros::format_description!("[hour]:[minute]:[second]"),
    );
    assert_eq!(
        Selected::parse("2024-02-29", input).map(|(_, time)| time),
        expected.map_err(|_error| "time invalid"),
        "time {input:?}"
    );
}

#[test]
fn calendar_boundaries_match_backend() {
    for year in (1800..=2200).chain([0, 1, 4, 99, 100, 400, 9999]) {
        for month in 0..=13 {
            for day in 0..=32 {
                check_date(&format!("{year:04}-{month:02}-{day:02}"));
            }
        }
    }
}

#[test]
fn clock_values_and_leap_seconds_match_backend() {
    for hour in 0..=24 {
        for minute in 0..=60 {
            for second in 0..=61 {
                check_time(&format!("{hour:02}:{minute:02}:{second:02}"));
            }
        }
    }
}

#[test]
fn alternate_formats_and_malformed_inputs_match_backend() {
    for input in [
        "",
        "2024-2-9",
        "+10000-01-01",
        "-0001-01-01",
        "-9999-12-31",
        "-000001-01-01",
        "+001970-01-01",
        "-009999-12-31",
        "-010000-01-01",
        "2024-02-29T12:34:56Z",
        "2024-02-29[u-ca=gregory]",
        "20240229",
        " 2024-02-29",
        "2024-02-29 ",
        "2024-02-29x",
        "2024/02/29",
        "２０２４-02-29",
    ] {
        check_date(input);
    }
    for input in [
        "",
        "1:2:3",
        "23:59:60",
        "12:34:60",
        "12:34:56.123456789",
        "12:34:60.123",
        "12:34:56Z",
        "12:34:56+01:00",
        "T12:34:56",
        "123456",
        "2024-02-29T12:34:56Z",
        "12:34:56x",
        " 12:34:56",
        "12:34:56 ",
        "１２:34:56",
    ] {
        check_time(input);
    }
    for (base, check) in [
        ("2024-02-29", check_date as fn(&str)),
        ("12:34:56", check_time as fn(&str)),
    ] {
        for index in 0..base.len() {
            for replacement in (0..=127).map(char::from).chain(['é', '０', '🦀']) {
                let mut input = base.to_owned();
                input.replace_range(index..=index, &replacement.to_string());
                check(&input);
            }
        }
    }
}

#[test]
fn date_errors_precede_time_errors() {
    assert_eq!(
        Selected::parse("2024-02-30", "25:00:00"),
        Err("date invalid")
    );
    assert_eq!(
        Selected::parse("2024-02-29", "25:00:00"),
        Err("time invalid")
    );
}
