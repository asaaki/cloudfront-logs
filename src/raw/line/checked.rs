use crate::shared::*;

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LogLine<'a>(pub(crate) super::LogLine<'a>);

impl<'a> std::ops::Deref for LogLine<'a> {
    type Target = super::LogLine<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LogLine<'_> {
    pub fn inner(&self) -> &super::LogLine<'_> {
        &self.0
    }
}

impl<'a> TryFrom<&'a str> for LogLine<'a> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        valid_line(line)?;

        let mut iter = split(line);

        Ok(Self(super::LogLine {
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
        }))
    }
}
