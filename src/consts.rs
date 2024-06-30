/// CloudFront log format version header; currently only 1.0 is known and supported
///
/// This line is the first line of the log file and is used to identify the version of the log file format.
#[allow(unused)]
pub(crate) const VERSION_COMMENT: &str = "#Version: 1.0";

/// CloudFront log fields header comment
///
/// This line is the second line of the log file and is used to identify the fields in the log file.
///
/// Also check the official documentation for the list of fields and their description:
/// <https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html#LogFileFormat>
#[allow(unused)]
pub(crate) const FIELDS_COMMENT: &str = "#Fields: date time x-edge-location sc-bytes c-ip cs-method cs(Host) cs-uri-stem sc-status cs(Referer) cs(User-Agent) cs-uri-query cs(Cookie) x-edge-result-type x-edge-request-id x-host-header cs-protocol cs-bytes time-taken x-forwarded-for ssl-protocol ssl-cipher x-edge-response-result-type cs-protocol-version fle-status fle-encrypted-fields c-port time-to-first-byte x-edge-detailed-result-type sc-content-type sc-content-len sc-range-start sc-range-end";

/// Comment marker
pub(crate) const COMMENT: char = '#';

pub(crate) const COMMENT_U8: u8 = COMMENT as u8;

/// Field delimiter, as char
pub(crate) const TAB: char = '\t';

/// Field delimiter, as u8
pub(crate) const TAB_U8: u8 = TAB as u8;

/// Number of field separators in the log line
pub(crate) const TABS: usize = 32;

/// Number of fields in the log line
#[allow(unused)]
pub(crate) const FIELDS: usize = TABS + 1;

#[cfg(feature = "time")]
pub const TIME_DATE_FMT: &[time::format_description::FormatItem<'_>] =
    time::macros::format_description!("[year]-[month]-[day]");

#[cfg(feature = "time")]
pub const TIME_TIME_FMT: &[time::format_description::FormatItem<'_>] =
    time::macros::format_description!("[hour]:[minute]:[second]");

#[cfg(feature = "chrono")]
pub const CHRONO_DATE_FMT: &str = "%Y-%m-%d";

#[cfg(feature = "chrono")]
pub const CHRONO_TIME_FMT: &str = "%H:%M:%S";
