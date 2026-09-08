use cloudfront_logs::{borrowed::structured::DateTimeBackend, referential::structured::StructuredLine};

struct External;

impl DateTimeBackend for External {
    type Date<'a> = &'a str;
    type Time<'a> = &'a str;

    fn parse<'a>(date: &'a str, time: &'a str) -> Result<(&'a str, &'a str), &'static str> {
        Ok((date, time))
    }
}

impl StructuredLine for External {
    type Container = ();
    type View<'a> = ();

    fn new_container(_: std::sync::Arc<str>) -> Result<Self::Container, &'static str> { Ok(()) }
    fn as_raw(_: &Self::Container) -> &str { "" }
    fn into_raw(_: Self::Container) -> std::sync::Arc<str> { "".into() }
    fn view(_: &Self::Container) -> &Self::View<'_> { &() }
}

fn main() {}
