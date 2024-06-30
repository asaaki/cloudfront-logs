#[cfg(feature = "chrono")]
pub mod chrono;

#[cfg(feature = "chrono")]
pub use chrono::{
    UnvalidatedLogline as UnvalidatedChronoLogline, ValidatedLogline as ValidatedChronoLogline,
};

#[cfg(feature = "time")]
pub mod time;

#[cfg(feature = "time")]
pub use time::{
    UnvalidatedLogline as UnvalidatedTimeLogline, ValidatedLogline as ValidatedTimeLogline,
};
