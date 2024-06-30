pub(crate) use std::{
    marker::PhantomData,
    net::IpAddr,
    sync::Arc,
    time::Duration,
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

#[derive(Debug, Clone, PartialEq, strum::Display, strum::EnumString)]
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

#[derive(Debug, Clone, PartialEq, strum::Display, strum::EnumString)]
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

#[derive(Debug, Clone, Copy, PartialEq, strum::Display, strum::EnumString)]
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

#[derive(Debug, Clone, Copy, PartialEq, strum::Display, strum::EnumString)]
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

// <https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/secure-connections-supported-viewer-protocols-ciphers.html>

#[derive(Debug, Clone, Copy, PartialEq, strum::Display, strum::EnumString)]
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
