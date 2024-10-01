//! AWS CloudFront logs parser
//!
//! The log file format is described in the official documentation:
//! <https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html#LogFileFormat>

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::missing_enforced_import_renames,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match_else,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    unexpected_cfgs,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// not enforced right now:
// clippy::doc_markdown -- false positives for term CloudFront

// mem_forget: safe_cell
// tabs_in_doc_comments: tab'ed CF log lines in examples
#![allow(deprecated, clippy::tabs_in_doc_comments, clippy::mem_forget)]

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
