//! Line owning variants of the parser types by taking ownership
//! of the input data and offering a borrowed view into the parsed line.
//!
//! These records retain the input and a parsed view in an owning container.
//! Construction and destruction costs depend on the fields and representation.
//!
//! This module provides a way to own the parsed log lines, so you can pass them around.
//!
//! One use case this can solve is stream processing of CloudFront log files, which are gzipped.
//!
//! # Sharing parsed records
//!
//! Cloning a referential record shares its input `Arc<str>` but rebuilds the
//! parsed view. For repeated fan-out, put the complete record in
//! [`Arc`](std::sync::Arc) and use `Arc::clone(&record)`. Those handles share both
//! the input and parsed values, including forwarded-address vectors and unknown
//! result strings. `view()` and `as_raw()` remain available through dereferencing.
//!
//! Wrapping a record adds one allocation and atomic reference-count operations.
//! Keep a directly owned record when sharing is unnecessary. The final shared
//! handle still destroys the parsed values and container; destruction is not
//! constant-time for every record.
//!
//! To consume a uniquely held record, use `Arc::try_unwrap(shared)` followed by
//! `into_raw()`. With other handles alive, `Arc::unwrap_or_clone(shared).into_raw()`
//! preserves the input allocation but invokes the record's reparsing clone once.
//! If callers need raw input while sharing the parsed record, they can retain
//! their original `Arc<str>` and pass a clone into construction.

// TODO: elaborate on stream processing and owned/borrowed data

pub mod raw;
pub mod structured;

#[cfg(feature = "parquet")]
pub mod parquet;

#[cfg(feature = "parquet")]
pub use parquet::{
    UnvalidatedLogline as UnvalidatedParquetLogline, ValidatedLogline as ValidatedParquetLogline,
};
pub use raw::{
    UnvalidatedLogline as UnvalidatedRawLogline, ValidatedLogline as ValidatedRawLogline,
};
pub use structured::{
    UnvalidatedSimpleLogline as OwningUnvalidatedSimpleLogline,
    ValidatedSimpleLogline as OwningValidatedSimpleLogline,
};
#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
pub use structured::{
    UnvalidatedTypedLogline as OwningUnvalidatedTypedLogline,
    ValidatedTypedLogline as OwningValidatedTypedLogline,
};

pub use structured::{UnvalidatedSimpleLogline, ValidatedSimpleLogline};
#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
pub use structured::{UnvalidatedTypedLogline, ValidatedTypedLogline};
