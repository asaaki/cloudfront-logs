use cloudfront_logs::{borrowed, types::*};
use std::{net::IpAddr, time::Duration};

const LINE: &str = "2019-12-04\t21:02:31\tLAX1\t392\t192.0.2.100\tGET\td.example\t/\t200\t-\tagent\t-\t-\tHit\tid\td.example\thttps\t23\t0.001\t-\tTLSv1.2\tcipher\tHit\tHTTP/2.0\t-\t-\t11040\t0.001\tHit\ttext/html\t78\t-\t-";

#[test]
fn selected_fields_match_full_conversion() {
    let mut raw = borrowed::ValidatedRawLogline::try_from(LINE).unwrap();
    raw.time_to_first_byte = "0.002";
    raw.x_edge_response_result_type = "Miss";
    raw.x_edge_detailed_result_type = "Error";
    let parsed = borrowed::ValidatedSimpleLogline::try_from(raw).unwrap();
    macro_rules! same {
        ($($method:ident => $field:ident),* $(,)?) => { $(
            assert_eq!(raw.$method().unwrap(), parsed.$field);
        )* };
    }
    same!(
        parse_sc_bytes => sc_bytes, parse_c_ip => c_ip,
        parse_sc_status => sc_status, parse_x_edge_result_type => x_edge_result_type,
        parse_cs_protocol => cs_protocol, parse_cs_bytes => cs_bytes,
        parse_time_taken => time_taken, parse_x_forwarded_for => x_forwarded_for,
        parse_ssl_protocol => ssl_protocol,
        parse_x_edge_response_result_type => x_edge_response_result_type,
        parse_cs_protocol_version => cs_protocol_version,
        parse_fle_encrypted_fields => fle_encrypted_fields, parse_c_port => c_port,
        parse_time_to_first_byte => time_to_first_byte,
        parse_x_edge_detailed_result_type => x_edge_detailed_result_type,
        parse_sc_content_len => sc_content_len, parse_sc_range_start => sc_range_start,
        parse_sc_range_end => sc_range_end,
    );
    assert_eq!(raw.parse_sc_status(), Ok(200));
    assert_eq!(raw.parse_sc_bytes(), Ok(392));
    assert_eq!(raw.parse_cs_bytes(), Ok(23));
    assert_eq!(raw.parse_c_port(), Ok(11040));
    assert_eq!(raw.parse_sc_content_len(), Ok(Some(78)));
    assert_eq!(raw.parse_cs_protocol(), Ok(CsProtocol::Https));
    assert_eq!(
        raw.parse_cs_protocol_version(),
        Ok(CsProtocolVersion::HTTP2_0)
    );
    assert_eq!(raw.parse_ssl_protocol(), Ok(Some(SslProtocol::TLSv1_2)));
    assert_eq!(raw.parse_x_edge_result_type(), Ok(EdgeResultType::Hit));
    assert_eq!(
        raw.parse_x_edge_response_result_type(),
        Ok(EdgeResultType::Miss)
    );
    assert_eq!(
        raw.parse_x_edge_detailed_result_type(),
        Ok(DetailedEdgeResultType::Error)
    );
    assert_eq!(raw.parse_time_taken(), Ok(Duration::from_millis(1)));
    assert_eq!(raw.parse_time_to_first_byte(), Ok(Duration::from_millis(2)));
}

#[test]
fn selected_conversion_checks_only_the_current_field() {
    let mut raw = borrowed::ValidatedRawLogline::try_from(LINE).unwrap();
    raw.c_ip = "broken";
    raw.date = "broken";
    raw.time_taken = "NaN";
    assert_eq!(raw.parse_sc_status(), Ok(200));
    assert_eq!(raw.parse_sc_bytes(), Ok(392));
    assert_eq!(raw.parse_c_ip(), Err("c_ip invalid"));
    assert_eq!(raw.parse_time_taken(), Err("time_taken invalid"));
    assert!(borrowed::ValidatedSimpleLogline::try_from(raw).is_err());
    raw.sc_status = "65536";
    assert_eq!(raw.parse_sc_status(), Err("sc_status invalid"));
    raw.sc_status = "404";
    assert_eq!(raw.parse_sc_status(), Ok(404));
    let raw: borrowed::UnvalidatedRawLogline<'_> = raw.into();
    assert_eq!(raw.parse_sc_status(), Ok(404));
    let raw: borrowed::ValidatedRawLogline<'_> = raw.into();
    assert_eq!(raw.parse_time_taken(), Err("time_taken invalid"));
}

