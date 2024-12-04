use crate::{
    borrowed::raw::{UnvalidatedLogline as UnvalidatedRaw, ValidatedLogline as ValidatedRaw},
    shared::*,
    types::*,
    CHRONO_DATE_FMT, CHRONO_TIME_FMT,
};

pub use crate::types::{Datelike, Timelike};

/// The validated log line for [`parquet`] usage
///
/// Most fields are parsed into more meaningful types.
/// Unfortunately, [`parquet_derive`] does not support all the types;
/// thus we lower some fields down to:
/// * &str / Option<&str> (instead of enums, NaiveTime, IpAddr)
/// * f64 (instead of Duration)
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
/// use cloudfront_logs::{borrowed::parquet::ValidatedLogline, types::*};
///
/// let line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";
///
/// let item = ValidatedLogline::try_from(line).unwrap();
/// // alternative:
/// let item: ValidatedLogline<'_> = line.try_into().unwrap();
///
/// assert_eq!(item.date, NaiveDate::from_ymd_opt(2019, 12, 4).unwrap());
/// assert_eq!(item.sc_bytes, 392u64);
/// assert_eq!(item.cs_protocol, "https");
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, parquet_derive::ParquetRecordWriter)]
pub struct ValidatedLogline<'a> {
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
    pub time_taken: f64,
    pub x_forwarded_for: Option<&'a str>,
    pub ssl_protocol: Option<&'a str>,
    pub ssl_cipher: Option<&'a str>,
    pub x_edge_response_result_type: &'a str,
    pub cs_protocol_version: &'a str,
    pub fle_status: Option<&'a str>,
    pub fle_encrypted_fields: Option<u64>,
    pub c_port: u16,
    pub time_to_first_byte: f64,
    pub x_edge_detailed_result_type: &'a str,
    pub sc_content_type: Option<&'a str>,
    pub sc_content_len: Option<u64>,
    pub sc_range_start: Option<i64>,
    pub sc_range_end: Option<i64>,
}

impl ValidatedLogline<'_> {
    pub fn schema() -> &'static str {
        crate::consts::parquet_schemata::V1
    }

    pub fn schema_as_type() -> parquet::schema::types::Type {
        parquet::schema::parser::parse_message_type(crate::consts::parquet_schemata::V1).unwrap()
    }
}

impl<'a> TryFrom<&'a str> for ValidatedLogline<'a> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        validate_line(line)?;
        let mut iter = MemchrTabSplitter::new(line);

        let date = NaiveDate::parse_from_str(iter.next().unwrap(), CHRONO_DATE_FMT)
            .map_err(|_e| "date invalid")?;
        let raw_time = iter.next().unwrap();
        let time =
            NaiveTime::parse_from_str(raw_time, CHRONO_TIME_FMT).map_err(|_e| "time invalid")?;
        let datetime = NaiveDateTime::new(date, time);

        let line = Self {
            date,
            time: raw_time,
            datetime,
            x_edge_location: iter.next().unwrap(),
            sc_bytes: iter
                .next()
                .unwrap()
                .parse::<u64>()
                .map_err(|_e| "sc_bytes invalid")?,
            c_ip: iter.next().unwrap(),
            cs_method: iter.next().unwrap(),
            cs_host: iter.next().unwrap(),
            cs_uri_stem: iter.next().unwrap(),
            sc_status: iter
                .next()
                .unwrap()
                .parse::<u16>()
                .map_err(|_e| "sc_status invalid")?,
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
                .map_err(|_e| "cs_bytes invalid")?,
            time_taken: iter
                .next()
                .unwrap()
                .parse::<f64>()
                .map_err(|_e| "time_taken invalid")?,
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
                .map_err(|_e| "fle_encrypted_fields invalid")?,
            c_port: iter
                .next()
                .unwrap()
                .parse::<u16>()
                .map_err(|_e| "c_port invalid")?,
            time_to_first_byte: iter
                .next()
                .unwrap()
                .parse::<f64>()
                .map_err(|_e| "time_to_first_byte invalid")?,
            x_edge_detailed_result_type: iter.next().unwrap(),
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
        };
        Ok(line)
    }
}

