//! AWS CloudFront logs parser
//!
//! The log file format is described in the official documentation:
//! <https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html#LogFileFormat>

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(deprecated)]

// @@@ NEW STRUCTURE @@@

mod shared;

pub mod borrowed;
pub mod consts;
pub mod owned;
pub mod referential; // not sure about the module name yet
pub mod types;

pub use consts::*;
pub use types::*;

// useful helper function for minimizing validation needs
pub use shared::validate_line;

#[doc(inline)]
pub use borrowed::{
    UnvalidatedRawLogline, UnvalidatedSimpleLogline, ValidatedRawLogline, ValidatedSimpleLogline,
};

#[cfg(feature = "chrono")]
#[doc(inline)]
pub use borrowed::typed::{UnvalidatedChronoLogline, ValidatedChronoLogline};

#[cfg(feature = "time")]
#[doc(inline)]
pub use borrowed::typed::{UnvalidatedTimeLogline, ValidatedTimeLogline};

#[cfg(feature = "parquet")]
#[doc(inline)]
pub use borrowed::{UnvalidatedParquetLogline, ValidatedParquetLogline};

#[doc(inline)]
pub use referential::{
    UnvalidatedRawLogline as OwningUnvalidatedRawLogline,
    UnvalidatedSimpleLogline as OwningUnvalidatedSimpleLogline,
    ValidatedRawLogline as OwningValidatedRawLogline,
    ValidatedSimpleLogline as OwningValidatedSimpleLogline,
};

#[cfg(feature = "chrono")]
#[doc(inline)]
pub use referential::typed::{
    UnvalidatedChronoLogline as OwningUnvalidatedChronoLogline,
    ValidatedChronoLogline as OwningValidatedChronoLogline,
};

#[cfg(feature = "time")]
#[doc(inline)]
pub use referential::typed::{
    UnvalidatedTimeLogline as OwningUnvalidatedTimeLogline,
    ValidatedTimeLogline as OwningValidatedTimeLogline,
};

#[cfg(feature = "parquet")]
#[doc(inline)]
pub use referential::{
    UnvalidatedParquetLogline as OwningUnvalidatedParquetLogline,
    ValidatedParquetLogline as OwningValidatedParquetLogline,
};

#[cfg(feature = "parquet")]
#[doc(inline)]
pub use owned::{
    UnvalidatedParquetLogline as OwnedUnvalidatedParquetLogline,
    ValidatedParquetLogline as OwnedValidatedParquetLogline,
};

// === tests ===

#[cfg(test)]
mod tests;

// !!! DEPRECATED !!!

#[deprecated(
    since = "0.7.0",
    note = "use new modules/types instead (borrowed, owned, referential)"
)]
mod raw;
#[deprecated(
    since = "0.7.0",
    note = "use new modules/types instead (borrowed, owned, referential)"
)]
mod simple;

#[deprecated(
    since = "0.7.0",
    note = "use new modules/types instead (borrowed, owned, referential)"
)]
#[cfg(feature = "time")]
mod typed;

#[deprecated(
    since = "0.7.0",
    note = "use new modules/types instead (borrowed, owned, referential)"
)]
#[cfg(feature = "parquet")]
mod parquet;

#[deprecated(
    since = "0.7.0",
    note = "use new modules/types instead (borrowed, owned, referential)"
)]
pub mod deprecated {
    pub use crate::raw::{CheckedRawLogLine, CheckedRawLogLineView, SmartRawLogLineView};

    #[cfg(feature = "alloc")]
    pub use crate::simple::SimpleLogLine;

    #[cfg(feature = "time")]
    pub use crate::typed::TypedLogLine;

    #[cfg(feature = "parquet")]
    pub use crate::parquet::ParquetLogLine;
}

pub use deprecated::*;
