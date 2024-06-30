//! Logline types which (mostly) borrow their data from a [`&str`]
//!
//! Therefore they are only suitable for cases where the log date is immediately processed/consumed
//! and the struct can be discarded quickly afterwards.

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