impl<'a> TryFrom<ValidatedRaw<'a>> for ValidatedLogline<'a> {
    type Error = &'static str;

    fn try_from(raw: ValidatedRaw<'a>) -> Result<Self, Self::Error> {
        let date =
            NaiveDate::parse_from_str(raw.date, CHRONO_DATE_FMT).map_err(|_e| "date invalid")?;
        let time =
            NaiveTime::parse_from_str(raw.time, CHRONO_TIME_FMT).map_err(|_e| "time invalid")?;
        let datetime = NaiveDateTime::new(date, time);

        let line = Self {
            date,
            time: raw.time,
            datetime,
            x_edge_location: raw.x_edge_location,
            sc_bytes: raw
                .sc_bytes
                .parse::<u64>()
                .map_err(|_e| "sc_bytes invalid")?,
            c_ip: raw.c_ip,
            cs_method: raw.cs_method,
            cs_host: raw.cs_host,
            cs_uri_stem: raw.cs_uri_stem,
            sc_status: raw
                .sc_status
                .parse::<u16>()
                .map_err(|_e| "sc_status invalid")?,
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
                .map_err(|_e| "cs_bytes invalid")?,
            time_taken: raw
                .time_taken
                .parse::<f64>()
                .map_err(|_e| "time_taken invalid")?,
            x_forwarded_for: raw.x_forwarded_for.as_optional_str(),
            ssl_protocol: raw.ssl_protocol.as_optional_str(),
            ssl_cipher: raw.ssl_cipher.as_optional_str(),
            x_edge_response_result_type: raw.x_edge_response_result_type,
            cs_protocol_version: raw.cs_protocol_version,
            fle_status: raw.fle_status.as_optional_str(),
            fle_encrypted_fields: parse_as_option(raw.fle_encrypted_fields)
                .map_err(|_e| "fle_encrypted_fields invalid")?,
            c_port: raw.c_port.parse::<u16>().map_err(|_e| "c_port invalid")?,
            time_to_first_byte: raw
                .time_to_first_byte
                .parse::<f64>()
                .map_err(|_e| "time_to_first_byte invalid")?,
            x_edge_detailed_result_type: raw.x_edge_detailed_result_type,
            sc_content_type: raw.sc_content_type.as_optional_str(),
            sc_content_len: parse_as_option(raw.sc_content_len)
                .map_err(|_e| "sc_content_len invalid")?,
            sc_range_start: parse_as_option(raw.sc_range_start)
                .map_err(|_e| "sc_range_start invalid")?,
            sc_range_end: parse_as_option(raw.sc_range_end).map_err(|_e| "sc_range_end invalid")?,
        };
        Ok(line)
    }
}

/// The unvalidated log line for [`parquet`] usage
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
/// use cloudfront_logs::{borrowed::parquet::UnvalidatedLogline, types::*};
///
/// let line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";
///
/// let item = UnvalidatedLogline::try_from(line).unwrap();
/// // alternative:
/// let item: UnvalidatedLogline<'_> = line.try_into().unwrap();
///
/// assert_eq!(item.date, NaiveDate::from_ymd_opt(2019, 12, 4).unwrap());
/// assert_eq!(item.sc_bytes, 392u64);
/// assert_eq!(item.cs_protocol, "https");
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, parquet_derive::ParquetRecordWriter)]
pub struct UnvalidatedLogline<'a> {
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
    pub time_taken: f64,
    pub x_forwarded_for: Option<&'a str>,
    pub ssl_protocol: Option<&'a str>,
    pub ssl_cipher: Option<&'a str>,
    pub x_edge_response_result_type: &'a str,
    pub cs_protocol_version: &'a str,
    pub fle_status: Option<&'a str>,
    pub fle_encrypted_fields: Option<u64>,
    pub c_port: u16,
    pub time_to_first_byte: f64,
    pub x_edge_detailed_result_type: &'a str,
    pub sc_content_type: Option<&'a str>,
    pub sc_content_len: Option<u64>,
    pub sc_range_start: Option<i64>,
    pub sc_range_end: Option<i64>,
}

