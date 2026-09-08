use crate::{
    borrowed::raw::{
        Logline as RawLogline, UnvalidatedLogline as UnvalidatedRaw,
        ValidatedLogline as ValidatedRaw,
    },
    shared::*,
    types::*,
};

#[cfg(feature = "chrono")]
use crate::consts::{CHRONO_DATE_FMT, CHRONO_TIME_FMT};
#[cfg(feature = "time")]
use crate::consts::{TIME_DATE_FMT, TIME_TIME_FMT};

/// Extension API for date/time representations used by generic structured loglines.
///
/// Most users should use [`ValidatedSimpleLogline`], [`UnvalidatedSimpleLogline`],
/// [`ValidatedTypedLogline`], or [`UnvalidatedTypedLogline`] instead of implementing this trait.
///
/// A custom backend can keep date and time as borrowed text:
///
/// ```
/// use cloudfront_logs::{borrowed::structured::{DateTimeBackend, Logline}, Unvalidated};
///
/// struct TextBackend;
/// impl DateTimeBackend for TextBackend {
///     type Date<'a> = &'a str;
///     type Time<'a> = &'a str;
///
///     fn parse<'a>(date: &'a str, time: &'a str) -> Result<(&'a str, &'a str), &'static str> {
///         Ok((date, time))
///     }
/// }
///
/// # let input = "2019-12-04\t21:02:31\tLAX1\t392\t192.0.2.100\tGET\td.example\t/\t200\t-\tagent\t-\t-\tHit\tid\td.example\thttps\t23\t0.001\t-\tTLSv1.2\tcipher\tHit\tHTTP/2.0\t-\t-\t11040\t0.001\tHit\ttext/html\t78\t-\t-";
/// let line = Logline::<Unvalidated, TextBackend>::try_from(input)?;
/// assert_eq!(line.date, "2019-12-04");
/// # Ok::<(), &'static str>(())
/// ```
pub trait DateTimeBackend {
    type Date<'a>: Clone + std::fmt::Debug + PartialEq;
    type Time<'a>: Clone + std::fmt::Debug + PartialEq;

    fn parse<'a>(
        date: &'a str,
        time: &'a str,
    ) -> Result<(Self::Date<'a>, Self::Time<'a>), &'static str>;
}

/// Extension API for a backend whose parsed date and time can be combined into a date-time value.
///
/// [`DateTimeBackend::parse`] must report invalid date or time input. Once parsing succeeds,
/// `datetime` must combine those compatible values without another fallible operation.
pub trait TypedDateTimeBackend: DateTimeBackend {
    type DateTime;

    fn datetime(date: &Self::Date<'_>, time: &Self::Time<'_>) -> Self::DateTime;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text;

impl DateTimeBackend for Text {
    type Date<'a> = &'a str;
    type Time<'a> = &'a str;

    fn parse<'a>(
        date: &'a str,
        time: &'a str,
    ) -> Result<(Self::Date<'a>, Self::Time<'a>), &'static str> {
        Ok((date, time))
    }
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Selected;

#[cfg(feature = "time")]
impl DateTimeBackend for Selected {
    type Date<'a> = time::Date;
    type Time<'a> = time::Time;

    fn parse<'a>(
        date: &'a str,
        time: &'a str,
    ) -> Result<(Self::Date<'a>, Self::Time<'a>), &'static str> {
        let date = time::Date::parse(date, TIME_DATE_FMT).map_err(|_e| "date invalid")?;
        let time = time::Time::parse(time, TIME_TIME_FMT).map_err(|_e| "time invalid")?;
        Ok((date, time))
    }
}

#[cfg(feature = "time")]
impl TypedDateTimeBackend for Selected {
    type DateTime = time::OffsetDateTime;

    fn datetime(date: &Self::Date<'_>, time: &Self::Time<'_>) -> Self::DateTime {
        time::OffsetDateTime::new_utc(*date, *time)
    }
}

#[cfg(feature = "chrono")]
impl DateTimeBackend for Selected {
    type Date<'a> = chrono::NaiveDate;
    type Time<'a> = chrono::NaiveTime;

    fn parse<'a>(
        date: &'a str,
        time: &'a str,
    ) -> Result<(Self::Date<'a>, Self::Time<'a>), &'static str> {
        let date = chrono::NaiveDate::parse_from_str(date, CHRONO_DATE_FMT)
            .map_err(|_e| "date invalid")?;
        let time = chrono::NaiveTime::parse_from_str(time, CHRONO_TIME_FMT)
            .map_err(|_e| "time invalid")?;
        Ok((date, time))
    }
}

#[cfg(feature = "chrono")]
impl TypedDateTimeBackend for Selected {
    type DateTime = chrono::NaiveDateTime;

