use crate::{shared::*, types::*, CheckedRawLogLine};

pub use LogLine as ParquetLogLine;

/// A mostly borrowed version suitable for writing into parquet files
///
/// It's similar to [`CheckedRawLogLine`](crate::raw::CheckedRawLogLine),
/// but with some fields slightly more typed (time, numbers, options)
#[derive(Debug, PartialEq, parquet_derive::ParquetRecordWriter)]
pub struct LogLine<'a> {
    pub date: NaiveDate,
    pub time: &'a str, // not supported: NaiveTime
    pub datetime: NaiveDateTime,
    pub x_edge_location: &'a str,
    pub sc_bytes: u64,
    pub c_ip: &'a str,
    pub cs_method: &'a str,
    pub cs_host: &'a str,
    pub cs_uri_stem: &'a str,
    pub sc_status: u16,
    pub cs_referer: Option<&'a str>,
    pub cs_user_agent: &'a str,
    pub cs_uri_query: Option<&'a str>,
    pub cs_cookie: Option<&'a str>,
    pub x_edge_result_type: &'a str,
    pub x_edge_request_id: &'a str,
    pub x_host_header: &'a str,
    pub cs_protocol: &'a str,
    pub cs_bytes: u64,
    pub time_taken: f64, // not supported: Duration
    pub x_forwarded_for: Option<&'a str>,
    pub ssl_protocol: Option<&'a str>,
    pub ssl_cipher: Option<&'a str>,
    pub x_edge_response_result_type: &'a str,
    pub cs_protocol_version: &'a str,
    pub fle_status: Option<&'a str>,
    pub fle_encrypted_fields: Option<u64>,
    pub c_port: u16,
    pub time_to_first_byte: f64, // not supported: Duration
    pub x_edge_detailed_result_type: &'a str,
    pub sc_content_type: &'a str,
    pub sc_content_len: u64,
    pub sc_range_start: Option<u64>,
    pub sc_range_end: Option<u64>,
}

impl<'a> TryFrom<&'a str> for LogLine<'a> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        valid_line(line)?;

        let mut iter = MemchrTabSplitter::new(line);

        let date = NaiveDate::parse_from_str(iter.next().unwrap(), "%Y-%m-%d")
            .map_err(|_| "date invalid")?;
        let raw_time = iter.next().unwrap();
        let time = NaiveTime::parse_from_str(raw_time, "%H:%M:%S").map_err(|_| "time invalid")?;
        let datetime = NaiveDateTime::new(date, time);

        let line = LogLine {
            date,
            time: raw_time,
            datetime,
            x_edge_location: iter.next().unwrap(),
            sc_bytes: iter
                .next()
                .unwrap()
                .parse::<u64>()
                .map_err(|_| "sc_bytes invalid")?,
            c_ip: iter.next().unwrap(),
            cs_method: iter.next().unwrap(),
            cs_host: iter.next().unwrap(),
            cs_uri_stem: iter.next().unwrap(),
            sc_status: iter
                .next()
                .unwrap()
                .parse::<u16>()
                .map_err(|_| "sc_status invalid")?,
            cs_referer: iter.next().and_then(str::as_optional_str),
            cs_user_agent: iter.next().unwrap(),
            cs_uri_query: iter.next().and_then(str::as_optional_str),
            cs_cookie: iter.next().and_then(str::as_optional_str),
            x_edge_result_type: iter.next().unwrap(),
            x_edge_request_id: iter.next().unwrap(),
            x_host_header: iter.next().unwrap(),
            cs_protocol: iter.next().unwrap(),
            cs_bytes: iter
                .next()
                .unwrap()
                .parse::<u64>()
                .map_err(|_| "cs_bytes invalid")?,
            time_taken: iter
                .next()
                .unwrap()
                .parse::<f64>()
                .map_err(|_| "time_taken invalid")?,
            x_forwarded_for: iter.next().and_then(str::as_optional_str),
            ssl_protocol: iter.next().and_then(str::as_optional_str),
            ssl_cipher: iter.next().and_then(str::as_optional_str),
            x_edge_response_result_type: iter.next().unwrap(),
            cs_protocol_version: iter.next().unwrap(),
            fle_status: iter.next().and_then(str::as_optional_str),
            fle_encrypted_fields: iter
                .next()
                .and_then(as_optional_t)
                .transpose()
                .map_err(|_| "fle_encrypted_fields invalid")?,
            c_port: iter
                .next()
                .unwrap()
                .parse::<u16>()
                .map_err(|_| "c_port invalid")?,
            time_to_first_byte: iter
                .next()
                .unwrap()
                .parse::<f64>()
                .map_err(|_| "time_to_first_byte invalid")?,
            x_edge_detailed_result_type: iter.next().unwrap(),
            sc_content_type: iter.next().unwrap(),
            sc_content_len: iter
                .next()
                .unwrap()
                .parse::<u64>()
                .map_err(|_| "sc_content_len invalid")?,
            sc_range_start: iter
                .next()
                .and_then(as_optional_t)
                .transpose()
                .map_err(|_| "sc_range_start invalid")?,
            sc_range_end: iter
                .next()
                .and_then(as_optional_t)
                .transpose()
                .map_err(|_| "sc_range_end invalid")?,
        };
        Ok(line)
    }
}

