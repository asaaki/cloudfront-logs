use crate::{
    borrowed::raw::{
        Logline as RawLogline, UnvalidatedLogline as UnvalidatedRaw,
        ValidatedLogline as ValidatedRaw,
    },
    shared::*,
    types::*,
    TIME_DATE_FMT, TIME_TIME_FMT,
};

/// The validated typed log line, using [`time`](https://docs.rs/time/latest/time/index.html) crate for date and time
///
/// Most fields are parsed into more meaningful types.
///
/// On construction it checks if the line can be parsed.
/// This is useful if you cannot skip the comment lines or have reason to not trust the input for format correctness.
/// The latter should be only an issue if you do not use this crate on CloudFront logs directly.
///
/// # Panics
///
/// Construction can panic if the input is not a valid log line!
///
/// # Examples
///
/// Use `.try_from()` or `.try_into()` to construct an instance, since action can fail.
///
/// ```rust
/// use cloudfront_logs::{borrowed::typed::time::ValidatedLogline, types::*};
///
/// let line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";
///
/// let item = ValidatedLogline::try_from(line).unwrap();
/// // alternative:
/// let item: ValidatedLogline<'_> = line.try_into().unwrap();
///
/// assert_eq!(item.date, time::macros::date!(2019 - 12 - 04));
/// assert_eq!(item.sc_bytes, 392u64);
/// assert_eq!(item.cs_protocol, CsProtocol::Https);
/// ```
pub type ValidatedLogline<'a> = Logline<'a, Validated>;

/// The unvalidated typed log line, using [`time`](https://docs.rs/time/latest/time/index.html) crate for date and time
///
/// Most fields are parsed into more meaningful types.
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
/// Use `.try_from()` or `.try_into()` to construct an instance, since action can fail.
///
/// ```rust
/// use cloudfront_logs::{borrowed::typed::time::UnvalidatedLogline, types::*};
///
/// let line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";
///
/// let item = UnvalidatedLogline::try_from(line).unwrap();
/// // alternative:
/// let item: UnvalidatedLogline<'_> = line.try_into().unwrap();
///
/// assert_eq!(item.date, time::macros::date!(2019 - 12 - 04));
/// assert_eq!(item.sc_bytes, 392u64);
/// assert_eq!(item.cs_protocol, CsProtocol::Https);
/// ```
pub type UnvalidatedLogline<'a> = Logline<'a, Unvalidated>;

/// The generic, raw log line type
///
/// Do not use it directly, prefer [`ValidatedLogline`] or [`UnvalidatedLogline`] instead.
#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub struct Logline<'a, V> {
    pub date: Date,
    pub time: Time,
    pub datetime: OffsetDateTime,
    pub x_edge_location: &'a str,
    pub sc_bytes: u64,
    pub c_ip: IpAddr,
    pub cs_method: &'a str,
    pub cs_host: &'a str,
    pub cs_uri_stem: &'a str,
    pub sc_status: u16,
    pub cs_referer: Option<&'a str>,
    pub cs_user_agent: &'a str,
    pub cs_uri_query: Option<&'a str>,
    pub cs_cookie: Option<&'a str>,
    pub x_edge_result_type: EdgeResultType,
    pub x_edge_request_id: &'a str,
    pub x_host_header: &'a str,
    pub cs_protocol: CsProtocol,
    pub cs_bytes: u64,
    pub time_taken: Duration,
    pub x_forwarded_for: Option<IpAddr>,
    pub ssl_protocol: Option<SslProtocol>,
    pub ssl_cipher: Option<&'a str>,
    pub x_edge_response_result_type: EdgeResultType,
    pub cs_protocol_version: CsProtocolVersion,
    pub fle_status: Option<&'a str>,
    pub fle_encrypted_fields: Option<u64>,
    pub c_port: u16,
    pub time_to_first_byte: Duration,
    pub x_edge_detailed_result_type: DetailedEdgeResultType,
    pub sc_content_type: &'a str,
    pub sc_content_len: u64,
    pub sc_range_start: Option<u64>,
    pub sc_range_end: Option<u64>,
    __marker: PhantomData<V>,
}

