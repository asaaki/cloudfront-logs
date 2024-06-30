use crate::{
    borrowed::raw::{UnvalidatedLogline as UnvalidatedRaw, ValidatedLogline as ValidatedRaw},
    shared::*,
    types::*,
};

pub type ValidatedLogline = Logline<Validated>;
pub type UnvalidatedLogline = Logline<Unvalidated>;

/// A simple log line representation owning its field data
///
/// Only primitive types from Rust's core/std library and types composable from them are used for the fields.
/// Therefore types like Date and Time are not present, because they require external dependencies.
#[derive(Debug, Clone, PartialEq)]
pub struct Logline<V> {
    pub date: String,
    pub time: String,
    pub x_edge_location: String,
    pub sc_bytes: u64,
    pub c_ip: IpAddr,
    pub cs_method: String,
    pub cs_host: String,
    pub cs_uri_stem: String,
    pub sc_status: u16,
    pub cs_referer: Option<String>,
    pub cs_user_agent: String,
    pub cs_uri_query: Option<String>,
    pub cs_cookie: Option<String>,
    pub x_edge_result_type: EdgeResultType,
    pub x_edge_request_id: String,
    pub x_host_header: String,
    pub cs_protocol: CsProtocol,
    pub cs_bytes: u64,
    pub time_taken: Duration,
    pub x_forwarded_for: Option<IpAddr>,
    pub ssl_protocol: Option<SslProtocol>,
    pub ssl_cipher: Option<String>, // *1
    pub x_edge_response_result_type: EdgeResultType,
    pub cs_protocol_version: CsProtocolVersion,
    pub fle_status: Option<String>, // *1
    pub fle_encrypted_fields: Option<u64>,
    pub c_port: u16,
    pub time_to_first_byte: Duration,
    pub x_edge_detailed_result_type: DetailedEdgeResultType,
    pub sc_content_type: String,
    pub sc_content_len: u64,
    pub sc_range_start: Option<u64>,
    pub sc_range_end: Option<u64>,
    __marker: PhantomData<V>,
}

// *1: These fields are not typed (enums),
// currently I have no use case for them and some of those have too many variants.

impl TryFrom<&str> for Logline<Validated> {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        validate_line(line)?;
        new_log_line(line)
    }
}

impl TryFrom<&str> for Logline<Unvalidated> {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        new_log_line(line)
    }
}

fn new_log_line<'a, V>(line: &'a str) -> Result<Logline<V>, &'static str> {
    let mut iter = MemchrTabSplitter::new(line);

    let line = Logline {
        date: iter.next().unwrap().to_string(),
        time: iter.next().unwrap().to_string(),
        x_edge_location: iter.next().unwrap().to_string(),
        sc_bytes: iter
            .next()
            .unwrap()
            .parse::<u64>()
            .map_err(|_| "sc_bytes invalid")?,
        c_ip: iter.next().unwrap().parse().map_err(|_| "c_ip invalid")?,
        cs_method: iter.next().unwrap().to_string(),
        cs_host: iter.next().unwrap().to_string(),
        cs_uri_stem: iter.next().unwrap().to_string(),
        sc_status: iter
            .next()
            .unwrap()
            .parse::<u16>()
            .map_err(|_| "sc_status invalid")?,
        cs_referer: iter.next().unwrap().to_optional_string(),
        cs_user_agent: iter.next().unwrap().to_string(),
        cs_uri_query: iter.next().unwrap().to_optional_string(),
        cs_cookie: iter.next().unwrap().to_optional_string(),
        x_edge_result_type: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "x_edge_result_type invalid")?,
        x_edge_request_id: iter.next().unwrap().to_string(),
        x_host_header: iter.next().unwrap().to_string(),
        cs_protocol: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "cs_protocol invalid")?,
        cs_bytes: iter
            .next()
            .unwrap()
            .parse::<u64>()
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
        ssl_cipher: iter.next().unwrap().to_optional_string(),
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
        fle_status: iter.next().unwrap().to_optional_string(),
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
            .map(Duration::from_secs_f64)
            .map_err(|_| "time_to_first_byte invalid")?,
        x_edge_detailed_result_type: iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "x_edge_detailed_result_type invalid")?,
        sc_content_type: iter.next().unwrap().to_string(),
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
        __marker: PhantomData,
    };
    Ok(line)
}