    fn datetime(date: &Self::Date<'_>, time: &Self::Time<'_>) -> Self::DateTime {
        chrono::NaiveDateTime::new(*date, *time)
    }
}

#[cfg(feature = "jiff")]
impl DateTimeBackend for Selected {
    type Date<'a> = jiff::civil::Date;
    type Time<'a> = jiff::civil::Time;

    fn parse<'a>(
        date: &'a str,
        time: &'a str,
    ) -> Result<(Self::Date<'a>, Self::Time<'a>), &'static str> {
        let date = date.parse().map_err(|_e| "date invalid")?;
        let time = time.parse().map_err(|_e| "time invalid")?;
        Ok((date, time))
    }
}

#[cfg(feature = "jiff")]
impl TypedDateTimeBackend for Selected {
    type DateTime = jiff::civil::DateTime;

    fn datetime(date: &Self::Date<'_>, time: &Self::Time<'_>) -> Self::DateTime {
        jiff::civil::DateTime::from_parts(*date, *time)
    }
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
pub type ValidatedTypedLogline<'a> = TypedLogline<'a, Validated>;

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
pub type UnvalidatedTypedLogline<'a> = TypedLogline<'a, Unvalidated>;

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub struct TypedLogline<'a, V> {
    pub date: <Selected as DateTimeBackend>::Date<'static>,
    pub time: <Selected as DateTimeBackend>::Time<'static>,
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

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
impl<'a, V> From<Logline<'a, V, Selected>> for TypedLogline<'a, V> {
    fn from(line: Logline<'a, V, Selected>) -> Self {
        Self {
            date: line.date,
            time: line.time,
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

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
impl<'a, V> TypedLogline<'a, V> {
    pub fn datetime(&self) -> <Selected as TypedDateTimeBackend>::DateTime {
        Selected::datetime(&self.date, &self.time)
    }
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
impl<'a> TryFrom<&'a str> for TypedLogline<'a, Validated> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        Logline::<Validated, Selected>::try_from(line).map(Into::into)
    }
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
impl<'a> TryFrom<&'a str> for TypedLogline<'a, Unvalidated> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        Logline::<Unvalidated, Selected>::try_from(line).map(Into::into)
    }
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
impl<'a> TryFrom<ValidatedRaw<'a>> for TypedLogline<'a, Validated> {
    type Error = &'static str;

    fn try_from(raw: ValidatedRaw<'a>) -> Result<Self, Self::Error> {
        Logline::<Validated, Selected>::try_from(raw).map(Into::into)
    }
}

#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
impl<'a> TryFrom<UnvalidatedRaw<'a>> for TypedLogline<'a, Unvalidated> {
    type Error = &'static str;

    fn try_from(raw: UnvalidatedRaw<'a>) -> Result<Self, Self::Error> {
        Logline::<Unvalidated, Selected>::try_from(raw).map(Into::into)
    }
}

pub type ValidatedSimpleLogline<'a> = SimpleLogline<'a, Validated>;
pub type UnvalidatedSimpleLogline<'a> = SimpleLogline<'a, Unvalidated>;

#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleLogline<'a, V> {
    pub date: &'a str,
    pub time: &'a str,
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

impl<'a> TryFrom<&'a str> for SimpleLogline<'a, Validated> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        validate_line(line)?;
        new_simple_logline(line)
    }
}

impl<'a> TryFrom<&'a str> for SimpleLogline<'a, Unvalidated> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        new_simple_logline(line)
    }
}

fn new_simple_logline<V>(line: &str) -> Result<SimpleLogline<'_, V>, &'static str> {
    new_log_line::<V, Text>(line).map(simple_from_generic)
}

impl<'a> TryFrom<ValidatedRaw<'a>> for SimpleLogline<'a, Validated> {
    type Error = &'static str;

    fn try_from(raw: ValidatedRaw<'a>) -> Result<Self, Self::Error> {
        try_from_raw::<Validated, Text>(raw).map(simple_from_generic)
    }
}

impl<'a> TryFrom<UnvalidatedRaw<'a>> for SimpleLogline<'a, Unvalidated> {
    type Error = &'static str;

    fn try_from(raw: UnvalidatedRaw<'a>) -> Result<Self, Self::Error> {
        try_from_raw::<Unvalidated, Text>(raw).map(simple_from_generic)
    }
}

