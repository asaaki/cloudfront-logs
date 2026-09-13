use crate::consts::{COMMENT_U8, TAB_U8, TABS};
use crate::types::{
    CsProtocol, CsProtocolVersion, DetailedEdgeResultType, EdgeResultType, ForwardedForAddrs,
    SslProtocol,
};
use std::{net::IpAddr, time::Duration};

/// Validates a log line
///
/// This function checks if
/// * the line is not empty,
/// * not a comment line
/// * and has the correct number of fields.
///
/// Empty fields count as fields. This checks structure, not field values.
///
/// # Examples
///
/// ```rust
/// use cloudfront_logs::validate_line;
///
/// let okay_line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";
/// let broken_line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-";
/// let empty_line = "";
/// let comment_line = "#Version: 1.0";
///
/// assert!(validate_line(okay_line).is_ok());
/// assert!(validate_line(broken_line).is_err());
/// assert!(validate_line(empty_line).is_err());
/// assert!(validate_line(comment_line).is_err());
/// ```
#[inline]
pub fn validate_line(line: &str) -> Result<(), &'static str> {
    let bytes = line.as_bytes();
    if bytes.is_empty() {
        return Err("Invalid log line (empty)");
    }
    if bytes.first() == Some(&COMMENT_U8) {
        return Err("Invalid log line (comment)");
    }
    if memchr::memchr_iter(TAB_U8, bytes).count() != TABS {
        return Err("Invalid log line (field count)");
    }
    Ok(())
}

#[inline]
#[allow(dead_code)]
pub(crate) fn split(line: &str) -> MemchrTabSplitter<'_> {
    MemchrTabSplitter::new(line)
}

#[derive(Debug, Clone)]
pub(crate) struct MemchrTabSplitter<'a> {
    pub(crate) data: &'a str,
    pub(crate) prev: usize,
    pub(crate) finished: bool,
    pub(crate) iter: memchr::Memchr<'a>,
}

impl<'a> MemchrTabSplitter<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        let prev = 0;
        let iter = memchr::memchr_iter(TAB_U8, data.as_bytes());
        Self {
            data,
            prev,
            finished: false,
            iter,
        }
    }
}

impl<'a> Iterator for MemchrTabSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let current_tab = self.iter.next();
        if let Some(tab_idx) = current_tab {
            let from = self.prev;
            let to = tab_idx;
            self.prev = to + 1;
            Some(&self.data[from..to])
        } else {
            // get field after the last tab stop
            if !self.finished {
                let from = self.prev;
                self.finished = true;
                Some(&self.data[from..])
            } else {
                None
            }
        }
    }
}

// Keep the standard float-to-duration rounding, but report out-of-range values.
pub(crate) fn parse_duration(s: &str) -> Result<std::time::Duration, ()> {
    let seconds = s.parse::<f64>().map_err(|_error| ())?;
    std::time::Duration::try_from_secs_f64(seconds).map_err(|_error| ())
}

// One conversion and error mapping for selected and complete borrowed parsing.
macro_rules! field_parser {
    ($name:ident, $field:ident, $result:ty, $parse:path) => {
        pub(crate) fn $name(value: &str) -> Result<$result, &'static str> {
            $parse(value).map_err(|_error| concat!(stringify!($field), " invalid"))
        }
    };
}

field_parser!(parse_sc_bytes, sc_bytes, u64, str::parse::<u64>);
field_parser!(parse_c_ip, c_ip, IpAddr, str::parse::<IpAddr>);
field_parser!(parse_sc_status, sc_status, u16, str::parse::<u16>);
field_parser!(
    parse_x_edge_result_type,
    x_edge_result_type,
    EdgeResultType,
    str::parse::<EdgeResultType>
);
field_parser!(
    parse_cs_protocol,
    cs_protocol,
    CsProtocol,
    str::parse::<CsProtocol>
);
field_parser!(parse_cs_bytes, cs_bytes, u64, str::parse::<u64>);
field_parser!(parse_time_taken, time_taken, Duration, parse_duration);
field_parser!(
    parse_x_forwarded_for,
    x_forwarded_for,
    Option<ForwardedForAddrs>,
    parse_as_option::<ForwardedForAddrs>
);
field_parser!(
    parse_ssl_protocol,
    ssl_protocol,
    Option<SslProtocol>,
    parse_as_option::<SslProtocol>
);
field_parser!(
    parse_x_edge_response_result_type,
    x_edge_response_result_type,
    EdgeResultType,
    str::parse::<EdgeResultType>
);
field_parser!(
    parse_cs_protocol_version,
    cs_protocol_version,
    CsProtocolVersion,
    str::parse::<CsProtocolVersion>
);
field_parser!(
    parse_fle_encrypted_fields,
    fle_encrypted_fields,
    Option<u64>,
    parse_as_option::<u64>
);
field_parser!(parse_c_port, c_port, u16, str::parse::<u16>);
field_parser!(
    parse_time_to_first_byte,
    time_to_first_byte,
    Duration,
    parse_duration
);
field_parser!(
    parse_x_edge_detailed_result_type,
    x_edge_detailed_result_type,
    DetailedEdgeResultType,
    str::parse::<DetailedEdgeResultType>
);
field_parser!(
    parse_sc_content_len,
    sc_content_len,
    Option<u64>,
    parse_as_option::<u64>
);
field_parser!(
    parse_sc_range_start,
    sc_range_start,
    Option<i64>,
    parse_as_option::<i64>
);
field_parser!(
    parse_sc_range_end,
    sc_range_end,
    Option<i64>,
    parse_as_option::<i64>
);

// if the input is "-", return Ok(None), otherwise parse the input as T;
// -> parse_as_option(iter.next().unwrap()).map_err(|_e| "…")?
// -> parse_as_option(str_input).map_err(|_e| "…")?
pub(crate) fn parse_as_option<T: std::str::FromStr>(s: &str) -> Result<Option<T>, T::Err> {
    if s == "-" {
        Ok(None)
    } else {
        s.parse().map(|v| Some(v))
    }
}

// better chainable version of parse_as_option;
// -> iter.next().and_then(as_optional_t).transpose().map_err(|_e| "…")?
pub(crate) fn as_optional_t<T: std::str::FromStr>(s: &str) -> Option<Result<T, T::Err>> {
    if s == "-" { None } else { Some(s.parse()) }
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
            Some((*self).to_string())
        }
    }
}

// str type extension trait;
// returns None if the input is "-", otherwise Some(&str)
pub(crate) trait AsOptionalStr {
    fn as_optional_str(&self) -> Option<&str>;
}

impl AsOptionalStr for str {
    fn as_optional_str(&self) -> Option<&str> {
        if self == "-" { None } else { Some(self) }
    }
}
