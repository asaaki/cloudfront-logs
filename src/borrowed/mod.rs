//! Logline types which (mostly) borrow their data from a [`&str`]
//!
//! Therefore they are only suitable for cases where the log date is immediately processed/consumed
//! and the struct can be discarded quickly afterwards.

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
pub use structured::{UnvalidatedSimpleLogline, ValidatedSimpleLogline};
#[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
pub use structured::{UnvalidatedTypedLogline, ValidatedTypedLogline};