impl UnvalidatedLogline<'_> {
    pub fn schema() -> &'static str {
        crate::consts::parquet_schemata::V1
    }

    pub fn schema_as_type() -> parquet::schema::types::Type {
        parquet::schema::parser::parse_message_type(crate::consts::parquet_schemata::V1).unwrap()
    }
}

impl<'a> TryFrom<&'a str> for UnvalidatedLogline<'a> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let mut iter = MemchrTabSplitter::new(line);

        let date = NaiveDate::parse_from_str(iter.next().unwrap(), "%Y-%m-%d")
            .map_err(|_e| "date invalid")?;
        let raw_time = iter.next().unwrap();
        let time = NaiveTime::parse_from_str(raw_time, "%H:%M:%S").map_err(|_e| "time invalid")?;
        let datetime = NaiveDateTime::new(date, time);

        let line = Self {
            date,
            time: raw_time,
            datetime,
            x_edge_location: iter.next().unwrap(),
            sc_bytes: iter
                .next()
                .unwrap()
                .parse::<u64>()
                .map_err(|_e| "sc_bytes invalid")?,
            c_ip: iter.next().unwrap(),
            cs_method: iter.next().unwrap(),
            cs_host: iter.next().unwrap(),
            cs_uri_stem: iter.next().unwrap(),
            sc_status: iter
                .next()
                .unwrap()
                .parse::<u16>()
                .map_err(|_e| "sc_status invalid")?,
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
                .map_err(|_e| "cs_bytes invalid")?,
            time_taken: iter
                .next()
                .unwrap()
                .parse::<f64>()
                .map_err(|_e| "time_taken invalid")?,
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
                .map_err(|_e| "fle_encrypted_fields invalid")?,
            c_port: iter
                .next()
                .unwrap()
                .parse::<u16>()
                .map_err(|_e| "c_port invalid")?,
            time_to_first_byte: iter
                .next()
                .unwrap()
                .parse::<f64>()
                .map_err(|_e| "time_to_first_byte invalid")?,
            x_edge_detailed_result_type: iter.next().unwrap(),
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
        };
        Ok(line)
    }
}

impl<'a> TryFrom<UnvalidatedRaw<'a>> for UnvalidatedLogline<'a> {
    type Error = &'static str;

    fn try_from(raw: UnvalidatedRaw<'a>) -> Result<Self, Self::Error> {
        let date =
            NaiveDate::parse_from_str(raw.date, CHRONO_DATE_FMT).map_err(|_e| "date invalid")?;
        let time =
            NaiveTime::parse_from_str(raw.time, CHRONO_TIME_FMT).map_err(|_e| "time invalid")?;
        let datetime = NaiveDateTime::new(date, time);

        let line = Self {
            date,
            time: raw.time,
            datetime,
            x_edge_location: raw.x_edge_location,
            sc_bytes: raw
                .sc_bytes
                .parse::<u64>()
                .map_err(|_e| "sc_bytes invalid")?,
            c_ip: raw.c_ip,
            cs_method: raw.cs_method,
            cs_host: raw.cs_host,
            cs_uri_stem: raw.cs_uri_stem,
            sc_status: raw
                .sc_status
                .parse::<u16>()
                .map_err(|_e| "sc_status invalid")?,
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
                .map_err(|_e| "cs_bytes invalid")?,
            time_taken: raw
                .time_taken
                .parse::<f64>()
                .map_err(|_e| "time_taken invalid")?,
            x_forwarded_for: raw.x_forwarded_for.as_optional_str(),
            ssl_protocol: raw.ssl_protocol.as_optional_str(),
            ssl_cipher: raw.ssl_cipher.as_optional_str(),
            x_edge_response_result_type: raw.x_edge_response_result_type,
            cs_protocol_version: raw.cs_protocol_version,
            fle_status: raw.fle_status.as_optional_str(),
            fle_encrypted_fields: parse_as_option(raw.fle_encrypted_fields)
                .map_err(|_e| "fle_encrypted_fields invalid")?,
            c_port: raw.c_port.parse::<u16>().map_err(|_e| "c_port invalid")?,
            time_to_first_byte: raw
                .time_to_first_byte
                .parse::<f64>()
                .map_err(|_e| "time_to_first_byte invalid")?,
            x_edge_detailed_result_type: raw.x_edge_detailed_result_type,
            sc_content_type: raw.sc_content_type.as_optional_str(),
            sc_content_len: parse_as_option(raw.sc_content_len)
                .map_err(|_e| "sc_content_len invalid")?,
            sc_range_start: parse_as_option(raw.sc_range_start)
                .map_err(|_e| "sc_range_start invalid")?,
            sc_range_end: parse_as_option(raw.sc_range_end).map_err(|_e| "sc_range_end invalid")?,
        };
        Ok(line)
    }
}