impl Logline<Validated> {
    pub fn try_from_with_raw(line: &str) -> Result<Self, &'static str> {
        let raw = ValidatedRaw::try_from(line)?;
        let line = Self::try_from(raw)?;
        Ok(line)
    }
}

impl Logline<Unvalidated> {
    pub fn try_from_with_raw(line: &str) -> Result<Self, &'static str> {
        let raw = UnvalidatedRaw::from(line);
        let line = Self::try_from(raw)?;
        Ok(line)
    }
}

impl TryFrom<ValidatedRaw<'_>> for Logline<Validated> {
    type Error = &'static str;

    fn try_from(raw: ValidatedRaw<'_>) -> Result<Self, Self::Error> {
        let line = Self {
            date: raw.date.to_string(),
            time: raw.time.to_string(),
            x_edge_location: raw.x_edge_location.to_string(),
            sc_bytes: raw
                .sc_bytes
                .parse::<u64>()
                .map_err(|_| "sc_bytes invalid")?,
            c_ip: raw.c_ip.parse().map_err(|_| "c_ip invalid")?,
            cs_method: raw.cs_method.to_string(),
            cs_host: raw.cs_host.to_string(),
            cs_uri_stem: raw.cs_uri_stem.to_string(),
            sc_status: raw
                .sc_status
                .parse::<u16>()
                .map_err(|_| "sc_status invalid")?,
            cs_referer: raw.cs_referer.to_optional_string(),
            cs_user_agent: raw.cs_user_agent.to_string(),
            cs_uri_query: raw.cs_uri_query.to_optional_string(),
            cs_cookie: raw.cs_cookie.to_optional_string(),
            x_edge_result_type: raw
                .x_edge_result_type
                .parse()
                .map_err(|_| "x_edge_result_type invalid")?,
            x_edge_request_id: raw.x_edge_request_id.to_string(),
            x_host_header: raw.x_host_header.to_string(),
            cs_protocol: raw.cs_protocol.parse().map_err(|_| "cs_protocol invalid")?,
            cs_bytes: raw
                .cs_bytes
                .parse::<u64>()
                .map_err(|_| "cs_bytes invalid")?,
            time_taken: raw
                .time_taken
                .parse::<f64>()
                .map(Duration::from_secs_f64)
                .map_err(|_| "time_taken invalid")?,
            x_forwarded_for: parse_as_option(raw.x_forwarded_for)
                .map_err(|_| "x_forwarded_for invalid")?,
            ssl_protocol: parse_as_option(raw.ssl_protocol).map_err(|_| "ssl_protocol invalid")?,
            ssl_cipher: raw.ssl_cipher.to_optional_string(),
            x_edge_response_result_type: raw
                .x_edge_response_result_type
                .parse()
                .map_err(|_| "x_edge_response_result_type invalid")?,
            cs_protocol_version: raw
                .cs_protocol_version
                .parse()
                .map_err(|_| "cs_protocol_version invalid")?,
            fle_status: raw.fle_status.to_optional_string(),
            fle_encrypted_fields: parse_as_option(raw.fle_encrypted_fields)
                .map_err(|_| "fle_encrypted_fields invalid")?,
            c_port: raw.c_port.parse::<u16>().map_err(|_| "c_port invalid")?,
            time_to_first_byte: raw
                .time_to_first_byte
                .parse::<f64>()
                .map(Duration::from_secs_f64)
                .map_err(|_| "time_to_first_byte invalid")?,
            x_edge_detailed_result_type: raw
                .x_edge_detailed_result_type
                .parse()
                .map_err(|_| "x_edge_detailed_result_type invalid")?,
            sc_content_type: raw.sc_content_type.to_string(),
            sc_content_len: raw
                .sc_content_len
                .parse::<u64>()
                .map_err(|_| "sc_content_len invalid")?,
            sc_range_start: parse_as_option(raw.sc_range_start)
                .map_err(|_| "sc_range_start invalid")?,
            sc_range_end: parse_as_option(raw.sc_range_end).map_err(|_| "sc_range_end invalid")?,
            __marker: PhantomData,
        };
        Ok(line)
    }
}

