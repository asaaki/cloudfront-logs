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
            if input.starts_with('0')
                && input.contains('.')
                && let Some(ip) = leading_zero_ipv4(input)
            {
                return Ok(Self::IpAddr(IpAddr::V4(ip)));
            }
        }
        input
            .parse::<SocketAddr>()
            .map(Self::Socket)
            .map_err(|_e| "invalid X-Forwarded-For IP/socket address")
    }
}

fn leading_zero_ipv4(input: &str) -> Option<Ipv4Addr> {
    let mut pieces = input.splitn(4, '.');
    let mut octets = [0; 4];
    for octet in &mut octets {
        *octet = pieces.next()?.parse().ok()?;
    }
    Some(Ipv4Addr::from(octets))
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

impl ForwardedForAddrs {
    /// Parse a borrowed address list one address at a time, without allocating a vector.
    ///
    /// Each item uses the same rules and errors as [`Self::try_from`]. An error
    /// applies to that address; callers may stop or continue consuming the iterator.
    /// Reading only some items does not validate the rest of the list.
    /// The missing-field marker `-` must be handled by the caller, just as when
    /// converting a header directly to [`ForwardedForAddrs`].
    ///
    /// ```
    /// use cloudfront_logs::{Addressable, ForwardedForAddrs};
    /// let input = "192.0.2.1,\\x20unknown";
    /// let addresses = ForwardedForAddrs::iter_str(input).collect::<Result<Vec<_>, _>>()?;
    /// assert_eq!(addresses.len(), 2);
    /// assert_eq!(addresses.last(), Some(&Addressable::Unknown));
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn iter_str(
        input: &str,
    ) -> impl Iterator<Item = Result<Addressable, &'static str>> + Clone + '_ {
        input.split(',').map(|address| {
            let trimmed = address.trim();
            let address = trimmed.strip_prefix("\\x20").unwrap_or(trimmed);
            address
                .parse()
                .map_err(|_e| "invalid X-Forwarded-For IP(s)")
        })
    }
}

impl TryFrom<&str> for ForwardedForAddrs {
    type Error = &'static str;

    #[inline]
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::iter_str(input)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }
}

impl FromStr for ForwardedForAddrs {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::try_from(input)
    }
}
