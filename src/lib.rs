//! AWS CloudFront logs parser
//!
//! The log file format is described in the official documentation:
//! <https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html#LogFileFormat>

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(test)]
mod tests;

// shared functions
mod shared;

pub mod raw;

#[cfg(feature = "alloc")]
pub mod simple;

#[cfg(feature = "typed")]
pub mod typed;

#[cfg(feature = "parquet")]
pub mod parquet;

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

// top-level re-exports
pub use types::*;

pub use raw::{CheckedRawLogLine, CheckedRawLogLineView, SmartRawLogLineView};

#[cfg(feature = "alloc")]
pub use simple::SimpleLogLine;

#[cfg(feature = "typed")]
pub use typed::TypedLogLine;

#[cfg(feature = "parquet")]
pub use parquet::ParquetLogLine;
