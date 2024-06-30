// todo: add more variants (raw, typed, parquet)

pub mod simple;

pub use simple::{
    UnvalidatedLogline as UnvalidatedSimpleLogline, ValidatedLogline as ValidatedSimpleLogline,
};
