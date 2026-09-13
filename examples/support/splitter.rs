//! Safe benchmark-only alternatives. No parser implementation or API is changed.

use cloudfront_logs::ValidatedRawLogline as Raw;

const FIELD_COUNT: usize = 33;
const TAB_COUNT: usize = FIELD_COUNT - 1;
const FIELD_COUNT_ERROR: &str = "Invalid log line (field count)";

pub trait CheckedParser {
    fn parse<'a>(line: &'a str, template: Raw<'a>) -> Result<Raw<'a>, &'static str>;
}

pub struct Original;
pub struct Boundaries;
pub struct Streaming;

impl CheckedParser for Original {
    fn parse<'a>(line: &'a str, _template: Raw<'a>) -> Result<Raw<'a>, &'static str> {
        Raw::try_from(line)
    }
}

fn check_prefix(line: &str) -> Result<(), &'static str> {
    if line.is_empty() {
        Err("Invalid log line (empty)")
    } else if line.starts_with('#') {
        Err("Invalid log line (comment)")
    } else {
        Ok(())
    }
}

impl CheckedParser for Boundaries {
    fn parse<'a>(line: &'a str, template: Raw<'a>) -> Result<Raw<'a>, &'static str> {
        check_prefix(line)?;
        let mut tabs = memchr::memchr_iter(b'\t', line.as_bytes());
        let mut boundaries = [0_usize; TAB_COUNT];
        for boundary in &mut boundaries {
            *boundary = tabs.next().ok_or(FIELD_COUNT_ERROR)?;
        }
        if tabs.next().is_some() {
            return Err(FIELD_COUNT_ERROR);
        }
        let mut fields = [""; FIELD_COUNT];
        let mut from = 0;
        for (field, to) in fields.iter_mut().zip(boundaries) {
            // An ASCII tab cannot occur inside a UTF-8 code point.
            *field = &line[from..to];
            from = to + 1;
        }
        fields[TAB_COUNT] = &line[from..];
        Ok(record_from_fields(fields, template))
    }
}

impl CheckedParser for Streaming {
    fn parse<'a>(line: &'a str, template: Raw<'a>) -> Result<Raw<'a>, &'static str> {
        check_prefix(line)?;
        let mut tabs = memchr::memchr_iter(b'\t', line.as_bytes());
        let mut fields = [""; FIELD_COUNT];
        let mut from = 0;
        for field in fields.iter_mut().take(TAB_COUNT) {
            let to = tabs.next().ok_or(FIELD_COUNT_ERROR)?;
            *field = &line[from..to];
            from = to + 1;
        }
        if tabs.next().is_some() {
            return Err(FIELD_COUNT_ERROR);
        }
        fields[TAB_COUNT] = &line[from..];
        Ok(record_from_fields(fields, template))
    }
}

/// Produce a valid raw marker once, outside timing, through the public API.
/// Every field is replaced below; the marker itself is never treated as proof.
pub fn template() -> Raw<'static> {
    Raw::try_from(concat!(
        "\t\t\t\t\t\t\t\t",
        "\t\t\t\t\t\t\t\t",
        "\t\t\t\t\t\t\t\t",
        "\t\t\t\t\t\t\t\t"
    ))
    .unwrap()
}

fn record_from_fields<'a>(fields: [&'a str; FIELD_COUNT], mut record: Raw<'a>) -> Raw<'a> {
    // Public field assignment avoids adding a library-only constructor for an
    // experiment. There is no heap storage or fabricated private marker.
    [
        record.date,
        record.time,
        record.x_edge_location,
        record.sc_bytes,
        record.c_ip,
        record.cs_method,
        record.cs_host,
        record.cs_uri_stem,
        record.sc_status,
        record.cs_referer,
        record.cs_user_agent,
        record.cs_uri_query,
        record.cs_cookie,
        record.x_edge_result_type,
        record.x_edge_request_id,
        record.x_host_header,
        record.cs_protocol,
        record.cs_bytes,
        record.time_taken,
        record.x_forwarded_for,
        record.ssl_protocol,
        record.ssl_cipher,
        record.x_edge_response_result_type,
        record.cs_protocol_version,
        record.fle_status,
        record.fle_encrypted_fields,
        record.c_port,
        record.time_to_first_byte,
        record.x_edge_detailed_result_type,
        record.sc_content_type,
        record.sc_content_len,
        record.sc_range_start,
        record.sc_range_end,
    ] = fields;
    record
}

pub fn nominal_storage() -> (usize, usize, usize) {
    (
        std::mem::size_of::<[usize; TAB_COUNT]>(),
        std::mem::size_of::<[&str; FIELD_COUNT]>(),
        std::mem::size_of::<Raw<'_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudfront_logs::ValidatedSimpleLogline;

    fn compare(line: &str) {
        let original = Original::parse(line, template());
        for actual in [
            Boundaries::parse(line, template()),
            Streaming::parse(line, template()),
        ] {
            assert_eq!(actual, original, "input: {line:?}");
            let converted = actual.and_then(ValidatedSimpleLogline::try_from);
            assert_eq!(
                converted,
                ValidatedSimpleLogline::try_from(line),
                "input: {line:?}"
            );
            #[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
            assert_eq!(
                actual.and_then(cloudfront_logs::ValidatedTypedLogline::try_from),
                cloudfront_logs::ValidatedTypedLogline::try_from(line),
                "input: {line:?}"
            );
        }
    }

    #[test]
    fn preserves_utf8_empty_fields_and_structural_error_precedence() {
        for count in 0..=70 {
            let fields: Vec<_> = (0..count).map(|index| format!("資料🙂{index}")).collect();
            compare(&fields.join("\t"));
            compare(&format!("#{}", fields.join("\t")));
        }
        for empty in 0..FIELD_COUNT {
            let mut fields = vec!["é🙂"; FIELD_COUNT];
            fields[empty] = "";
            compare(&fields.join("\t"));
        }
        compare("");
        compare("\t");
        compare(&"\t".repeat(TAB_COUNT));
    }

    #[test]
    fn matches_valid_records_and_numeric_errors_before_or_after_wrong_count() {
        let corpus = crate::corpus::generate(crate::corpus::Profile::Mixed, 300);
        for line in &corpus.lines {
            compare(line);
            let fields: Vec<_> = line.split('\t').collect();
            for (field, bad) in [
                (0, "2025-99-99"),
                (3, "18446744073709551616"),
                (8, "bad"),
                (18, "NaN"),
                (19, "not-an-ip"),
                (27, "-1"),
                (30, "bad"),
            ] {
                let mut malformed = fields.clone();
                malformed[field] = bad;
                compare(&malformed.join("\t"));
                malformed.push("extra");
                compare(&malformed.join("\t"));
                malformed.truncate(31);
                compare(&malformed.join("\t"));
            }
        }
    }
}