fn simple_from_generic<V>(line: Logline<'_, V, Text>) -> SimpleLogline<'_, V> {
    SimpleLogline {
        date: line.date,
        time: line.time,
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

#[must_use]
#[derive(Debug, Clone)]
pub struct Logline<'a, V, D: DateTimeBackend> {
    pub date: D::Date<'a>,
    pub time: D::Time<'a>,
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
    __lifetime: PhantomData<&'a ()>,
}

impl<'a, V, D: DateTimeBackend> PartialEq for Logline<'a, V, D> {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
            && self.time == other.time
            && self.x_edge_location == other.x_edge_location
            && self.sc_bytes == other.sc_bytes
            && self.c_ip == other.c_ip
            && self.cs_method == other.cs_method
            && self.cs_host == other.cs_host
            && self.cs_uri_stem == other.cs_uri_stem
            && self.sc_status == other.sc_status
            && self.cs_referer == other.cs_referer
            && self.cs_user_agent == other.cs_user_agent
            && self.cs_uri_query == other.cs_uri_query
            && self.cs_cookie == other.cs_cookie
            && self.x_edge_result_type == other.x_edge_result_type
            && self.x_edge_request_id == other.x_edge_request_id
            && self.x_host_header == other.x_host_header
            && self.cs_protocol == other.cs_protocol
            && self.cs_bytes == other.cs_bytes
            && self.time_taken == other.time_taken
            && self.x_forwarded_for == other.x_forwarded_for
            && self.ssl_protocol == other.ssl_protocol
            && self.ssl_cipher == other.ssl_cipher
            && self.x_edge_response_result_type == other.x_edge_response_result_type
            && self.cs_protocol_version == other.cs_protocol_version
            && self.fle_status == other.fle_status
            && self.fle_encrypted_fields == other.fle_encrypted_fields
            && self.c_port == other.c_port
            && self.time_to_first_byte == other.time_to_first_byte
            && self.x_edge_detailed_result_type == other.x_edge_detailed_result_type
            && self.sc_content_type == other.sc_content_type
            && self.sc_content_len == other.sc_content_len
            && self.sc_range_start == other.sc_range_start
            && self.sc_range_end == other.sc_range_end
    }
}

impl<'a, V, D: TypedDateTimeBackend> Logline<'a, V, D> {
    pub fn datetime(&self) -> D::DateTime {
        D::datetime(&self.date, &self.time)
    }
}

impl<'a, D: DateTimeBackend> TryFrom<&'a str> for Logline<'a, Validated, D> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        validate_line(line)?;
        new_log_line(line)
    }
}

impl<'a, D: DateTimeBackend> TryFrom<&'a str> for Logline<'a, Unvalidated, D> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        new_log_line(line)
    }
}

fn new_log_line<'a, V, D: DateTimeBackend>(
    line: &'a str,
) -> Result<Logline<'a, V, D>, &'static str> {
    let mut iter = MemchrTabSplitter::new(line);
    let (date, time) = D::parse(iter.next().unwrap(), iter.next().unwrap())?;

    Ok(Logline {
        date,
        time,
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
        __lifetime: PhantomData,
    })
}

impl<'a, D: DateTimeBackend> TryFrom<ValidatedRaw<'a>> for Logline<'a, Validated, D> {
    type Error = &'static str;

    fn try_from(raw: ValidatedRaw<'a>) -> Result<Self, Self::Error> {
        try_from_raw(raw)
    }
}

impl<'a, D: DateTimeBackend> TryFrom<UnvalidatedRaw<'a>> for Logline<'a, Unvalidated, D> {
    type Error = &'static str;

    fn try_from(raw: UnvalidatedRaw<'a>) -> Result<Self, Self::Error> {
        try_from_raw(raw)
    }
}

fn try_from_raw<'a, V, D: DateTimeBackend>(
    raw: RawLogline<'a, V>,
) -> Result<Logline<'a, V, D>, &'static str> {
    let (date, time) = D::parse(raw.date, raw.time)?;

    Ok(Logline {
        date,
        time,
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
        __lifetime: PhantomData,
    })
}

macro_rules! convert_validation_state {
    ($from:ty, $to:ty) => {
        impl<'a, D: DateTimeBackend> From<Logline<'a, $from, D>> for Logline<'a, $to, D> {
            fn from(line: Logline<'a, $from, D>) -> Self {
                Logline {
                    date: line.date,
                    time: line.time,
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
                    __lifetime: PhantomData,
                }
            }
        }
    };
}

convert_validation_state!(Validated, Unvalidated);
convert_validation_state!(Unvalidated, Validated);

macro_rules! convert_facade_validation_state {
    ($name:ident, $from:ty, $to:ty) => {
        impl<'a> From<$name<'a, $from>> for $name<'a, $to> {
            fn from(line: $name<'a, $from>) -> Self {
                Self {
                    date: line.date,
                    time: line.time,
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

convert_facade_validation_state!(SimpleLogline, Validated, Unvalidated);
convert_facade_validation_state!(SimpleLogline, Unvalidated, Validated);
#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
convert_facade_validation_state!(TypedLogline, Validated, Unvalidated);
#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
convert_facade_validation_state!(TypedLogline, Unvalidated, Validated);
