//! AWS CloudFront logs parser
//!
//! The log file format is described in the official documentation:
//! <https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html#LogFileFormat>

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(test)]
mod tests;

pub mod raw;

#[cfg(feature = "alloc")]
pub mod simple;

#[cfg(feature = "typed")]
pub mod typed;

// common types used in simple and typed modules
#[cfg(any(feature = "alloc", feature = "typed"))]
pub mod types;

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

/// Field delimiter
pub(crate) const TAB: char = '\t';

/// Number of field separators in the log line
pub(crate) const TABS: usize = 32;

/// Number of fields in the log line
#[allow(unused)]
pub(crate) const FIELDS: usize = TABS + 1;

// shared functions

#[inline]
pub(crate) fn valid_line(line: &str) -> Result<(), &'static str> {
    if line.as_bytes()[..1].eq(b"#") {
        return Err("Invalid log line (comment)");
    }
    if memchr::memchr_iter(TAB as u8, line.as_bytes()).count() != TABS {
        return Err("Invalid log line (field count)");
    }
    Ok(())
}

#[inline]
pub(crate) fn splitn(line: &str) -> std::str::SplitTerminator<'_, char> {
    line.split_terminator(crate::TAB)
}

// if the input is "-", return Ok(None), otherwise parse the input as T
pub(crate) fn parse_as_option<T: std::str::FromStr>(s: &str) -> Result<Option<T>, T::Err> {
    if s == "-" {
        Ok(None)
    } else {
        s.parse().map(|v| Some(v))
    }
}

// String type extension trait;
// returns None if the input is "-", otherwise Some(String)
pub(crate) trait ToOptionalString {
    fn to_optional_string(&self) -> Option<String>;
}

impl ToOptionalString for &str {
    fn to_optional_string(&self) -> Option<String> {
        if self == &"-" {
            None
        } else {
            Some(self.to_string())
        }
    }
}

// top-level re-exports
pub use types::*;

pub use raw::{CheckedRawLogLine, CheckedRawLogLineView, RawLogLine, SmartRawLogLineView};

#[cfg(feature = "unsafe")]
pub use raw::UnsafeRawLogLine;

#[cfg(feature = "alloc")]
pub use simple::SimpleLogLine;

#[cfg(feature = "typed")]
pub use typed::TypedLogLine;