impl<'a> TryFrom<CheckedRawLogLine<'a>> for LogLine<'a> {
    type Error = &'static str;

    fn try_from(raw: CheckedRawLogLine<'a>) -> Result<Self, Self::Error> {
        let date = NaiveDate::parse_from_str(raw.date, "%Y-%m-%d").map_err(|_| "date invalid")?;
        let time = NaiveTime::parse_from_str(raw.time, "%H:%M:%S").map_err(|_| "time invalid")?;
        let datetime = NaiveDateTime::new(date, time);

        let line = LogLine {
            date,
            time: raw.time,
            datetime,
            x_edge_location: raw.x_edge_location,
            sc_bytes: raw
                .sc_bytes
                .parse::<u64>()
                .map_err(|_| "sc_bytes invalid")?,
            c_ip: raw.c_ip,
            cs_method: raw.cs_method,
            cs_host: raw.cs_host,
            cs_uri_stem: raw.cs_uri_stem,
            sc_status: raw
                .sc_status
                .parse::<u16>()
                .map_err(|_| "sc_status invalid")?,
            cs_referer: raw.cs_referer.as_optional_str(),
            cs_user_agent: raw.cs_user_agent,
            cs_uri_query: raw.cs_uri_query.as_optional_str(),
            cs_cookie: raw.cs_cookie.as_optional_str(),
            x_edge_result_type: raw.x_edge_result_type,
            x_edge_request_id: raw.x_edge_request_id,
            x_host_header: raw.x_host_header,
            cs_protocol: raw.cs_protocol,
            cs_bytes: raw
                .cs_bytes
                .parse::<u64>()
                .map_err(|_| "cs_bytes invalid")?,
            time_taken: raw
                .time_taken
                .parse::<f64>()
                .map_err(|_| "time_taken invalid")?,
            x_forwarded_for: raw.x_forwarded_for.as_optional_str(),
            ssl_protocol: raw.ssl_protocol.as_optional_str(),
            ssl_cipher: raw.ssl_cipher.as_optional_str(),
            x_edge_response_result_type: raw.x_edge_response_result_type,
            cs_protocol_version: raw.cs_protocol_version,
            fle_status: raw.fle_status.as_optional_str(),
            fle_encrypted_fields: parse_as_option(raw.fle_encrypted_fields)
                .map_err(|_| "fle_encrypted_fields invalid")?,
            c_port: raw.c_port.parse::<u16>().map_err(|_| "c_port invalid")?,
            time_to_first_byte: raw
                .time_to_first_byte
                .parse::<f64>()
                .map_err(|_| "time_to_first_byte invalid")?,
            x_edge_detailed_result_type: raw.x_edge_detailed_result_type,
            sc_content_type: raw.sc_content_type,
            sc_content_len: raw
                .sc_content_len
                .parse::<u64>()
                .map_err(|_| "sc_content_len invalid")?,
            sc_range_start: parse_as_option(raw.sc_range_start)
                .map_err(|_| "sc_range_start invalid")?,
            sc_range_end: parse_as_option(raw.sc_range_end).map_err(|_| "sc_range_end invalid")?,
        };
        Ok(line)
    }
}
