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
│  ├─ Line A                 220 ns        │ 1.062 µs      │ 226.5 ns      │ 228 ns        │ 1000    │ 1000000
│  ├─ Line B                 217.3 ns      │ 273.6 ns      │ 224.1 ns      │ 225 ns        │ 1000    │ 1000000
│  ├─ Lines A+B              434.4 ns      │ 556.7 ns      │ 448.8 ns      │ 449.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File            1.351 µs      │ 3.054 µs      │ 1.409 µs      │ 1.42 µs       │ 1000    │ 1000000
├─ 01 UnsafeRawLogLine                     │               │               │               │         │
│  ├─ Line A                 195.6 ns      │ 319.7 ns      │ 202.2 ns      │ 204 ns        │ 1000    │ 1000000
│  ├─ Line B                 195.7 ns      │ 254 ns        │ 203.8 ns      │ 204.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              388.8 ns      │ 1.097 µs      │ 412.7 ns      │ 436.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File            1.235 µs      │ 1.983 µs      │ 1.29 µs       │ 1.292 µs      │ 1000    │ 1000000
├─ 10 CheckedRawLogLineView                │               │               │               │         │
│  ├─ Line A                 486.5 ns      │ 622.9 ns      │ 508.2 ns      │ 509.3 ns      │ 1000    │ 1000000
│  ├─ Line B                 484.2 ns      │ 891.6 ns      │ 498.7 ns      │ 502.1 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              975.7 ns      │ 1.539 µs      │ 1.009 µs      │ 1.013 µs      │ 1000    │ 1000000
│  ╰─ Sample File            3.083 µs      │ 7.572 µs      │ 3.212 µs      │ 3.228 µs      │ 1000    │ 1000000
├─ 11 SmartRawLogLineView                  │               │               │               │         │
│  ├─ Line A                 402.1 ns      │ 729.9 ns      │ 418.3 ns      │ 424.5 ns      │ 1000    │ 1000000
│  ├─ Line B                 393 ns        │ 504.6 ns      │ 403.8 ns      │ 405.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              811.5 ns      │ 981.2 ns      │ 841.2 ns      │ 843.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File            2.563 µs      │ 3.17 µs       │ 2.66 µs       │ 2.662 µs      │ 1000    │ 1000000
├─ 20 SimpleLogLine                        │               │               │               │         │
│  ├─ Line A                 599.1 ns      │ 729.3 ns      │ 630 ns        │ 631.3 ns      │ 1000    │ 1000000
│  ├─ Line B                 567.9 ns      │ 952.5 ns      │ 594.5 ns      │ 599.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              1.213 µs      │ 1.906 µs      │ 1.26 µs       │ 1.262 µs      │ 1000    │ 1000000
│  ╰─ Sample File            3.708 µs      │ 4.643 µs      │ 3.863 µs      │ 3.867 µs      │ 1000    │ 1000000
╰─ 21 TypedLogLine                         │               │               │               │         │
   ├─ Line A                 480.1 ns      │ 655.7 ns      │ 503.2 ns      │ 505.9 ns      │ 1000    │ 1000000
   ├─ Line B                 471.5 ns      │ 765.4 ns      │ 491.8 ns      │ 495.2 ns      │ 1000    │ 1000000
   ├─ Lines A+B              943.1 ns      │ 1.492 µs      │ 984.9 ns      │ 986.9 ns      │ 1000    │ 1000000
   ╰─ Sample File            2.845 µs      │ 4.171 µs      │ 2.955 µs      │ 2.956 µs      │ 1000    │ 1000000
```

There are more benches you can run, like `single-field` and `two-fields` which should highlight where the "View" parsers shine.

## Safety

This crate includes optional features which utilise `unsafe` Rust code.

Those parts are usually for performance reasons and implementing features not present in the standard library.

The crate is generally usable without those unsafe features.

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
