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

#[cfg(feature = "parquet")]
pub mod parquet_schemata {
    // taken from auto-generated schema via parquet test samples
    pub const V0: &str = r#"message rust_schema {
REQUIRED INT32 date (DATE);
REQUIRED BYTE_ARRAY time (STRING);
REQUIRED INT64 datetime (TIMESTAMP_MILLIS);
REQUIRED BYTE_ARRAY x_edge_location (STRING);
REQUIRED INT64 sc_bytes (INTEGER(64,false));
REQUIRED BYTE_ARRAY c_ip (STRING);
REQUIRED BYTE_ARRAY cs_method (STRING);
REQUIRED BYTE_ARRAY cs_host (STRING);
REQUIRED BYTE_ARRAY cs_uri_stem (STRING);
REQUIRED INT32 sc_status (INTEGER(16,false));
OPTIONAL BYTE_ARRAY cs_referer (STRING);
REQUIRED BYTE_ARRAY cs_user_agent (STRING);
OPTIONAL BYTE_ARRAY cs_uri_query (STRING);
OPTIONAL BYTE_ARRAY cs_cookie (STRING);
REQUIRED BYTE_ARRAY x_edge_result_type (STRING);
REQUIRED BYTE_ARRAY x_edge_request_id (STRING);
REQUIRED BYTE_ARRAY x_host_header (STRING);
REQUIRED BYTE_ARRAY cs_protocol (STRING);
REQUIRED INT64 cs_bytes (INTEGER(64,false));
REQUIRED DOUBLE time_taken;
OPTIONAL BYTE_ARRAY x_forwarded_for (STRING);
OPTIONAL BYTE_ARRAY ssl_protocol (STRING);
OPTIONAL BYTE_ARRAY ssl_cipher (STRING);
REQUIRED BYTE_ARRAY x_edge_response_result_type (STRING);
REQUIRED BYTE_ARRAY cs_protocol_version (STRING);
OPTIONAL BYTE_ARRAY fle_status (STRING);
OPTIONAL INT64 fle_encrypted_fields (INTEGER(64,false));
REQUIRED INT32 c_port (INTEGER(16,false));
REQUIRED DOUBLE time_to_first_byte;
REQUIRED BYTE_ARRAY x_edge_detailed_result_type (STRING);
REQUIRED BYTE_ARRAY sc_content_type (STRING);
REQUIRED INT64 sc_content_len (INTEGER(64,false));
OPTIONAL INT64 sc_range_start (INTEGER(64,false));
OPTIONAL INT64 sc_range_end (INTEGER(64,false));
    }"#;

    // derived from V0, but with considerations of the parquet format spec;
    // see https://github.com/apache/parquet-format/blob/master/LogicalTypes.md
    //
    // notes:
    // * we cannot fix "time" yet, until upstream has proper type support
    // * docs are slightly confusing, keep in mind:
    //   * TIMESTAMP(MILLIS,true) is the correct way of writing the logical TS type
    //   * docs say "INT(64,false)", but we still have to write "INTEGER(64,false)"
    pub const V1: &str = r#"message rust_schema {
REQUIRED INT32 date (DATE);
REQUIRED BYTE_ARRAY time (STRING);
REQUIRED INT64 datetime (TIMESTAMP(MILLIS,true));
REQUIRED BYTE_ARRAY x_edge_location (STRING);
REQUIRED INT64 sc_bytes (INTEGER(64,false));
REQUIRED BYTE_ARRAY c_ip (STRING);
REQUIRED BYTE_ARRAY cs_method (STRING);
REQUIRED BYTE_ARRAY cs_host (STRING);
REQUIRED BYTE_ARRAY cs_uri_stem (STRING);
REQUIRED INT32 sc_status (INTEGER(16,false));
OPTIONAL BYTE_ARRAY cs_referer (STRING);
REQUIRED BYTE_ARRAY cs_user_agent (STRING);
OPTIONAL BYTE_ARRAY cs_uri_query (STRING);
OPTIONAL BYTE_ARRAY cs_cookie (STRING);
REQUIRED BYTE_ARRAY x_edge_result_type (STRING);
REQUIRED BYTE_ARRAY x_edge_request_id (STRING);
REQUIRED BYTE_ARRAY x_host_header (STRING);
REQUIRED BYTE_ARRAY cs_protocol (STRING);
REQUIRED INT64 cs_bytes (INTEGER(64,false));
REQUIRED DOUBLE time_taken;
OPTIONAL BYTE_ARRAY x_forwarded_for (STRING);
OPTIONAL BYTE_ARRAY ssl_protocol (STRING);
OPTIONAL BYTE_ARRAY ssl_cipher (STRING);
REQUIRED BYTE_ARRAY x_edge_response_result_type (STRING);
REQUIRED BYTE_ARRAY cs_protocol_version (STRING);
OPTIONAL BYTE_ARRAY fle_status (STRING);
OPTIONAL INT64 fle_encrypted_fields (INTEGER(64,false));
REQUIRED INT32 c_port (INTEGER(16,false));
REQUIRED DOUBLE time_to_first_byte;
REQUIRED BYTE_ARRAY x_edge_detailed_result_type (STRING);
REQUIRED BYTE_ARRAY sc_content_type (STRING);
REQUIRED INT64 sc_content_len (INTEGER(64,false));
OPTIONAL INT64 sc_range_start (INTEGER(64,false));
OPTIONAL INT64 sc_range_end (INTEGER(64,false));
    }"#;
}
