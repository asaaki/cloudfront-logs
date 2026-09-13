# cloudfront-logs

A Rust-based AWS CloudFront log line parser

## Log format

The AWS CloudFront log file format is described here:

<https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html#LogFileFormat>

## Purpose and Design

This parser currently focuses on parsing a single line of a log file only.
It provides are structured view to those tab-separated field items and avoids fiddling with numeric indices.

It's up to the user of this library to pass the individual lines to the parser.
This makes it very flexible to use in different scenarios as there are no assumptions about where those log lines come from and how they pass through the program.

It's possible that in the future more utilities get added, but as of now it's more important to deliver a fast and reliable parsing functionality.

The library therefore serves different parser implementation, so you can pick the one for your use cases and needs.

Run `just bench` for a synthetic overview. The command writes `benchmarks/<platform>.md`.

## Example

Given the following log line:

```log
2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-
```

You have different options to proces this line:

```rust
use cloudfront_logs::*;
use std::net::{IpAddr, Ipv4Addr};

let logline: &str = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";

// -- borrowing the input --

// reasonable default parser
let item = ValidatedRawLogline::try_from(logline).unwrap();

// fields are only sub-slices from the input and therefore all return &str
assert_eq!(item.date, "2019-12-04");
assert_eq!(item.sc_bytes, "392");
assert_eq!(item.c_ip, "192.0.2.100");

// -- parse structured fields without a date/time backend --

// parser which only uses types accessible without external dependencies,
// only Rust's core and std library is allowed
let item = ValidatedSimpleLogline::try_from(logline).unwrap();

assert_eq!(item.date, "2019-12-04");
assert_eq!(item.sc_content_len, Some(78));
assert_eq!(item.c_ip, IpAddr::V4(Ipv4Addr::new(192, 0, 2, 100)));

// -- parse structured fields with typed date and time values --

// parser which also converts some fields via external dependencies,
let item = ValidatedTypedLogline::try_from(logline).unwrap();
let datetime = item.datetime();
```

The `jiff` date/time backend is enabled by default. To use another backend,
disable default features and select exactly one backend:

```toml
cloudfront-logs = { version = "0.10", default-features = false, features = ["time"] }
# or
cloudfront-logs = { version = "0.10", default-features = false, features = ["chrono"] }
```

## Benchmark example

Run `just bench`. The command writes `benchmarks/<platform>.md`.
See [BENCHMARK.md](BENCHMARK.md) for benchmark configurations, platform results, and historical reference data.

## Safety

This crate uses ``#![forbid(unsafe_code)]`` to ensure everything is implemented in 100% Safe Rust.

## Processing selected fields and streams

Raw records expose named strings and fallible `parse_*` methods for selected fields:

```rust
fn error_response_bytes(line: &str) -> Result<u64, &'static str> {
    let raw = cloudfront_logs::ValidatedRawLogline::try_from(line)?;
    if raw.parse_sc_status()? >= 400 {
        raw.parse_sc_bytes()
    } else {
        Ok(0)
    }
}
```

Raw validation checks record structure. Each accessor checks only its own value.
Use full structured conversion when all typed fields need validation.
Simple conversion leaves dates and times as strings. Typed conversion also parses dates and times.
See [selected-fields.rs](examples/selected-fields.rs) for filtering and aggregation with error propagation.

The [stream example](examples/stream.rs) processes borrowed records through a callback and reuses a bounded input buffer.
It handles comments, CRLF, line numbers, errors, and early termination.
The library still accepts individual lines and does not choose an input source or decompressor.
See [the streaming report](docs/performance/task-8-streaming.md) for retention rules and memory limits.

For repeated sharing, wrap a referential record in `Arc` to reuse its parsed values.
See [shared-loglines.rs](examples/shared-loglines.rs) for ownership and input extraction.
The default record remains unchanged because the extra `Arc` costs time and memory without clones in the measured workload.

The [performance investigation results](docs/performance/RESULTS.md) describe measured improvements, rejected prototypes, and platform limits.

## License

<sup>
Licensed under either of
  <a href="https://raw.githubusercontent.com/asaaki/cloudfront-logs/main/LICENSE-APACHE">Apache License, Version 2.0</a> or
  <a href="https://raw.githubusercontent.com/asaaki/cloudfront-logs/main/LICENSE-MIT">MIT license</a>
at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

<!-- links -->
