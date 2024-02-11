pub(crate) mod line;
pub(crate) mod view;

// lines

pub use line::LogLine;
pub use LogLine as RawLogLine;

pub use line::checked::LogLine as CheckedRawLogLine;

#[cfg(feature = "unsafe")]
pub use line::unsafe_::LogLine as UnsafeRawLogLine;

// views

pub use view::checked::LogLineView as CheckedRawLogLineView;
pub use view::smart::LogLineView as SmartRawLogLineView;
// pub use view::unwrapped::LogLineView as UnwrappedRawLogLineView;

// #[cfg(feature = "unsafe")]
// pub use view::unsafe_::LogLineView as UnsafeRawLogLineView;

// experiment
// pub use line::regex::LogLine as RegexLogLine;
