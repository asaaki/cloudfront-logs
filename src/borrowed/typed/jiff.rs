use crate::{
    borrowed::raw::{
        Logline as RawLogline, UnvalidatedLogline as UnvalidatedRaw,
        ValidatedLogline as ValidatedRaw,
    },
    shared::*,
    types::*,
};

/// The validated typed log line, using [`jiff`] civil date and time types.
pub type ValidatedLogline<'a> = Logline<'a, Validated>;

/// The unvalidated typed log line, using [`jiff`] civil date and time types.
pub type UnvalidatedLogline<'a> = Logline<'a, Unvalidated>;

/// The generic typed log line.
#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub struct Logline<'a, V> {
    pub date: CivilDate,
    pub time: CivilTime,
    pub datetime: CivilDateTime,
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
    pub x_forwarded_for: Option<ForwardedForAddrs>,
    pub ssl_protocol: Option<SslProtocol>,
    pub ssl_cipher: Option<&'a str>,
    pub x_edge_response_result_type: EdgeResultType,
    pub cs_protocol_version: CsProtocolVersion,
    pub fle_status: Option<&'a str>,
    pub fle_encrypted_fields: Option<u64>,
    pub c_port: u16,
    pub time_to_first_byte: Duration,
    pub x_edge_detailed_result_type: DetailedEdgeResultType,
    pub sc_content_type: Option<&'a str>,
    pub sc_content_len: Option<u64>,
    pub sc_range_start: Option<i64>,
    pub sc_range_end: Option<i64>,
    __marker: PhantomData<V>,
}

impl<'a> TryFrom<&'a str> for Logline<'a, Validated> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        validate_line(line)?;
        new_log_line(line)
    }
}

impl<'a> TryFrom<&'a str> for Logline<'a, Unvalidated> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        new_log_line(line)
    }
}

fn parse_datetime(
    date: &str,
    time: &str,
) -> Result<(CivilDate, CivilTime, CivilDateTime), &'static str> {
    let date = date.parse().map_err(|_e| "date invalid")?;
    let time = time.parse().map_err(|_e| "time invalid")?;
    Ok((date, time, CivilDateTime::from_parts(date, time)))
}

fn new_log_line<V>(line: &str) -> Result<Logline<'_, V>, &'static str> {
    let mut iter = MemchrTabSplitter::new(line);
    let (date, time, datetime) = parse_datetime(iter.next().unwrap(), iter.next().unwrap())?;

    Ok(Logline {
        date,
        time,
        datetime,
        x_edge_location: iter.next().unwrap(),
        sc_bytes: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_e| "sc_bytes invalid")?,
        c_ip: iter.next().unwrap().parse().map_err(|_e| "c_ip invalid")?,
        cs_method: iter.next().unwrap(),
        cs_host: iter.next().unwrap(),
        cs_uri_stem: iter.next().unwrap(),
        sc_status: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_e| "sc_status invalid")?,
        cs_referer: iter.next().unwrap().as_optional_str(),
        cs_user_agent: iter.next().unwrap(),
        cs_uri_query: iter.next().unwrap().as_optional_str(),
        cs_cookie: iter.next().unwrap().as_optional_str(),
        x_edge_result_type: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_e| "x_edge_result_type invalid")?,
        x_edge_request_id: iter.next().unwrap(),
        x_host_header: iter.next().unwrap(),
        cs_protocol: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_e| "cs_protocol invalid")?,
        cs_bytes: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_e| "cs_bytes invalid")?,
        time_taken: iter
            .next()
            .unwrap()
            .parse::<f64>()
            .map(Duration::from_secs_f64)
            .map_err(|_e| "time_taken invalid")?,
        x_forwarded_for: iter
            .next()
            .and_then(as_optional_t)
            .transpose()
            .map_err(|_e| "x_forwarded_for invalid")?,
        ssl_protocol: iter
            .next()
            .and_then(as_optional_t)
            .transpose()
            .map_err(|_e| "ssl_protocol invalid")?,
        ssl_cipher: iter.next().unwrap().as_optional_str(),
        x_edge_response_result_type: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_e| "x_edge_response_result_type invalid")?,
        cs_protocol_version: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_e| "cs_protocol_version invalid")?,
        fle_status: iter.next().unwrap().as_optional_str(),
        fle_encrypted_fields: iter
            .next()
            .and_then(as_optional_t)
            .transpose()
            .map_err(|_e| "fle_encrypted_fields invalid")?,
        c_port: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_e| "c_port invalid")?,
        time_to_first_byte: iter
            .next()
            .unwrap()
            .parse::<f64>()
            .map(Duration::from_secs_f64)
            .map_err(|_e| "time_to_first_byte invalid")?,
        x_edge_detailed_result_type: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_e| "x_edge_detailed_result_type invalid")?,
        sc_content_type: iter.next().unwrap().as_optional_str(),
        sc_content_len: iter
            .next()
            .and_then(as_optional_t)
            .transpose()
            .map_err(|_e| "sc_content_len invalid")?,
        sc_range_start: iter
            .next()
            .and_then(as_optional_t)
            .transpose()
            .map_err(|_e| "sc_range_start invalid")?,
        sc_range_end: iter
            .next()
            .and_then(as_optional_t)
            .transpose()
            .map_err(|_e| "sc_range_end invalid")?,
        __marker: PhantomData,
    })
}

impl<'a> TryFrom<ValidatedRaw<'a>> for Logline<'a, Validated> {
    type Error = &'static str;

    fn try_from(raw: ValidatedRaw<'a>) -> Result<Self, Self::Error> {
        try_from_v(raw)
    }
}

