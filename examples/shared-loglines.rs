//! Share a complete parsed record between consumers without reparsing clones.
#![forbid(unsafe_code)]

use cloudfront_logs::referential::ValidatedSimpleLogline;
use std::sync::Arc;

fn main() -> Result<(), &'static str> {
    let input: Arc<str> = "2024-02-29\t12:34:56\tLAX1\t392\t192.0.2.100\tGET\td.example\t/\t200\t-\tagent\t-\t-\tFutureResult\tid\td.example\thttps\t23\t0.001\t192.0.2.7,\\x202001:db8::7\tTLSv1.2\tcipher\tFutureResponse\tHTTP/2.0\t-\t-\t11040\t0.001\tFutureDetail\ttext/html\t78\t-\t-".into();

    // Retain input when consumers need raw text independently of the record.
    let parsed = Arc::new(ValidatedSimpleLogline::try_from(Arc::clone(&input))?);
    let metrics = Arc::clone(&parsed);
    let storage = Arc::clone(&parsed);
    drop(parsed);

    // Each consumer reads the same parsed values. No per-record thread is needed.
    println!(
        "status={}, bytes={}",
        metrics.view().sc_status,
        metrics.view().sc_bytes
    );
    println!("raw input has {} bytes", storage.as_raw().len());
    drop(metrics);

    // With one shared handle left, consume the record without a reparsing clone.
    let record = Arc::try_unwrap(storage).expect("storage is the last handle");
    let recovered = record.into_raw();
    assert!(Arc::ptr_eq(&input, &recovered));
    Ok(())
}
