use crate::*;
use ::parquet::{record::RecordWriter, schema::parser::parse_message_type};
use std::net::{Ipv4Addr, Ipv6Addr};
use time::macros::{date, time};

const AWS_DOCS_EXAMPLE: &str = r#"#Version: 1.0
#Fields: date time x-edge-location sc-bytes c-ip cs-method cs(Host) cs-uri-stem sc-status cs(Referer) cs(User-Agent) cs-uri-query cs(Cookie) x-edge-result-type x-edge-request-id x-host-header cs-protocol cs-bytes time-taken x-forwarded-for ssl-protocol ssl-cipher x-edge-response-result-type cs-protocol-version fle-status fle-encrypted-fields c-port time-to-first-byte x-edge-detailed-result-type sc-content-type sc-content-len sc-range-start sc-range-end
2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-
2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	k6WGMNkEzR5BEM_SaF47gjtX9zBDO2m349OY2an0QPEaUum1ZOLrow==	d111111abcdef8.cloudfront.net	https	23	0.000	1.2.3.4, 5.6.7.8, 9.10.11.12	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.000	Hit	text/html	78	-	-
2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	f37nTMVvnKvV2ZSvEsivup_c2kZ7VXzYdjC-GUQZ5qNs-89BlWazbw==	d111111abcdef8.cloudfront.net	https	23	0.001	24.62.90.247,\x2010.89.113.224, 10.89.112.141	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-
2019-12-13	22:36:27	SEA19-C1	900	192.0.2.200	GET	d111111abcdef8.cloudfront.net	/favicon.ico	502	http://www.example.com/	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Error	1pkpNfBQ39sYMnjjUQjmH2w1wdJnbHYTbag21o_3OfcQgPzdL2RSSQ==	www.example.com	http	675	0.102	-	-	-	Error	HTTP/1.1	-	-	25260	0.102	OriginDnsError	text/html	507	-	-
2019-12-13	22:36:26	SEA19-C1	900	192.0.2.200	GET	d111111abcdef8.cloudfront.net	/	502	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Error	3AqrZGCnF_g0-5KOvfA7c9XLcf4YGvMFSeFdIetR1N_2y8jSis8Zxg==	www.example.com	http	735	0.107	-	-	-	Error	HTTP/1.1	-	-	3802	0.107	OriginDnsError	text/html	507	-	-
2019-12-13	22:37:02	SEA19-C2	900	192.0.2.200	GET	d111111abcdef8.cloudfront.net	/	502	-	curl/7.55.1	-	-	Error	kBkDzGnceVtWHqSCqBUqtA_cEs2T3tFUBbnBNkB9El_uVRhHgcZfcw==	www.example.com	http	387	0.103	-	-	-	Error	HTTP/1.1	-	-	12644	0.103	OriginDnsError	text/html	507	-	-"#;

const SINGLE_LOG_LINE: &str = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	1.2.3.4, 5.6.7.8,\x209.10.11.12	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";

#[test]
fn readme_examples_borrow() {
    let logline: &str = SINGLE_LOG_LINE;
    let item = ValidatedRawLogline::try_from(logline).unwrap();

    assert_eq!(item.date, "2019-12-04");
    assert_eq!(item.sc_bytes, "392");
    assert_eq!(item.c_ip, "192.0.2.100");
}

#[test]
fn readme_examples_simple() {
    let logline: &str = SINGLE_LOG_LINE;
    let item = ValidatedSimpleLogline::try_from(logline).unwrap();

    assert_eq!(item.date, "2019-12-04");
    assert_eq!(item.sc_content_len, Some(78));
    assert_eq!(item.c_ip, IpAddr::V4(Ipv4Addr::new(192, 0, 2, 100)));
}

#[test]
fn readme_examples_typed() {
    use time::macros::{date, time}; // just for the example

    let logline: &str = SINGLE_LOG_LINE;
    let item = ValidatedTimeLogline::try_from(logline).unwrap();

    assert_eq!(item.date, date!(2019 - 12 - 04));
    assert_eq!(item.time, time!(21:02:31));
    assert_eq!(item.time_taken, Duration::from_millis(1));
}

#[test]
fn transformation_roundtrip() {
    let checked_line = CheckedRawLogLine::try_from(SINGLE_LOG_LINE).unwrap();
    let simple_line = SimpleLogLine::try_from(checked_line).unwrap();
    let typed_line = TypedLogLine::try_from(checked_line).unwrap();

    assert_eq!(simple_line.sc_bytes, typed_line.sc_bytes);
    assert_eq!(simple_line.cs_host, typed_line.cs_host);
    assert_eq!(simple_line.c_ip, typed_line.c_ip);
    assert_eq!(simple_line.x_forwarded_for, typed_line.x_forwarded_for);
}

// === new structure ===

// note: this also tests the underlying borrowed types
#[test]
fn self_referentially_owned_types() {
    let logline: &str = SINGLE_LOG_LINE;
    let item = OwningValidatedParquetLogline::try_from(logline).unwrap();
    let view = item.view();

    assert_eq!(view.date, NaiveDate::from_ymd_opt(2019, 12, 4).unwrap());
    assert_eq!(view.time, "21:02:31");
    assert_eq!(view.time_taken, 0.001f64);
    assert_eq!(view.sc_bytes, 392u64);
    assert_eq!(view.cs_protocol, "https");
    assert_eq!(view.x_forwarded_for, Some("1.2.3.4, 5.6.7.8, 9.10.11.12"));
}

