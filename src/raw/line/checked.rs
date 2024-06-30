use crate::shared::*;

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

impl<'a> TryFrom<&'a str> for LogLine<'a> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        validate_line(line)?;

        let mut iter = MemchrTabSplitter::new(line);

        Ok(Self {
            date: iter.next().unwrap(),
            time: iter.next().unwrap(),
            x_edge_location: iter.next().unwrap(),
            sc_bytes: iter.next().unwrap(),
            c_ip: iter.next().unwrap(),
            cs_method: iter.next().unwrap(),
            cs_host: iter.next().unwrap(),
            cs_uri_stem: iter.next().unwrap(),
            sc_status: iter.next().unwrap(),
            cs_referer: iter.next().unwrap(),
            cs_user_agent: iter.next().unwrap(),
            cs_uri_query: iter.next().unwrap(),
            cs_cookie: iter.next().unwrap(),
            x_edge_result_type: iter.next().unwrap(),
            x_edge_request_id: iter.next().unwrap(),
            x_host_header: iter.next().unwrap(),
            cs_protocol: iter.next().unwrap(),
            cs_bytes: iter.next().unwrap(),
            time_taken: iter.next().unwrap(),
            x_forwarded_for: iter.next().unwrap(),
            ssl_protocol: iter.next().unwrap(),
            ssl_cipher: iter.next().unwrap(),
            x_edge_response_result_type: iter.next().unwrap(),
            cs_protocol_version: iter.next().unwrap(),
            fle_status: iter.next().unwrap(),
            fle_encrypted_fields: iter.next().unwrap(),
            c_port: iter.next().unwrap(),
            time_to_first_byte: iter.next().unwrap(),
            x_edge_detailed_result_type: iter.next().unwrap(),
            sc_content_type: iter.next().unwrap(),
            sc_content_len: iter.next().unwrap(),
            sc_range_start: iter.next().unwrap(),
            sc_range_end: iter.next().unwrap(),
        })
    }
}
