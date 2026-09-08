use cloudfront_logs::{OwningValidatedJiffLogline, ValidatedJiffLogline};

fn main() {
    let _ = std::mem::size_of::<ValidatedJiffLogline<'static>>();
    let _ = std::mem::size_of::<OwningValidatedJiffLogline>();
}
