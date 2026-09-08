use cloudfront_logs::referential::{ValidatedJiffLogline, UnvalidatedJiffLogline};

fn main() {
    let _ = std::mem::size_of::<ValidatedJiffLogline>();
    let _ = std::mem::size_of::<UnvalidatedJiffLogline>();
}
