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

Consult the benchmarks (run `./benches.sh`) for a synthetic overview.

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
let item = CheckedRawLogLine::try_from(logline).unwrap();

// fields are only sub-slices from the input and therefore all return &str
assert_eq!(item.date, "2019-12-04");
assert_eq!(item.sc_bytes, "392");
assert_eq!(item.c_ip, "192.0.2.100");

// -- get an owned version --

// parser which only uses types accessible without external dependencies,
// only Rust's core and std library is allowed
let item = SimpleLogLine::try_from(logline).unwrap();

assert_eq!(item.date, "2019-12-04");
assert_eq!(item.sc_content_len, 78);
assert_eq!(item.c_ip, IpAddr::V4(Ipv4Addr::new(192, 0, 2, 100)));

// -- get an owned and typed version --

// parser which also converts some fields via external dependencies,
let item = TypedLogLine::try_from(logline).unwrap();

// here: date and time from the `time` crate
assert_eq!(item.date, time_macros::date!(2019-12-04));
assert_eq!(item.time, time_macros::time!(21:02:31));
assert_eq!(item.time_taken, Duration::from_millis(1));
```

## Benchmark example

The following was run under WSL Ubuntu, on a AMD Ryzen 9 7950X3D 16-Core Processor, 64 GiB RAM machine.

Your own numbers may vary. What's more important are the relative differences of the parser implementations.

```shell
# code under benches/comparison-real-world.rs
RUSTFLAGS="-Ctarget-cpu=native" cargo bench -q --all-features --bench real-world
```

```txt
*** Comparing different parsers for AWS CloudFront logs ***

Parses lines and extracts a few fields, slightly unordered,
this should simulate close to real-world usages.
Timer precision: 10 ns
real_world                   fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ 00 CheckedRawLogLine                    │               │               │               │         │
│  ├─ Line A                 162.3 ns      │ 210.6 ns      │ 167.1 ns      │ 167.8 ns      │ 1000    │ 1000000
│  ├─ Line B                 164.2 ns      │ 275.6 ns      │ 171.8 ns      │ 175.8 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              325.1 ns      │ 398.2 ns      │ 337.4 ns      │ 337.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File            994 ns        │ 1.1 µs        │ 1.024 µs      │ 1.029 µs      │ 1000    │ 1000000
├─ 10 CheckedRawLogLineView                │               │               │               │         │
│  ├─ Line A                 366.6 ns      │ 422.9 ns      │ 376.8 ns      │ 378 ns        │ 1000    │ 1000000
│  ├─ Line B                 358.8 ns      │ 412.5 ns      │ 369 ns        │ 370 ns        │ 1000    │ 1000000
│  ├─ Lines A+B              716.5 ns      │ 888.4 ns      │ 748.2 ns      │ 749.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File            2.178 µs      │ 2.784 µs      │ 2.279 µs      │ 2.279 µs      │ 1000    │ 1000000
├─ 11 SmartRawLogLineView                  │               │               │               │         │
│  ├─ Line A                 287.5 ns      │ 385 ns        │ 298.9 ns      │ 301.3 ns      │ 1000    │ 1000000
│  ├─ Line B                 285.2 ns      │ 401.5 ns      │ 301.5 ns      │ 303 ns        │ 1000    │ 1000000
│  ├─ Lines A+B              556.7 ns      │ 680.3 ns      │ 594.7 ns      │ 595.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File            1.694 µs      │ 2.671 µs      │ 1.789 µs      │ 1.796 µs      │ 1000    │ 1000000
├─ 20 SimpleLogLine                        │               │               │               │         │
│  ├─ Line A                 355.2 ns      │ 432 ns        │ 370.8 ns      │ 372.9 ns      │ 1000    │ 1000000
│  ├─ Line B                 347.8 ns      │ 533.7 ns      │ 370.7 ns      │ 373.5 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              715.5 ns      │ 883.4 ns      │ 752 ns        │ 753.9 ns      │ 1000    │ 1000000
│  ╰─ Sample File            2.136 µs      │ 3.085 µs      │ 2.236 µs      │ 2.247 µs      │ 1000    │ 1000000
╰─ 21 TypedLogLine                         │               │               │               │         │
   ├─ Line A                 395.5 ns      │ 467.9 ns      │ 407.9 ns      │ 409.6 ns      │ 1000    │ 1000000
   ├─ Line B                 387.8 ns      │ 512.7 ns      │ 397.1 ns      │ 399.5 ns      │ 1000    │ 1000000
   ├─ Lines A+B              781 ns        │ 1.164 µs      │ 812.2 ns      │ 813.6 ns      │ 1000    │ 1000000
   ╰─ Sample File            2.317 µs      │ 3.551 µs      │ 2.384 µs      │ 2.409 µs      │ 1000    │ 1000000
```

There are more benches you can run, like `single-field` and `two-fields` which should highlight where the "View" parsers shine.

## Safety

This crate uses ``#![forbid(unsafe_code)]`` to ensure everything is implemented in 100% Safe Rust.

<sup>(See `[lints.rust]` section in `Cargo.toml`)</sup>

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