impl<'a> TryFrom<ValidatedRaw<'a>> for UnvalidatedLogline<'a> {
    type Error = &'static str;

    fn try_from(raw: ValidatedRaw<'a>) -> Result<Self, Self::Error> {
        let date =
            NaiveDate::parse_from_str(raw.date, CHRONO_DATE_FMT).map_err(|_e| "date invalid")?;
        let time =
            NaiveTime::parse_from_str(raw.time, CHRONO_TIME_FMT).map_err(|_e| "time invalid")?;
        let datetime = NaiveDateTime::new(date, time);

        let line = Self {
            date,
            time: raw.time,
            datetime,
            x_edge_location: raw.x_edge_location,
            sc_bytes: raw
                .sc_bytes
                .parse::<u64>()
                .map_err(|_e| "sc_bytes invalid")?,
            c_ip: raw.c_ip,
            cs_method: raw.cs_method,
            cs_host: raw.cs_host,
            cs_uri_stem: raw.cs_uri_stem,
            sc_status: raw
                .sc_status
                .parse::<u16>()
                .map_err(|_e| "sc_status invalid")?,
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
                .map_err(|_e| "cs_bytes invalid")?,
            time_taken: raw
                .time_taken
                .parse::<f64>()
                .map_err(|_e| "time_taken invalid")?,
            x_forwarded_for: raw.x_forwarded_for.as_optional_str(),
            ssl_protocol: raw.ssl_protocol.as_optional_str(),
            ssl_cipher: raw.ssl_cipher.as_optional_str(),
            x_edge_response_result_type: raw.x_edge_response_result_type,
            cs_protocol_version: raw.cs_protocol_version,
            fle_status: raw.fle_status.as_optional_str(),
            fle_encrypted_fields: parse_as_option(raw.fle_encrypted_fields)
                .map_err(|_e| "fle_encrypted_fields invalid")?,
            c_port: raw.c_port.parse::<u16>().map_err(|_e| "c_port invalid")?,
            time_to_first_byte: raw
                .time_to_first_byte
                .parse::<f64>()
                .map_err(|_e| "time_to_first_byte invalid")?,
            x_edge_detailed_result_type: raw.x_edge_detailed_result_type,
            sc_content_type: raw.sc_content_type.as_optional_str(),
            sc_content_len: parse_as_option(raw.sc_content_len)
                .map_err(|_e| "sc_content_len invalid")?,
            sc_range_start: parse_as_option(raw.sc_range_start)
                .map_err(|_e| "sc_range_start invalid")?,
            sc_range_end: parse_as_option(raw.sc_range_end).map_err(|_e| "sc_range_end invalid")?,
        };
        Ok(line)
    }
}

impl<'a> From<ValidatedLogline<'a>> for UnvalidatedLogline<'a> {
    fn from(validated: ValidatedLogline<'a>) -> Self {
        UnvalidatedLogline {
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
        }
    }
}

impl<'a> From<UnvalidatedLogline<'a>> for ValidatedLogline<'a> {
    fn from(unvalidated: UnvalidatedLogline<'a>) -> Self {
        ValidatedLogline {
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
        }
    }
}
