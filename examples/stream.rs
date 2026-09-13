//! Read CloudFront logs from stdin with bounded memory.
//!
//! Run: cargo run --release --example stream --no-default-features -- [max-line-bytes]
//! The limit includes a line's CR/LF terminator. Default: 1 MiB.

#![forbid(unsafe_code)]

#[path = "support/stream.rs"]
mod stream;

use std::{io, ops::ControlFlow};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let limit = std::env::args()
        .nth(1)
        .map(|s| s.parse::<usize>())
        .transpose()?
        .unwrap_or(1024 * 1024);
    let mut records = 0u64;
    let mut server_errors = 0u64;
    let mut bytes = 0u64;
    let _ = stream::process(&mut io::stdin().lock(), limit, |_number, raw| {
        let status = raw
            .sc_status
            .parse::<u16>()
            .map_err(|_| "sc_status invalid")?;
        let size = raw
            .sc_bytes
            .parse::<u64>()
            .map_err(|_| "sc_bytes invalid")?;
        records += 1;
        server_errors += u64::from(status >= 500);
        bytes = bytes.checked_add(size).ok_or("byte total overflow")?;
        Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
    })?;
    println!("records={records} server_errors={server_errors} response_bytes={bytes}");
    Ok(())
}
