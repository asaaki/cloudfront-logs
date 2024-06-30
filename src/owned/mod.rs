// todo: add more variants (raw, typed, parquet)

pub mod simple;

pub use simple::{
    UnvalidatedLogline as UnvalidatedSimpleLogline, ValidatedLogline as ValidatedSimpleLogline,
};

#[cfg(feature = "parquet")]
pub mod parquet;

#[cfg(feature = "parquet")]
pub use parquet::{
    UnvalidatedLogline as UnvalidatedParquetLogline, ValidatedLogline as ValidatedParquetLogline,
};