#[test]
fn selected_errors_agree_with_full_parsing() {
    macro_rules! invalid {
        ($field:ident, $method:ident, $value:expr, $error:literal) => {{
            let mut raw = borrowed::ValidatedRawLogline::try_from(LINE).unwrap();
            raw.$field = $value;
            assert_eq!(raw.$method().unwrap_err(), $error);
            assert_eq!(
                borrowed::ValidatedSimpleLogline::try_from(raw).unwrap_err(),
                $error
            );
        }};
    }
    for value in ["", "invalid", "184467440737095516160"] {
        invalid!(sc_bytes, parse_sc_bytes, value, "sc_bytes invalid");
        invalid!(sc_status, parse_sc_status, value, "sc_status invalid");
        invalid!(cs_bytes, parse_cs_bytes, value, "cs_bytes invalid");
        invalid!(c_port, parse_c_port, value, "c_port invalid");
        invalid!(
            fle_encrypted_fields,
            parse_fle_encrypted_fields,
            value,
            "fle_encrypted_fields invalid"
        );
        invalid!(
            sc_content_len,
            parse_sc_content_len,
            value,
            "sc_content_len invalid"
        );
        invalid!(
            sc_range_start,
            parse_sc_range_start,
            value,
            "sc_range_start invalid"
        );
        invalid!(
            sc_range_end,
            parse_sc_range_end,
            value,
            "sc_range_end invalid"
        );
    }
    for value in ["", "-1", "NaN", "inf", "1e300"] {
        invalid!(time_taken, parse_time_taken, value, "time_taken invalid");
        invalid!(
            time_to_first_byte,
            parse_time_to_first_byte,
            value,
            "time_to_first_byte invalid"
        );
    }
    invalid!(c_ip, parse_c_ip, "999.0.0.1", "c_ip invalid");
    invalid!(
        x_forwarded_for,
        parse_x_forwarded_for,
        "192.0.2.1,broken",
        "x_forwarded_for invalid"
    );
    invalid!(
        cs_protocol,
        parse_cs_protocol,
        "future",
        "cs_protocol invalid"
    );
    invalid!(
        ssl_protocol,
        parse_ssl_protocol,
        "future",
        "ssl_protocol invalid"
    );
    invalid!(
        cs_protocol_version,
        parse_cs_protocol_version,
        "future",
        "cs_protocol_version invalid"
    );
}

#[test]
fn selected_optional_and_unknown_values_preserve_semantics() {
    let mut raw = borrowed::ValidatedRawLogline::try_from(LINE).unwrap();
    raw.sc_content_len = "-";
    assert_eq!(raw.parse_sc_content_len(), Ok(None));
    assert_eq!(raw.parse_fle_encrypted_fields(), Ok(None));
    assert_eq!(raw.parse_sc_range_start(), Ok(None));
    assert_eq!(raw.parse_sc_range_end(), Ok(None));
    raw.sc_range_start = "-12";
    raw.sc_range_end = "24";
    raw.fle_encrypted_fields = "2";
    assert_eq!(raw.parse_sc_range_start(), Ok(Some(-12)));
    assert_eq!(raw.parse_sc_range_end(), Ok(Some(24)));
    assert_eq!(raw.parse_fle_encrypted_fields(), Ok(Some(2)));
    raw.x_edge_result_type = "Future";
    raw.x_edge_response_result_type = "";
    raw.x_edge_detailed_result_type = "FutureDetail";
    assert_eq!(
        raw.parse_x_edge_result_type(),
        Ok(EdgeResultType::Other("Future".into()))
    );
    assert_eq!(
        raw.parse_x_edge_response_result_type(),
        Ok(EdgeResultType::Other("".into()))
    );
    assert_eq!(
        raw.parse_x_edge_detailed_result_type(),
        Ok(DetailedEdgeResultType::Other("FutureDetail".into()))
    );
    raw.ssl_protocol = "-";
    assert_eq!(raw.parse_ssl_protocol(), Ok(None));
    assert_eq!(raw.parse_x_forwarded_for(), Ok(None));
    raw.x_forwarded_for = "001.002.003.004,\\x20[2001:db8::7]:443";
    let addresses = raw.parse_x_forwarded_for().unwrap().unwrap();
    assert_eq!(addresses.0.len(), 2);
    raw.c_ip = "2001:db8::7";
    assert_eq!(
        raw.parse_c_ip(),
        Ok("2001:db8::7".parse::<IpAddr>().unwrap())
    );
    let parsed = borrowed::ValidatedSimpleLogline::try_from(raw).unwrap();
    assert_eq!(parsed.x_forwarded_for, Some(addresses));
    assert_eq!(
        parsed.x_edge_result_type,
        raw.parse_x_edge_result_type().unwrap()
    );
}
