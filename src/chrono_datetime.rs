use crate::consts::{CHRONO_DATE_FMT, CHRONO_TIME_FMT};
use chrono::{NaiveDate, NaiveTime};

/// Recognize only the fixed ASCII format; retain Chrono's accepted language otherwise.
pub(crate) fn parse_date(input: &str) -> Result<NaiveDate, &'static str> {
    if let [
        a @ b'0'..=b'9',
        b @ b'0'..=b'9',
        c @ b'0'..=b'9',
        d @ b'0'..=b'9',
        b'-',
        e @ b'0'..=b'9',
        f @ b'0'..=b'9',
        b'-',
        g @ b'0'..=b'9',
        h @ b'0'..=b'9',
    ] = input.as_bytes()
    {
        let year = (pair(*a, *b) * 100 + pair(*c, *d)) as i32;
        return NaiveDate::from_ymd_opt(year, pair(*e, *f), pair(*g, *h)).ok_or("date invalid");
    }
    NaiveDate::parse_from_str(input, CHRONO_DATE_FMT).map_err(|_error| "date invalid")
}

pub(crate) fn parse_time(input: &str) -> Result<NaiveTime, &'static str> {
    if let [
        a @ b'0'..=b'9',
        b @ b'0'..=b'9',
        b':',
        c @ b'0'..=b'9',
        d @ b'0'..=b'9',
        b':',
        e @ b'0'..=b'9',
        f @ b'0'..=b'9',
    ] = input.as_bytes()
    {
        let second = pair(*e, *f);
        // Chrono represents :60 as second 59 plus one billion nanoseconds.
        // Keep its parser responsible for leap-second handling.
        if second < 60 {
            return NaiveTime::from_hms_opt(pair(*a, *b), pair(*c, *d), second)
                .ok_or("time invalid");
        }
    }
    NaiveTime::parse_from_str(input, CHRONO_TIME_FMT).map_err(|_error| "time invalid")
}

// Callers match ASCII digits before decoding; arithmetic fits even in a byte.
fn pair(tens: u8, units: u8) -> u32 {
    u32::from(tens - b'0') * 10 + u32::from(units - b'0')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_date(input: &str) {
        assert_eq!(
            parse_date(input),
            NaiveDate::parse_from_str(input, "%Y-%m-%d").map_err(|_error| "date invalid"),
            "date {input:?}"
        );
    }

    fn check_time(input: &str) {
        assert_eq!(
            parse_time(input),
            NaiveTime::parse_from_str(input, "%H:%M:%S").map_err(|_error| "time invalid"),
            "time {input:?}"
        );
    }

    #[test]
    fn matches_calendar_boundaries_and_gregorian_cycle() {
        for year in (1800..=2200).chain([0, 1, 4, 99, 100, 400, 9999]) {
            for month in 0..=13 {
                for day in 0..=32 {
                    check_date(&format!("{year:04}-{month:02}-{day:02}"));
                }
            }
        }
    }

    #[test]
    fn matches_all_clock_values_and_boundaries() {
        for hour in 0..=24 {
            for minute in 0..=60 {
                for second in 0..=61 {
                    check_time(&format!("{hour:02}:{minute:02}:{second:02}"));
                }
            }
        }
    }

    #[test]
    fn matches_fallback_and_malformed_input() {
        for input in [
            "",
            "2024-2-9",
            "+10000-01-01",
            "-0001-01-01",
            "+262142-12-31",
            "+262143-01-01",
            "-262143-01-01",
            "-262144-01-01",
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
            "12:34:56.123",
            "12:34:60.123",
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
}
