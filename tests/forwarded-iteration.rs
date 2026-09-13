use cloudfront_logs::{Addressable, ForwardedForAddrs};
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn leading_zero_compatibility_preserves_four_checked_octets() {
    for (text, expected) in [
        ("0123.045.067.089", [123, 45, 67, 89]),
        ("0.0.0.0", [0, 0, 0, 0]),
        ("000000001.002.003.004", [1, 2, 3, 4]),
    ] {
        assert_eq!(
            Addressable::try_from(text).unwrap(),
            Addressable::IpAddr(IpAddr::V4(Ipv4Addr::from(expected)))
        );
    }
    for text in [
        "01.2.3",
        "01.2.3.4.5",
        "01..3.4",
        "01.2.3.",
        "01.2.3.256",
        "01.2.3.-1",
        "01.2.3. 4",
        "01.2.3.四",
        "01.2.3.4:80",
    ] {
        assert_eq!(
            Addressable::try_from(text),
            Err("invalid X-Forwarded-For IP/socket address")
        );
    }
}

#[test]
fn lazy_iteration_agrees_with_owned_parsing_including_error_semantics() {
    for input in [
        "192.0.2.1",
        "2001:db8::1",
        "192.0.2.1:443",
        "[2001:db8::1]:443",
        "unknown",
        "0123.045.067.089",
        "192.0.2.1,\\x202001:db8::1, unknown",
        " 192.0.2.1 , 192.0.2.2:80 ",
        "",
        "-",
        ",",
        "192.0.2.1,",
        "192.0.2.1,broken",
        "\\x20\\x20192.0.2.1",
        "unknown,未知",
    ] {
        let collected = ForwardedForAddrs::iter_str(input).collect::<Result<Vec<_>, _>>();
        assert_eq!(
            collected,
            ForwardedForAddrs::try_from(input).map(|v| v.0),
            "{input:?}"
        );
    }
}

#[test]
fn selected_address_does_not_validate_remaining_addresses() {
    let mut iter = ForwardedForAddrs::iter_str("192.0.2.1,broken,unknown");
    assert_eq!(
        iter.next(),
        Some(Ok(Addressable::IpAddr("192.0.2.1".parse().unwrap())))
    );
    assert_eq!(iter.next(), Some(Err("invalid X-Forwarded-For IP(s)")));
    assert_eq!(iter.next(), Some(Ok(Addressable::Unknown)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}