#[test]
fn owned_types() {
    let logline: &str = SINGLE_LOG_LINE;
    let item = OwnedValidatedParquetLogline::try_from(logline).unwrap();

    assert_eq!(item.date, NaiveDate::from_ymd_opt(2019, 12, 4).unwrap());
    assert_eq!(item.time, "21:02:31");
    assert_eq!(item.time_taken, 0.001f64);
    assert_eq!(item.sc_bytes, 392u64);
    assert_eq!(item.cs_protocol, "https");
    assert_eq!(
        item.x_forwarded_for,
        Some(String::from("1.2.3.4, 5.6.7.8, 9.10.11.12"))
    );
}

#[test]
fn derived_parquet_schema() {
    let sample: &str = AWS_DOCS_EXAMPLE;
    let rows = sample
        .lines()
        .filter_map(|l| OwnedValidatedParquetLogline::try_from(l).ok())
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 6); // adjust to actual line count
    let schema = (&rows.as_slice()).schema().unwrap();
    // print_schema(&mut std::io::stdout(), &schema);
    assert_eq!(schema.name(), "rust_schema");
}

#[test]
fn validate_parquet_schema_v0() {
    let schema = parquet_schemata::V0;
    let schema_t = parse_message_type(schema).unwrap();
    assert_eq!(schema_t.name(), "rust_schema");
}

#[test]
fn validate_parquet_schema_v1() {
    let schema = parquet_schemata::V1;
    let schema_t = parse_message_type(schema).unwrap();
    assert_eq!(schema_t.name(), "rust_schema");
}

#[test]
fn result_type_from_str_checks() {
    let input = "Hit";
    assert_eq!(
        EdgeResultType::try_from(input).unwrap(),
        EdgeResultType::Hit
    );

    let input = "Unknown";
    assert_eq!(
        EdgeResultType::try_from(input).unwrap(),
        EdgeResultType::Other(String::from(input))
    );
}

#[test]
fn result_type_to_str_checks() {
    let input = EdgeResultType::Hit;
    assert_eq!(input.to_string(), "Hit");

    let input = EdgeResultType::Other(String::from("Unknown"));
    assert_eq!(input.to_string(), "Unknown");
}

#[test]
fn x_forwarded_for_addrs_checks() {
    let input = "1.2.3.4";
    let addrs = ForwardedForAddrs::try_from(input).unwrap();
    assert_eq!(addrs.0.len(), 1);
    assert_eq!(
        addrs.0.first(),
        Some(&Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))))
    );

    // use both regular spaces as well as escaped spaces
    // note: CloudFront logs use escaped strings for X-Forwarded-For IP lists 🧐
    let input = "1.2.3.4, 5.6.7.8,\\x209.10.11.12";
    let addrs = ForwardedForAddrs::try_from(input).unwrap();
    assert_eq!(addrs.0.len(), 3);
    assert_eq!(
        addrs.0.first(),
        Some(&Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))))
    );
    assert_eq!(
        addrs.0.get(1),
        Some(&Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(5, 6, 7, 8))))
    );
    assert_eq!(
        addrs.0.get(2),
        Some(&Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(9, 10, 11, 12))))
    );

    // mixed input IPv4 and IPv6; no space after comma
    let input = "1.2.3.4,2001:db8:85a3:8d3:1319:8a2e:370:7348";
    let addrs = ForwardedForAddrs::try_from(input).unwrap();
    assert_eq!(addrs.0.len(), 2);
    assert_eq!(
        addrs.0.first(),
        Some(&Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))))
    );
    assert_eq!(
        addrs.0.get(1),
        Some(&Addressable::IpAddr(IpAddr::V6(Ipv6Addr::new(
            0x2001, 0xdb8, 0x85a3, 0x8d3, 0x1319, 0x8a2e, 0x370, 0x7348
        ))))
    );

    // mixed input of IPv4 address and socket address
    // note: CloudFront sometimes loves to add some port numbers to the IPs 🤬
    let input = "1.2.3.4, 5.6.7.8:6969, 9.10.11.12, 2001:db8:85a3:8d3:1319:8a2e:370:7348";
    let addrs = ForwardedForAddrs::try_from(input).unwrap();
    assert_eq!(addrs.0.len(), 4);
    assert_eq!(
        addrs.0.first(),
        Some(&Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))))
    );
    assert_eq!(
        addrs.0.get(1),
        Some(&Addressable::Socket(std::net::SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(5, 6, 7, 8)),
            6969
        )))
    );
    assert_eq!(
        addrs.0.get(2),
        Some(&Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(9, 10, 11, 12))))
    );
    assert_eq!(
        addrs.0.get(3),
        Some(&Addressable::IpAddr(IpAddr::V6(Ipv6Addr::new(
            0x2001, 0xdb8, 0x85a3, 0x8d3, 0x1319, 0x8a2e, 0x370, 0x7348
        ))))
    );

    // SPECIAL CASE: CloudFront throws "unknown" on us 🤔
    let input = "unknown";
    let addrs = ForwardedForAddrs::try_from(input).unwrap();
    assert_eq!(addrs.0.len(), 1);
    assert_eq!(addrs.0.first(), Some(&Addressable::Unknown));

    let input = "unknown,1.2.3.4";
    let addrs = ForwardedForAddrs::try_from(input).unwrap();
    assert_eq!(addrs.0.len(), 2);
    assert_eq!(addrs.0.first(), Some(&Addressable::Unknown));
    assert_eq!(
        addrs.0.get(1),
        Some(&Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))))
    );

    // SPECIAL CASE: CloudFront captures IPs with leading zeros 🤬
    let input = "0123.045.067.089";
    let addrs = ForwardedForAddrs::try_from(input).unwrap();
    assert_eq!(addrs.0.len(), 1);
    assert_eq!(
        addrs.0.first(),
        Some(&Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(123, 45, 67, 89))))
    );
}
