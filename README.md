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

Consult the benchmarks (run `./bin/benches.sh` on Linux/macOS or `./bin/benches.ps1` on PowerShell) for a synthetic overview. Use `--doc` (bash) or `-Doc` (PowerShell) to emit a BENCHMARK.md-ready output block.

## Example

Given the following log line:

```log
2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-
```

You have different options to proces this line:

```rust
use cloudfront_logs::*;

let logline: &str = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";

// -- borrowing the input --

// reasonable default parser
let item = ValidatedRawLogline::try_from(logline).unwrap();

// fields are only sub-slices from the input and therefore all return &str
assert_eq!(item.date, "2019-12-04");
assert_eq!(item.sc_bytes, "392");
assert_eq!(item.c_ip, "192.0.2.100");

// -- get an owned version --

// parser which only uses types accessible without external dependencies,
// only Rust's core and std library is allowed
let item = ValidatedSimpleLogline::try_from(logline).unwrap();

assert_eq!(item.date, "2019-12-04");
assert_eq!(item.sc_content_len, 78);
assert_eq!(item.c_ip, IpAddr::V4(Ipv4Addr::new(192, 0, 2, 100)));

// -- get an owned and typed version --

// parser which also converts some fields via external dependencies,
let item = ValidatedTimeLogline::try_from(logline).unwrap();

// here: date and time from the `time` crate
assert_eq!(item.date, time_macros::date!(2019-12-04));
assert_eq!(item.time, time_macros::time!(21:02:31));
assert_eq!(item.time_taken, Duration::from_millis(1));
```

## Benchmark example

See [BENCHMARK.md](BENCHMARK.md) for benchmark setup, sample output, and additional benchmark notes.

## Safety

This crate uses ``#![forbid(unsafe_code)]`` to ensure everything is implemented in 100% Safe Rust.

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
