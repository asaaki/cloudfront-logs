//! Checked fast paths for the fixed CloudFront formats, with backend fallbacks.

#[cfg(feature = "jiff")]
use jiff::civil::{Date, Time};
#[cfg(feature = "time")]
use time::{Date, Time};

pub(crate) fn parse_date(input: &str) -> Result<Date, &'static str> {
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
        let year = u16::from(pair(*a, *b)) * 100 + u16::from(pair(*c, *d));
        let month = pair(*e, *f);
        let day = pair(*g, *h);
        // Four year digits and two month/day digits fit the signed Jiff inputs.
        #[cfg(feature = "jiff")]
        return Date::new(year as i16, month as i8, day as i8).map_err(|_error| "date invalid");
        #[cfg(feature = "time")]
        return Date::from_calendar_date(
            i32::from(year),
            time::Month::try_from(month).map_err(|_error| "date invalid")?,
            day,
        )
        .map_err(|_error| "date invalid");
    }
    #[cfg(feature = "jiff")]
    return input.parse().map_err(|_error| "date invalid");
    #[cfg(feature = "time")]
    Date::parse(input, crate::consts::TIME_DATE_FMT).map_err(|_error| "date invalid")
}

pub(crate) fn parse_time(input: &str) -> Result<Time, &'static str> {
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
        let hour = pair(*a, *b);
        let minute = pair(*c, *d);
        let second = pair(*e, *f);
        // Keep each backend's leap-second parsing behavior in its original parser.
        if second < 60 {
            #[cfg(feature = "jiff")]
            return Time::new(hour as i8, minute as i8, second as i8, 0)
                .map_err(|_error| "time invalid");
            #[cfg(feature = "time")]
            return Time::from_hms(hour, minute, second).map_err(|_error| "time invalid");
        }
    }
    #[cfg(feature = "jiff")]
    return input.parse().map_err(|_error| "time invalid");
    #[cfg(feature = "time")]
    Time::parse(input, crate::consts::TIME_TIME_FMT).map_err(|_error| "time invalid")
}

// Every caller first matches both bytes as ASCII digits; the result is at most 99.
fn pair(tens: u8, units: u8) -> u8 {
    (tens - b'0') * 10 + units - b'0'
}
