pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
pub use std::time::Duration;

use std::str::FromStr;

#[cfg(feature = "typed")]
pub use time::{
    format_description::FormatItem, macros::format_description, Date, OffsetDateTime, Time,
    UtcOffset,
};

#[cfg(feature = "typed")]
pub use time::macros as time_macros;

#[derive(Debug, PartialEq)]
pub enum EdgeResultType {
    Hit,
    RefreshHit,
    Miss,
    LimitExceeded,
    CapacityExceeded,
    Error,
    Redirect,

    // catch-all in case AWS' docs forgot something to mention
    Other(String),
}

impl FromStr for EdgeResultType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Hit" => EdgeResultType::Hit,
            "RefreshHit" => EdgeResultType::RefreshHit,
            "Miss" => EdgeResultType::Miss,
            "LimitExceeded" => EdgeResultType::LimitExceeded,
            "CapacityExceeded" => EdgeResultType::CapacityExceeded,
            "Error" => EdgeResultType::Error,
            "Redirect" => EdgeResultType::Redirect,
            _ => EdgeResultType::Other(s.to_string()),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum DetailedEdgeResultType {
    // same as EdgeResultType
    Hit,
    RefreshHit,
    Miss,
    LimitExceeded,
    CapacityExceeded,
    Error,
    Redirect,

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
    Other(String),
}

impl FromStr for DetailedEdgeResultType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Hit" => DetailedEdgeResultType::Hit,
            "RefreshHit" => DetailedEdgeResultType::RefreshHit,
            "Miss" => DetailedEdgeResultType::Miss,
            "LimitExceeded" => DetailedEdgeResultType::LimitExceeded,
            "CapacityExceeded" => DetailedEdgeResultType::CapacityExceeded,
            "Error" => DetailedEdgeResultType::Error,
            "Redirect" => DetailedEdgeResultType::Redirect,
            "OriginShieldHit" => DetailedEdgeResultType::OriginShieldHit,
            "MissGeneratedResponse" => DetailedEdgeResultType::MissGeneratedResponse,
            "AbortedOrigin" => DetailedEdgeResultType::AbortedOrigin,
            "ClientCommError" => DetailedEdgeResultType::ClientCommError,
            "ClientGeoBlocked" => DetailedEdgeResultType::ClientGeoBlocked,
            "ClientHungUpRequest" => DetailedEdgeResultType::ClientHungUpRequest,
            "InvalidRequest" => DetailedEdgeResultType::InvalidRequest,
            "InvalidRequestBlocked" => DetailedEdgeResultType::InvalidRequestBlocked,
            "InvalidRequestCertificate" => DetailedEdgeResultType::InvalidRequestCertificate,
            "InvalidRequestHeader" => DetailedEdgeResultType::InvalidRequestHeader,
            "InvalidRequestMethod" => DetailedEdgeResultType::InvalidRequestMethod,
            "OriginCommError" => DetailedEdgeResultType::OriginCommError,
            "OriginConnectError" => DetailedEdgeResultType::OriginConnectError,
            "OriginContentRangeLengthError" => {
                DetailedEdgeResultType::OriginContentRangeLengthError
            }
            "OriginDnsError" => DetailedEdgeResultType::OriginDnsError,
            "OriginError" => DetailedEdgeResultType::OriginError,
            "OriginHeaderTooBigError" => DetailedEdgeResultType::OriginHeaderTooBigError,
            "OriginInvalidResponseError" => DetailedEdgeResultType::OriginInvalidResponseError,
            "OriginReadError" => DetailedEdgeResultType::OriginReadError,
            "OriginWriteError" => DetailedEdgeResultType::OriginWriteError,
            "OriginZeroSizeObjectError" => DetailedEdgeResultType::OriginZeroSizeObjectError,
            "SlowReaderOriginError" => DetailedEdgeResultType::SlowReaderOriginError,
            _ => DetailedEdgeResultType::Other(s.to_string()),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum CsProtcol {
    Http,
    Https,
    Ws,
    Wss,
}

impl FromStr for CsProtcol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "http" => CsProtcol::Http,
            "https" => CsProtcol::Https,
            "ws" => CsProtcol::Ws,
            "wss" => CsProtcol::Wss,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum CsProtcolVersion {
    HTTP3_0,
    HTTP2_0,
    HTTP1_1,
    HTTP1_0,
    HTTP0_9,
}

impl FromStr for CsProtcolVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "HTTP/3.0" => CsProtcolVersion::HTTP3_0,
            "HTTP/2.0" => CsProtcolVersion::HTTP2_0,
            "HTTP/1.1" => CsProtcolVersion::HTTP1_1,
            "HTTP/1.0" => CsProtcolVersion::HTTP1_0,
            "HTTP/0.9" => CsProtcolVersion::HTTP0_9,
            _ => return Err(()),
        })
    }
}

// <https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/secure-connections-supported-viewer-protocols-ciphers.html>

#[derive(Debug, PartialEq)]
pub enum SslProtocol {
    TLSv1_3,
    TLSv1_2,
    TLSv1_1,
    TLSv1_0,
    SSLv3,
}

impl FromStr for SslProtocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "TLSv1.3" => SslProtocol::TLSv1_3,
            "TLSv1.2" => SslProtocol::TLSv1_2,
            "TLSv1.1" => SslProtocol::TLSv1_1,
            "TLSv1" => SslProtocol::TLSv1_0,
            "SSLv3" => SslProtocol::SSLv3,
            _ => return Err(()),
        })
    }
}
