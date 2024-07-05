pub(crate) use std::{marker::PhantomData, net::IpAddr, sync::Arc, time::Duration};
use std::{
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};

/// Marker for which validate the log line before parsing
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Validated;

/// Marker for which does not validate the log line before parsing
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unvalidated;

#[cfg(feature = "chrono")]
pub use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

#[cfg(feature = "time")]
pub use time::{Date, OffsetDateTime, Time, UtcOffset};

#[derive(Debug, Clone, PartialEq, strum::Display, strum::AsRefStr, strum::EnumString)]
pub enum EdgeResultType {
    Hit,
    RefreshHit,
    Miss,
    LimitExceeded,
    CapacityExceeded,
    Error,
    Redirect,

    // AWS' docs forgot something to mention
    LambdaGeneratedResponse,

    // catch-all in case AWS' docs forgot something to mention
    #[strum(default)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, strum::Display, strum::AsRefStr, strum::EnumString)]
pub enum DetailedEdgeResultType {
    // same as EdgeResultType
    Hit,
    RefreshHit,
    Miss,
    LimitExceeded,
    CapacityExceeded,
    Error,
    Redirect,

    // AWS' docs forgot something to mention
    LambdaGeneratedResponse,

    // origin shield used
    OriginShieldHit,

    // origin request lambda@edge
    MissGeneratedResponse,

    // errors if EdgeResultType is Error
    AbortedOrigin,
    ClientCommError,
    ClientGeoBlocked,
    ClientHungUpRequest,
    InvalidRequest,
    InvalidRequestBlocked,
    InvalidRequestCertificate,
    InvalidRequestHeader,
    InvalidRequestMethod,
    OriginCommError,
    OriginConnectError,
    OriginContentRangeLengthError,
    OriginDnsError,
    OriginError,
    OriginHeaderTooBigError,
    OriginInvalidResponseError,
    OriginReadError,
    OriginWriteError,
    OriginZeroSizeObjectError,
    SlowReaderOriginError,

    // catch-all in case AWS' docs forgot something to mention
    #[strum(default)]
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, strum::Display, strum::AsRefStr, strum::EnumString)]
pub enum CsProtocol {
    #[strum(serialize = "http")]
    Http,
    #[strum(serialize = "https")]
    Https,
    #[strum(serialize = "ws")]
    Ws,
    #[strum(serialize = "wss")]
    Wss,
}

#[derive(Debug, Clone, Copy, PartialEq, strum::Display, strum::AsRefStr, strum::EnumString)]
pub enum CsProtocolVersion {
    #[strum(serialize = "HTTP/3.0")]
    HTTP3_0,
    #[strum(serialize = "HTTP/2.0")]
    HTTP2_0,
    #[strum(serialize = "HTTP/1.1")]
    HTTP1_1,
    #[strum(serialize = "HTTP/1.0")]
    HTTP1_0,
    #[strum(serialize = "HTTP/0.9")]
    HTTP0_9,
}

// todo: <https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/secure-connections-supported-viewer-protocols-ciphers.html>

#[derive(Debug, Clone, Copy, PartialEq, strum::Display, strum::AsRefStr, strum::EnumString)]
pub enum SslProtocol {
    #[strum(serialize = "TLSv1.3")]
    TLSv1_3,
    #[strum(serialize = "TLSv1.2")]
    TLSv1_2,
    #[strum(serialize = "TLSv1.1")]
    TLSv1_1,
    #[strum(serialize = "TLSv1")]
    TLSv1_0,
    #[strum(serialize = "SSLv3")]
    SSLv3,
}

/// CloudFront seems to return one of three types of "IPs" if the field is set:
/// * IP address (e.g. 1.2.3.4, 2001:db8:85a3:8d3:1319:8a2e:370:7348)
/// * Socket address (e.g. 1.2.3.4:6969)
/// * "Unknown"
#[derive(Debug, Clone, PartialEq)]
pub enum Addressable {
    IpAddr(IpAddr),
    Socket(SocketAddr),
    Unknown,
}

impl From<IpAddr> for Addressable {
    fn from(ip: IpAddr) -> Self {
        Self::IpAddr(ip)
    }
}

impl From<SocketAddr> for Addressable {
    fn from(socket: SocketAddr) -> Self {
        Self::Socket(socket)
    }
}

impl TryFrom<&str> for Addressable {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input == "unknown" {
            return Ok(Self::Unknown);
        }
        let maybe_ip = input.parse::<IpAddr>();
        if let Ok(ip) = maybe_ip {
            return Ok(Self::IpAddr(ip));
        } else {
            // special case: leading zeros (0123.045.067.089)
            if input.starts_with('0') && input.contains('.') {
                let octets = input
                    .splitn(4, '.')
                    .filter_map(|s| s.parse::<u8>().ok())
                    .collect::<Vec<u8>>();
                if octets.len() == 4 {
                    return Ok(Self::IpAddr(IpAddr::V4(Ipv4Addr::new(
                        octets[0], octets[1], octets[2], octets[3],
                    ))));
                }
            }
        }
        input
            .parse::<SocketAddr>()
            .map(Self::Socket)
            .map_err(|_e| "invalid X-Forwarded-For IP/socket address")
    }
}

impl FromStr for Addressable {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::try_from(input)
    }
}

/// A list of [`Addressable`] items used in the `x-forwarded-for` header field
///
/// See [`Addressable`] for more details, especially why we cannot simply use IPv4/IPv6 only.
#[derive(Debug, Clone, PartialEq)]
pub struct ForwardedForAddrs(pub Vec<Addressable>);

impl TryFrom<&str> for ForwardedForAddrs {
    type Error = &'static str;

    #[inline]
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        const ESCAPED_SPACE: &str = "\\x20";
        const ESCAPED_SPACE_LEN: usize = ESCAPED_SPACE.len();

        let addresses: Vec<Addressable> = input
            .split(',')
            .map(|address| {
                // Note: CloudFront logs use escaped strings for X-Forwarded-For IP lists
                let trimmed = address.trim();
                if trimmed.starts_with(ESCAPED_SPACE) {
                    &trimmed[ESCAPED_SPACE_LEN..]
                } else {
                    trimmed
                }
            })
            // .filter(|address| !address.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<Addressable>, _>>()
            .map_err(|_e| "invalid X-Forwarded-For IP(s)")?;
        Ok(Self(addresses))
    }
}

impl FromStr for ForwardedForAddrs {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::try_from(input)
    }
}