impl<'a> TryFrom<&'a str> for Logline<'a, Validated> {
    type Error = &'static str;

    #[must_use]
    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        validate_line(line)?;
        new_log_line(line)
    }
}

impl<'a> TryFrom<&'a str> for Logline<'a, Unvalidated> {
    type Error = &'static str;

    #[must_use]
    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        new_log_line(line)
    }
}

fn new_log_line<'a, V>(line: &'a str) -> Result<Logline<'a, V>, &'static str> {
    let mut iter = MemchrTabSplitter::new(line);

    let date = Date::parse(iter.next().unwrap(), TIME_DATE_FMT).map_err(|_| "date invalid")?;
    let time = Time::parse(iter.next().unwrap(), TIME_TIME_FMT).map_err(|_| "time invalid")?;
    let datetime = OffsetDateTime::new_utc(date, time);

    let line = Logline {
        date,
        time,
        datetime,
        x_edge_location: iter.next().unwrap(),
        sc_bytes: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "sc_bytes invalid")?,
        c_ip: iter.next().unwrap().parse().map_err(|_| "c_ip invalid")?,
        cs_method: iter.next().unwrap(),
        cs_host: iter.next().unwrap(),
        cs_uri_stem: iter.next().unwrap(),
        sc_status: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "sc_status invalid")?,
        cs_referer: iter.next().unwrap().as_optional_str(),
        cs_user_agent: iter.next().unwrap(),
        cs_uri_query: iter.next().unwrap().as_optional_str(),
        cs_cookie: iter.next().unwrap().as_optional_str(),
        x_edge_result_type: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "x_edge_result_type invalid")?,
        x_edge_request_id: iter.next().unwrap(),
        x_host_header: iter.next().unwrap(),
        cs_protocol: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "cs_protocol invalid")?,
        cs_bytes: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "cs_bytes invalid")?,
        time_taken: iter
            .next()
            .unwrap()
            .parse::<f64>()
            .map(Duration::from_secs_f64)
            .map_err(|_| "time_taken invalid")?,
        x_forwarded_for: iter
            .next()
            .and_then(as_optional_t)
            .transpose()
            .map_err(|_| "x_forwarded_for invalid")?,
        ssl_protocol: iter
            .next()
            .and_then(as_optional_t)
            .transpose()
            .map_err(|_| "ssl_protocol invalid")?,
        ssl_cipher: iter.next().unwrap().as_optional_str(),
        x_edge_response_result_type: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "x_edge_response_result_type invalid")?,
        cs_protocol_version: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "cs_protocol_version invalid")?,
        fle_status: iter.next().unwrap().as_optional_str(),
        fle_encrypted_fields: iter
            .next()
            .and_then(as_optional_t)
            .transpose()
            .map_err(|_| "fle_encrypted_fields invalid")?,
        c_port: iter.next().unwrap().parse().map_err(|_| "c_port invalid")?,
        time_to_first_byte: iter
            .next()
            .unwrap()
            .parse::<f64>()
            .map(Duration::from_secs_f64)
            .map_err(|_| "time_to_first_byte invalid")?,
        x_edge_detailed_result_type: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "x_edge_detailed_result_type invalid")?,
        sc_content_type: iter.next().unwrap(),
        sc_content_len: iter
            .next()
            .unwrap()
            .parse()
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
        __marker: PhantomData,
    };
    Ok(line)
}

impl<'a> TryFrom<ValidatedRaw<'a>> for Logline<'a, Validated> {
    type Error = &'static str;

    #[must_use]
    fn try_from(raw: ValidatedRaw<'a>) -> Result<Self, Self::Error> {
        try_from_v(raw)
    }
}

impl<'a> TryFrom<UnvalidatedRaw<'a>> for Logline<'a, Unvalidated> {
    type Error = &'static str;

    #[must_use]
    fn try_from(raw: UnvalidatedRaw<'a>) -> Result<Self, Self::Error> {
        try_from_v(raw)
    }
}