impl TryFrom<UnvalidatedRaw<'_>> for Logline<Unvalidated> {
    type Error = &'static str;

    fn try_from(raw: UnvalidatedRaw<'_>) -> Result<Self, Self::Error> {
        let line = Self {
            date: raw.date.to_string(),
            time: raw.time.to_string(),
            x_edge_location: raw.x_edge_location.to_string(),
            sc_bytes: raw
                .sc_bytes
                .parse::<u64>()
                .map_err(|_| "sc_bytes invalid")?,
            c_ip: raw.c_ip.parse().map_err(|_| "c_ip invalid")?,
            cs_method: raw.cs_method.to_string(),
            cs_host: raw.cs_host.to_string(),
            cs_uri_stem: raw.cs_uri_stem.to_string(),
            sc_status: raw
                .sc_status
                .parse::<u16>()
                .map_err(|_| "sc_status invalid")?,
            cs_referer: raw.cs_referer.to_optional_string(),
            cs_user_agent: raw.cs_user_agent.to_string(),
            cs_uri_query: raw.cs_uri_query.to_optional_string(),
            cs_cookie: raw.cs_cookie.to_optional_string(),
            x_edge_result_type: raw
                .x_edge_result_type
                .parse()
                .map_err(|_| "x_edge_result_type invalid")?,
            x_edge_request_id: raw.x_edge_request_id.to_string(),
            x_host_header: raw.x_host_header.to_string(),
            cs_protocol: raw.cs_protocol.parse().map_err(|_| "cs_protocol invalid")?,
            cs_bytes: raw
                .cs_bytes
                .parse::<u64>()
                .map_err(|_| "cs_bytes invalid")?,
            time_taken: raw
                .time_taken
                .parse::<f64>()
                .map(Duration::from_secs_f64)
                .map_err(|_| "time_taken invalid")?,
            x_forwarded_for: parse_as_option(raw.x_forwarded_for)
                .map_err(|_| "x_forwarded_for invalid")?,
            ssl_protocol: parse_as_option(raw.ssl_protocol).map_err(|_| "ssl_protocol invalid")?,
            ssl_cipher: raw.ssl_cipher.to_optional_string(),
            x_edge_response_result_type: raw
                .x_edge_response_result_type
                .parse()
                .map_err(|_| "x_edge_response_result_type invalid")?,
            cs_protocol_version: raw
                .cs_protocol_version
                .parse()
                .map_err(|_| "cs_protocol_version invalid")?,
            fle_status: raw.fle_status.to_optional_string(),
            fle_encrypted_fields: parse_as_option(raw.fle_encrypted_fields)
                .map_err(|_| "fle_encrypted_fields invalid")?,
            c_port: raw.c_port.parse::<u16>().map_err(|_| "c_port invalid")?,
            time_to_first_byte: raw
                .time_to_first_byte
                .parse::<f64>()
                .map(Duration::from_secs_f64)
                .map_err(|_| "time_to_first_byte invalid")?,
            x_edge_detailed_result_type: raw
                .x_edge_detailed_result_type
                .parse()
                .map_err(|_| "x_edge_detailed_result_type invalid")?,
            sc_content_type: raw.sc_content_type.to_string(),
            sc_content_len: raw
                .sc_content_len
                .parse::<u64>()
                .map_err(|_| "sc_content_len invalid")?,
            sc_range_start: parse_as_option(raw.sc_range_start)
                .map_err(|_| "sc_range_start invalid")?,
            sc_range_end: parse_as_option(raw.sc_range_end).map_err(|_| "sc_range_end invalid")?,
            __marker: PhantomData,
        };
        Ok(line)
    }
}
