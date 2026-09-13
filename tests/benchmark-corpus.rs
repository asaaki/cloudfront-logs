#[path = "../benches/corpus.rs"]
mod corpus;

use cloudfront_logs::{ValidatedRawLogline, ValidatedSimpleLogline};
use corpus::{PROFILES, Profile, generate};

#[test]
fn generated_profiles_are_valid_for_every_available_representation() {
    for profile in PROFILES {
        let corpus = generate(profile, 300);
        assert_eq!(corpus.bytes, corpus.lines.iter().map(String::len).sum());
        for text in &corpus.lines {
            let raw = ValidatedRawLogline::try_from(text.as_str()).unwrap();
            let parsed = ValidatedSimpleLogline::try_from(raw).unwrap();
            assert!(parsed.sc_status >= 200 && parsed.sc_status < 300);
            cloudfront_logs::owned::ValidatedSimpleLogline::try_from(text.as_str()).unwrap();
            cloudfront_logs::referential::ValidatedSimpleLogline::try_from(text.as_str()).unwrap();
            #[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
            let _ = cloudfront_logs::ValidatedTypedLogline::try_from(text.as_str()).unwrap();
            #[cfg(feature = "parquet")]
            let _ = cloudfront_logs::ValidatedParquetLogline::try_from(text.as_str()).unwrap();
        }
    }
}

#[test]
fn deterministic_selection_has_the_advertised_rates() {
    let first = generate(Profile::Mixed, 1000);
    assert_eq!(first.lines, generate(Profile::Mixed, 1000).lines);
    for (rate, expected) in [(0, 0), (1, 10), (10, 100), (50, 500), (100, 1000)] {
        let selected = first
            .lines
            .iter()
            .filter(|line| {
                ValidatedRawLogline::try_from(line.as_str())
                    .unwrap()
                    .sc_status
                    .parse::<u16>()
                    .unwrap()
                    < 200 + rate
            })
            .count();
        assert_eq!(selected, expected);
    }
    let parsed: Vec<_> = first
        .lines
        .iter()
        .map(|line| ValidatedSimpleLogline::try_from(line.as_str()).unwrap())
        .collect();
    assert!(parsed.iter().any(|line| line.x_forwarded_for.is_some()));
    assert!(parsed.iter().any(|line| line.c_ip.is_ipv6()));
    assert!(parsed.iter().any(|line| line.cs_cookie.is_some()));
    assert!(parsed.iter().any(|line| line.cs_cookie.is_none()));
    assert!(parsed.iter().any(|line| matches!(
        line.x_edge_result_type,
        cloudfront_logs::EdgeResultType::Other(_)
    )));
    assert!(first.lines.iter().any(|line| line.len() > 4000));
}
