//! Line owning variants of the parser types by taking ownership
//! of the input data and offering a borrowed view into the parsed line.
//!
//! This is a "compromise" with regards to fully owned versions of the parser.
//! Therefore the performance is not as good as such versions, since more memory is needed for partially and fully typed variants.
//!
//! This module provides a way to own the parsed log lines, so you can pass them around.
//!
//! One use case this can solve is stream processing of CloudFront log files, which are gzipped.

// TODO: elaborate on stream processing and owned/borrowed data

pub mod raw;
pub mod simple;
pub mod typed;

#[cfg(feature = "parquet")]
pub mod parquet;

pub use raw::{
    UnvalidatedLogline as UnvalidatedRawLogline, ValidatedLogline as ValidatedRawLogline,
};

pub use simple::{
    UnvalidatedLogline as UnvalidatedSimpleLogline, ValidatedLogline as ValidatedSimpleLogline,
};

#[cfg(feature = "chrono")]
pub use typed::chrono::{
    UnvalidatedLogline as UnvalidatedChronoLogline, ValidatedLogline as ValidatedChronoLogline,
};

#[cfg(feature = "time")]
pub use typed::time::{
    UnvalidatedLogline as UnvalidatedTimeLogline, ValidatedLogline as ValidatedTimeLogline,
};

#[cfg(feature = "parquet")]
pub use parquet::{
    UnvalidatedLogline as UnvalidatedParquetLogline, ValidatedLogline as ValidatedParquetLogline,
};
