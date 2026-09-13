//! Conditional-allocation experiments. See docs/performance/task-7-allocations.md.

#![forbid(unsafe_code)]

use cloudfront_logs::{Addressable, DetailedEdgeResultType, EdgeResultType, ForwardedForAddrs};
use divan::{Bencher, black_box};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[cfg(feature = "bench-alloc")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

// Exact baseline algorithm from revision 099023f, retained only for measurement.
fn legacy_address(input: &str) -> Result<Addressable, &'static str> {
    if input == "unknown" {
        return Ok(Addressable::Unknown);
    }
    if let Ok(ip) = input.parse::<IpAddr>() {
        return Ok(Addressable::IpAddr(ip));
    } else if input.starts_with('0') && input.contains('.') {
        let octets = input
            .splitn(4, '.')
            .filter_map(|s| s.parse::<u8>().ok())
            .collect::<Vec<u8>>();
        if octets.len() == 4 {
            return Ok(Addressable::IpAddr(IpAddr::V4(Ipv4Addr::new(
                octets.first().copied().unwrap_or(0),
                octets.get(1).copied().unwrap_or(0),
                octets.get(2).copied().unwrap_or(0),
                octets.get(3).copied().unwrap_or(0),
            ))));
        }
    }
    input
        .parse::<SocketAddr>()
        .map(Addressable::Socket)
        .map_err(|_e| "invalid X-Forwarded-For IP/socket address")
}

const ADDRESSES: &[&str] = &["192.0.2.1", "2001:db8::1", "0123.045.067.089", "01.2.3.256"];

#[divan::bench(args = ADDRESSES)]
fn old_address(input: &str) -> Result<Addressable, &'static str> {
    legacy_address(black_box(input))
}

#[divan::bench(args = ADDRESSES)]
fn fixed_address(input: &str) -> Result<Addressable, &'static str> {
    Addressable::try_from(black_box(input))
}

const LISTS: &[&str] = &[
    "-",
    "192.0.2.1",
    "192.0.2.1,\\x202001:db8::1,unknown,192.0.2.2:443",
    "0123.045.067.089,192.0.2.2",
    "192.0.2.1,broken",
];

#[divan::bench(args = LISTS)]
fn owned_addresses(bencher: Bencher<'_, '_>, input: &str) {
    bencher.bench_local(|| {
        if black_box(input) != "-" {
            match ForwardedForAddrs::try_from(input) {
                Ok(addresses) => {
                    for address in &addresses.0 {
                        black_box(address);
                    }
                }
                Err(error) => {
                    black_box(error);
                }
            }
        }
    });
}

#[divan::bench(args = LISTS)]
fn iterated_addresses(bencher: Bencher<'_, '_>, input: &str) {
    bencher.bench_local(|| {
        if black_box(input) != "-" {
            for address in ForwardedForAddrs::iter_str(input) {
                match address {
                    Ok(address) => {
                        black_box(address);
                    }
                    Err(error) => {
                        black_box(error);
                        break;
                    }
                }
            }
        }
    });
}

#[divan::bench(args = LISTS)]
fn first_address(input: &str) -> Option<Result<Addressable, &'static str>> {
    if black_box(input) == "-" {
        None
    } else {
        ForwardedForAddrs::iter_str(input).next()
    }
}

const RESULTS: &[&str] = &[
    "Hit",
    "Miss",
    "OriginShieldHit",
    "FutureCloudFrontResultValue",
];

#[divan::bench(args = RESULTS)]
fn owned_result(input: &str) -> EdgeResultType {
    EdgeResultType::from(black_box(input))
}

#[divan::bench(args = RESULTS)]
fn owned_detailed_result(input: &str) -> DetailedEdgeResultType {
    DetailedEdgeResultType::from(black_box(input))
}

// The existing raw API already exposes exactly this representation without allocation.
#[divan::bench(args = RESULTS)]
fn borrowed_result(input: &str) -> &str {
    black_box(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_storage_matches_original_parser() {
        for first in ["0", "00", "01", "0255", "0256", "0+1", "0x01"] {
            for tail in [
                "2.3.4", "2.3.256", "2.3", "2.3.4.5", ".3.4", "2.3.", "2.3.-1", "2.3.+4", "2.3.四",
            ] {
                let input = format!("{first}.{tail}");
                assert_eq!(
                    Addressable::try_from(input.as_str()),
                    legacy_address(&input),
                    "{input:?}"
                );
            }
        }
    }
}