impl<'a> TryFrom<UnvalidatedRaw<'a>> for Logline<'a, Unvalidated> {
    type Error = &'static str;

    fn try_from(raw: UnvalidatedRaw<'a>) -> Result<Self, Self::Error> {
        try_from_v(raw)
    }
}

fn try_from_v<V>(raw: RawLogline<'_, V>) -> Result<Logline<'_, V>, &'static str> {
    let (date, time, datetime) = parse_datetime(raw.date, raw.time)?;

    Ok(Logline {
        date,
        time,
        datetime,
        x_edge_location: raw.x_edge_location,
        sc_bytes: raw.sc_bytes.parse().map_err(|_e| "sc_bytes invalid")?,
        c_ip: raw.c_ip.parse().map_err(|_e| "c_ip invalid")?,
        cs_method: raw.cs_method,
        cs_host: raw.cs_host,
        cs_uri_stem: raw.cs_uri_stem,
        sc_status: raw.sc_status.parse().map_err(|_e| "sc_status invalid")?,
        cs_referer: raw.cs_referer.as_optional_str(),
        cs_user_agent: raw.cs_user_agent,
        cs_uri_query: raw.cs_uri_query.as_optional_str(),
        cs_cookie: raw.cs_cookie.as_optional_str(),
        x_edge_result_type: raw
            .x_edge_result_type
            .parse()
            .map_err(|_e| "x_edge_result_type invalid")?,
        x_edge_request_id: raw.x_edge_request_id,
        x_host_header: raw.x_host_header,
        cs_protocol: raw
            .cs_protocol
            .parse()
            .map_err(|_e| "cs_protocol invalid")?,
        cs_bytes: raw.cs_bytes.parse().map_err(|_e| "cs_bytes invalid")?,
        time_taken: raw
            .time_taken
            .parse::<f64>()
            .map(Duration::from_secs_f64)
            .map_err(|_e| "time_taken invalid")?,
        x_forwarded_for: parse_as_option(raw.x_forwarded_for)
            .map_err(|_e| "x_forwarded_for invalid")?,
        ssl_protocol: parse_as_option(raw.ssl_protocol).map_err(|_e| "ssl_protocol invalid")?,
        ssl_cipher: raw.ssl_cipher.as_optional_str(),
        x_edge_response_result_type: raw
            .x_edge_response_result_type
            .parse()
            .map_err(|_e| "x_edge_response_result_type invalid")?,
        cs_protocol_version: raw
            .cs_protocol_version
            .parse()
            .map_err(|_e| "cs_protocol_version invalid")?,
        fle_status: raw.fle_status.as_optional_str(),
        fle_encrypted_fields: parse_as_option(raw.fle_encrypted_fields)
            .map_err(|_e| "fle_encrypted_fields invalid")?,
        c_port: raw.c_port.parse().map_err(|_e| "c_port invalid")?,
        time_to_first_byte: raw
            .time_to_first_byte
            .parse::<f64>()
            .map(Duration::from_secs_f64)
            .map_err(|_e| "time_to_first_byte invalid")?,
        x_edge_detailed_result_type: raw
            .x_edge_detailed_result_type
            .parse()
            .map_err(|_e| "x_edge_detailed_result_type invalid")?,
        sc_content_type: raw.sc_content_type.as_optional_str(),
        sc_content_len: parse_as_option(raw.sc_content_len)
            .map_err(|_e| "sc_content_len invalid")?,
        sc_range_start: parse_as_option(raw.sc_range_start)
            .map_err(|_e| "sc_range_start invalid")?,
        sc_range_end: parse_as_option(raw.sc_range_end).map_err(|_e| "sc_range_end invalid")?,
        __marker: PhantomData,
    })
}

macro_rules! impl_validation_conversion {
    ($from:ident, $to:ident) => {
        impl<'a> From<Logline<'a, $from>> for Logline<'a, $to> {
            fn from(line: Logline<'a, $from>) -> Self {
                Logline {
                    date: line.date,
                    time: line.time,
                    datetime: line.datetime,
                    x_edge_location: line.x_edge_location,
                    sc_bytes: line.sc_bytes,
                    c_ip: line.c_ip,
                    cs_method: line.cs_method,
                    cs_host: line.cs_host,
                    cs_uri_stem: line.cs_uri_stem,
                    sc_status: line.sc_status,
                    cs_referer: line.cs_referer,
                    cs_user_agent: line.cs_user_agent,
                    cs_uri_query: line.cs_uri_query,
                    cs_cookie: line.cs_cookie,
                    x_edge_result_type: line.x_edge_result_type,
                    x_edge_request_id: line.x_edge_request_id,
                    x_host_header: line.x_host_header,
                    cs_protocol: line.cs_protocol,
                    cs_bytes: line.cs_bytes,
                    time_taken: line.time_taken,
                    x_forwarded_for: line.x_forwarded_for,
                    ssl_protocol: line.ssl_protocol,
                    ssl_cipher: line.ssl_cipher,
                    x_edge_response_result_type: line.x_edge_response_result_type,
                    cs_protocol_version: line.cs_protocol_version,
                    fle_status: line.fle_status,
                    fle_encrypted_fields: line.fle_encrypted_fields,
                    c_port: line.c_port,
                    time_to_first_byte: line.time_to_first_byte,
                    x_edge_detailed_result_type: line.x_edge_detailed_result_type,
                    sc_content_type: line.sc_content_type,
                    sc_content_len: line.sc_content_len,
                    sc_range_start: line.sc_range_start,
                    sc_range_end: line.sc_range_end,
                    __marker: PhantomData,
                }
            }
        }
    };
}

impl_validation_conversion!(Validated, Unvalidated);
impl_validation_conversion!(Unvalidated, Validated);