fn try_from_v<'a, V>(raw: RawLogline<'a, V>) -> Result<Logline<'a, V>, &'static str> {
    let date = Date::parse(raw.date, TIME_DATE_FMT).map_err(|_| "date invalid")?;
    let time = Time::parse(raw.time, TIME_TIME_FMT).map_err(|_| "time invalid")?;
    let datetime = OffsetDateTime::new_utc(date, time);

    let line = Logline {
        date,
        time,
        datetime,
        x_edge_location: raw.x_edge_location,
        sc_bytes: raw.sc_bytes.parse().map_err(|_| "sc_bytes invalid")?,
        c_ip: raw.c_ip.parse().map_err(|_| "c_ip invalid")?,
        cs_method: raw.cs_method,
        cs_host: raw.cs_host,
        cs_uri_stem: raw.cs_uri_stem,
        sc_status: raw.sc_status.parse().map_err(|_| "sc_status invalid")?,
        cs_referer: raw.cs_referer.as_optional_str(),
        cs_user_agent: raw.cs_user_agent,
        cs_uri_query: raw.cs_uri_query.as_optional_str(),
        cs_cookie: raw.cs_cookie.as_optional_str(),
        x_edge_result_type: raw
            .x_edge_result_type
            .parse()
            .map_err(|_| "x_edge_result_type invalid")?,
        x_edge_request_id: raw.x_edge_request_id,
        x_host_header: raw.x_host_header,
        cs_protocol: raw.cs_protocol.parse().map_err(|_| "cs_protocol invalid")?,
        cs_bytes: raw.cs_bytes.parse().map_err(|_| "cs_bytes invalid")?,
        time_taken: raw
            .time_taken
            .parse::<f64>()
            .map(Duration::from_secs_f64)
            .map_err(|_| "time_taken invalid")?,
        x_forwarded_for: parse_as_option(raw.x_forwarded_for)
            .map_err(|_| "x_forwarded_for invalid")?,
        ssl_protocol: parse_as_option(raw.ssl_protocol).map_err(|_| "ssl_protocol invalid")?,
        ssl_cipher: raw.ssl_cipher.as_optional_str(),
        x_edge_response_result_type: raw
            .x_edge_response_result_type
            .parse()
            .map_err(|_| "x_edge_response_result_type invalid")?,
        cs_protocol_version: raw
            .cs_protocol_version
            .parse()
            .map_err(|_| "cs_protocol_version invalid")?,
        fle_status: raw.fle_status.as_optional_str(),
        fle_encrypted_fields: parse_as_option(raw.fle_encrypted_fields)
            .map_err(|_| "fle_encrypted_fields invalid")?,
        c_port: raw.c_port.parse().map_err(|_| "c_port invalid")?,
        time_to_first_byte: raw
            .time_to_first_byte
            .parse::<f64>()
            .map(Duration::from_secs_f64)
            .map_err(|_| "time_to_first_byte invalid")?,
        x_edge_detailed_result_type: raw
            .x_edge_detailed_result_type
            .parse()
            .map_err(|_| "x_edge_detailed_result_type invalid")?,
        sc_content_type: raw.sc_content_type,
        sc_content_len: raw
            .sc_content_len
            .parse()
            .map_err(|_| "sc_content_len invalid")?,
        sc_range_start: parse_as_option(raw.sc_range_start)
            .map_err(|_| "sc_range_start invalid")?,
        sc_range_end: parse_as_option(raw.sc_range_end).map_err(|_| "sc_range_end invalid")?,
        __marker: PhantomData,
    };
    Ok(line)
}

impl<'a> From<Logline<'a, Validated>> for Logline<'a, Unvalidated> {
    #[must_use]
    fn from(validated: Logline<'a, Validated>) -> Self {
        Logline {
            date: validated.date,
            time: validated.time,
            datetime: validated.datetime,
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
            datetime: unvalidated.datetime,
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
