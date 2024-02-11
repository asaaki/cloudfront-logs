pub(crate) mod checked;

// SAFETY: the line has been checked for the correct number of fields
#[cfg(feature = "unsafe")]
pub(crate) mod unsafe_;

/// The raw and untyped log line representation,
/// all fields are slices of the original log line.
/// Use this struct for an efficient and low-level access to the log line fields.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LogLine<'a> {
    pub date: &'a str,
    pub time: &'a str,
    pub x_edge_location: &'a str,
    pub sc_bytes: &'a str,
    pub c_ip: &'a str,
    pub cs_method: &'a str,
    pub cs_host: &'a str,
    pub cs_uri_stem: &'a str,
    pub sc_status: &'a str,
    pub cs_referer: &'a str,
    pub cs_user_agent: &'a str,
    pub cs_uri_query: &'a str,
    pub cs_cookie: &'a str,
    pub x_edge_result_type: &'a str,
    pub x_edge_request_id: &'a str,
    pub x_host_header: &'a str,
    pub cs_protocol: &'a str,
    pub cs_bytes: &'a str,
    pub time_taken: &'a str,
    pub x_forwarded_for: &'a str,
    pub ssl_protocol: &'a str,
    pub ssl_cipher: &'a str,
    pub x_edge_response_result_type: &'a str,
    pub cs_protocol_version: &'a str,
    pub fle_status: &'a str,
    pub fle_encrypted_fields: &'a str,
    pub c_port: &'a str,
    pub time_to_first_byte: &'a str,
    pub x_edge_detailed_result_type: &'a str,
    pub sc_content_type: &'a str,
    pub sc_content_len: &'a str,
    pub sc_range_start: &'a str,
    pub sc_range_end: &'a str,
}
