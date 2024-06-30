use crate::{shared::*, types::*};

/// The validated raw log line
///
/// All fields are [`&str`] slices into the original log line.
///
/// On construction it checks if the line can be parsed.
/// This is useful if you cannot skip the comment lines or have reason to not trust the input for format correctness.
/// The latter should be only an issue if you do not use this crate on CloudFront logs directly.
///
/// # Panics
///
/// Technically the construction should never panic, because the input is validated before the line is sliced into fields.
/// The fields are only view into the original full log line, no parsing errors can occur.
///
/// # Examples
///
/// Use `.try_from()` or `.try_into()` to construct an instance, since action can fail.
///
/// ```rust
/// use cloudfront_logs::{borrowed::raw::ValidatedLogline, types::*};
///
/// let line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";
///
/// let item = ValidatedLogline::try_from(line).unwrap();
/// // alternative:
/// let item: ValidatedLogline<'_> = line.try_into().unwrap();
///
/// assert_eq!(item.date, "2019-12-04");
/// assert_eq!(item.sc_bytes, "392");
/// assert_eq!(item.c_ip, "192.0.2.100");
/// ```
pub type ValidatedLogline<'a> = Logline<'a, Validated>;

/// The unvalidated raw log line
///
/// All fields are [`&str`] slices into the original log line.
///
/// Unlike [`ValidatedLogline`], this variant does not check if the line can be parsed.
/// Use this if you already did a check before creating this struct.
/// A common scenario is that you 1) trust the input data and 2) skipped the comment lines.
///
/// Note: This is the only variant which can use the `From` trait instead of `TryFrom`,
/// because validation is skipped and the input data does not need to be parsed into other types.
///
/// # Panics
///
/// Construction can panic if the input is not a valid log line!
///
/// # Examples
///
/// Use `.from()` or `.into()` to construct an instance, since no validation is done.
///
/// ```rust
/// use cloudfront_logs::{borrowed::raw::UnvalidatedLogline, types::*};
///
/// let line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";
///
/// let item = UnvalidatedLogline::from(line);
/// // alternative:
/// let item: UnvalidatedLogline<'_> = line.into();
///
/// assert_eq!(item.date, "2019-12-04");
/// assert_eq!(item.sc_bytes, "392");
/// assert_eq!(item.c_ip, "192.0.2.100");
/// ```
pub type UnvalidatedLogline<'a> = Logline<'a, Unvalidated>;

/// The generic, raw log line type
///
/// Do not use it directly, prefer [`ValidatedLogline`] or [`UnvalidatedLogline`] instead.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Logline<'a, V> {
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
    __marker: PhantomData<V>,
}

impl<'a> TryFrom<&'a str> for Logline<'a, Validated> {
    type Error = &'static str;

    #[must_use]
    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        validate_line(line)?;
        let result = new_log_line(line);
        Ok(result)
    }
}

impl<'a> From<&'a str> for Logline<'a, Unvalidated> {
    #[must_use]
    fn from(line: &'a str) -> Self {
        new_log_line(line)
    }
}

fn new_log_line<'a, V>(line: &'a str) -> Logline<'a, V> {
    let mut iter = MemchrTabSplitter::new(line);

    Logline {
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
        __marker: PhantomData,
    }
}

impl<'a> From<Logline<'a, Validated>> for Logline<'a, Unvalidated> {
    #[must_use]
    fn from(validated: Logline<'a, Validated>) -> Self {
        Logline {
            date: validated.date,
            time: validated.time,
            x_edge_location: validated.x_edge_location,
            sc_bytes: validated.sc_bytes,
            c_ip: validated.c_ip,
            cs_method: validated.cs_method,
            cs_host: validated.cs_host,
            cs_uri_stem: validated.cs_uri_stem,
            sc_status: validated.sc_status,
            cs_referer: validated.cs_referer,
            cs_user_agent: validated.cs_user_agent,
            cs_uri_query: validated.cs_uri_query,
            cs_cookie: validated.cs_cookie,
            x_edge_result_type: validated.x_edge_result_type,
            x_edge_request_id: validated.x_edge_request_id,
            x_host_header: validated.x_host_header,
            cs_protocol: validated.cs_protocol,
            cs_bytes: validated.cs_bytes,
            time_taken: validated.time_taken,
            x_forwarded_for: validated.x_forwarded_for,
            ssl_protocol: validated.ssl_protocol,
            ssl_cipher: validated.ssl_cipher,
            x_edge_response_result_type: validated.x_edge_response_result_type,
            cs_protocol_version: validated.cs_protocol_version,
            fle_status: validated.fle_status,
            fle_encrypted_fields: validated.fle_encrypted_fields,
            c_port: validated.c_port,
            time_to_first_byte: validated.time_to_first_byte,
            x_edge_detailed_result_type: validated.x_edge_detailed_result_type,
            sc_content_type: validated.sc_content_type,
            sc_content_len: validated.sc_content_len,
            sc_range_start: validated.sc_range_start,
            sc_range_end: validated.sc_range_end,
            __marker: PhantomData,
        }
    }
}

impl<'a> From<Logline<'a, Unvalidated>> for Logline<'a, Validated> {
    #[must_use]
    fn from(unvalidated: Logline<'a, Unvalidated>) -> Self {
        Logline {
            date: unvalidated.date,
            time: unvalidated.time,
            x_edge_location: unvalidated.x_edge_location,
            sc_bytes: unvalidated.sc_bytes,
            c_ip: unvalidated.c_ip,
            cs_method: unvalidated.cs_method,
            cs_host: unvalidated.cs_host,
            cs_uri_stem: unvalidated.cs_uri_stem,
            sc_status: unvalidated.sc_status,
            cs_referer: unvalidated.cs_referer,
            cs_user_agent: unvalidated.cs_user_agent,
            cs_uri_query: unvalidated.cs_uri_query,
            cs_cookie: unvalidated.cs_cookie,
            x_edge_result_type: unvalidated.x_edge_result_type,
            x_edge_request_id: unvalidated.x_edge_request_id,
            x_host_header: unvalidated.x_host_header,
            cs_protocol: unvalidated.cs_protocol,
            cs_bytes: unvalidated.cs_bytes,
            time_taken: unvalidated.time_taken,
            x_forwarded_for: unvalidated.x_forwarded_for,
            ssl_protocol: unvalidated.ssl_protocol,
            ssl_cipher: unvalidated.ssl_cipher,
            x_edge_response_result_type: unvalidated.x_edge_response_result_type,
            cs_protocol_version: unvalidated.cs_protocol_version,
            fle_status: unvalidated.fle_status,
            fle_encrypted_fields: unvalidated.fle_encrypted_fields,
            c_port: unvalidated.c_port,
            time_to_first_byte: unvalidated.time_to_first_byte,
            x_edge_detailed_result_type: unvalidated.x_edge_detailed_result_type,
            sc_content_type: unvalidated.sc_content_type,
            sc_content_len: unvalidated.sc_content_len,
            sc_range_start: unvalidated.sc_range_start,
            sc_range_end: unvalidated.sc_range_end,
            __marker: PhantomData,
        }
    }
}
