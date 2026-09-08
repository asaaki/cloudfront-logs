use cloudfront_logs::{CheckedRawLogLine, SimpleLogLine, TypedLogLine};

fn main() {
    let _ = std::mem::size_of::<CheckedRawLogLine>();
    let _ = std::mem::size_of::<SimpleLogLine>();
    let _ = std::mem::size_of::<TypedLogLine>();
}
