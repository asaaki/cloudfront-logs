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
│  ├─ Line A                 182.8 ns      │ 254.7 ns      │ 188.1 ns      │ 190.9 ns      │ 1000    │ 1000000
│  ├─ Line B                 175.9 ns      │ 249.9 ns      │ 179.9 ns      │ 182.9 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              348.5 ns      │ 418.4 ns      │ 361.3 ns      │ 362.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File            1.019 µs      │ 1.209 µs      │ 1.05 µs       │ 1.061 µs      │ 1000    │ 1000000
├─ 01 UnsafeRawLogLine                     │               │               │               │         │
│  ├─ Line A                 172.4 ns      │ 250.3 ns      │ 177.3 ns      │ 180.3 ns      │ 1000    │ 1000000
│  ├─ Line B                 174 ns        │ 256.8 ns      │ 182.8 ns      │ 186.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              340.4 ns      │ 558.6 ns      │ 357.9 ns      │ 360.8 ns      │ 1000    │ 1000000
│  ╰─ Sample File            997.9 ns      │ 1.442 µs      │ 1.043 µs      │ 1.047 µs      │ 1000    │ 1000000
├─ 10 CheckedRawLogLineView                │               │               │               │         │
│  ├─ Line A                 377.6 ns      │ 448 ns        │ 391.9 ns      │ 393.1 ns      │ 1000    │ 1000000
│  ├─ Line B                 378.4 ns      │ 494.2 ns      │ 395.6 ns      │ 397.2 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              749.9 ns      │ 845.1 ns      │ 782.2 ns      │ 782.7 ns      │ 1000    │ 1000000
│  ╰─ Sample File            2.293 µs      │ 3.49 µs       │ 2.332 µs      │ 2.349 µs      │ 1000    │ 1000000
├─ 11 SmartRawLogLineView                  │               │               │               │         │
│  ├─ Line A                 294.4 ns      │ 355.8 ns      │ 304 ns        │ 305.4 ns      │ 1000    │ 1000000
│  ├─ Line B                 293.1 ns      │ 347.8 ns      │ 298.1 ns      │ 299 ns        │ 1000    │ 1000000
│  ├─ Lines A+B              579.7 ns      │ 661.2 ns      │ 596.4 ns      │ 599.2 ns      │ 1000    │ 1000000
│  ╰─ Sample File            1.752 µs      │ 1.91 µs       │ 1.801 µs      │ 1.806 µs      │ 1000    │ 1000000
├─ 20 SimpleLogLine                        │               │               │               │         │
│  ├─ Line A                 382.5 ns      │ 579.7 ns      │ 395.2 ns      │ 398 ns        │ 1000    │ 1000000
│  ├─ Line B                 370.1 ns      │ 428.7 ns      │ 381.2 ns      │ 383.6 ns      │ 1000    │ 1000000
│  ├─ Lines A+B              768.2 ns      │ 1.126 µs      │ 790.3 ns      │ 793.5 ns      │ 1000    │ 1000000
│  ╰─ Sample File            2.334 µs      │ 2.542 µs      │ 2.406 µs      │ 2.41 µs       │ 1000    │ 1000000
╰─ 21 TypedLogLine                         │               │               │               │         │
   ├─ Line A                 429.2 ns      │ 699.4 ns      │ 436.2 ns      │ 439 ns        │ 1000    │ 1000000
   ├─ Line B                 414.8 ns      │ 502.8 ns      │ 429.5 ns      │ 432.1 ns      │ 1000    │ 1000000
   ├─ Lines A+B              836 ns        │ 1.277 µs      │ 862 ns        │ 867.3 ns      │ 1000    │ 1000000
   ╰─ Sample File            2.52 µs       │ 3.003 µs      │ 2.578 µs      │ 2.593 µs      │ 1000    │ 1000000
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
