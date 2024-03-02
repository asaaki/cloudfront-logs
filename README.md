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
│  ├─ Line A                 162.5 ns      │ 207.4 ns      │ 165.2 ns      │ 166.3 ns      │ 1000    │ 1000000
│  ├─ Line B                 173.8 ns      │ 220 ns        │ 183 ns        │ 182.7 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              332.6 ns      │ 392.9 ns      │ 343.2 ns      │ 344.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File            983.6 ns      │ 1.119 µs      │ 1.012 µs      │ 1.017 µs      │ 1000    │ 1000000
├─ 01 UnsafeRawLogLine                     │               │               │               │         │
│  ├─ Line A                 167.7 ns      │ 234.9 ns      │ 175.9 ns      │ 175.9 ns      │ 1000    │ 1000000
│  ├─ Line B                 163.5 ns      │ 218.8 ns      │ 168.4 ns      │ 168.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              326.5 ns      │ 371.4 ns      │ 344 ns        │ 343.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File            962.8 ns      │ 1.095 µs      │ 993.1 ns      │ 996.4 ns      │ 1000    │ 1000000
├─ 10 CheckedRawLogLineView                │               │               │               │         │
│  ├─ Line A                 358.7 ns      │ 413.4 ns      │ 368.2 ns      │ 370.1 ns      │ 1000    │ 1000000
│  ├─ Line B                 360.2 ns      │ 423.3 ns      │ 373.4 ns      │ 373.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              716.2 ns      │ 806.4 ns      │ 734.9 ns      │ 739.4 ns      │ 1000    │ 1000000
│  ╰─ Sample File            2.158 µs      │ 2.486 µs      │ 2.236 µs      │ 2.241 µs      │ 1000    │ 1000000
├─ 11 SmartRawLogLineView                  │               │               │               │         │
│  ├─ Line A                 281.4 ns      │ 335.5 ns      │ 289.9 ns      │ 291.3 ns      │ 1000    │ 1000000
│  ├─ Line B                 284 ns        │ 342 ns        │ 296.6 ns      │ 296.4 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              558 ns        │ 945.4 ns      │ 584.5 ns      │ 590.1 ns      │ 1000    │ 1000000
│  ╰─ Sample File            1.689 µs      │ 2.478 µs      │ 1.778 µs      │ 1.779 µs      │ 1000    │ 1000000
├─ 20 SimpleLogLine                        │               │               │               │         │
│  ├─ Line A                 364 ns        │ 439.6 ns      │ 380 ns        │ 381 ns        │ 1000    │ 1000000
│  ├─ Line B                 352.7 ns      │ 406.9 ns      │ 363.2 ns      │ 364 ns        │ 1000    │ 1000000
│  ├─ Lines A+B              723.8 ns      │ 815.6 ns      │ 744.2 ns      │ 746.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File            2.191 µs      │ 3.003 µs      │ 2.244 µs      │ 2.255 µs      │ 1000    │ 1000000
╰─ 21 TypedLogLine                         │               │               │               │         │
   ├─ Line A                 399 ns        │ 459.2 ns      │ 412.2 ns      │ 413.3 ns      │ 1000    │ 1000000
   ├─ Line B                 389.9 ns      │ 465.2 ns      │ 401.6 ns      │ 402.9 ns      │ 1000    │ 1000000
   ├─ Lines A+B              784.1 ns      │ 1.237 µs      │ 813.9 ns      │ 817.3 ns      │ 1000    │ 1000000
   ╰─ Sample File            2.341 µs      │ 2.806 µs      │ 2.407 µs      │ 2.414 µs      │ 1000    │ 1000000
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
