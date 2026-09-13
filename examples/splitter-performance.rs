//! Run with --bench and explicit sample counts; see task-6-splitter.md.
#![forbid(unsafe_code)]

#[path = "../benches/corpus.rs"]
mod corpus;
#[path = "support/splitter.rs"]
mod splitter;

use cloudfront_logs::{ValidatedRawLogline, ValidatedSimpleLogline};
use corpus::{Corpus, Profile, generate};
use divan::{
    Bencher, black_box,
    counter::{BytesCount, ItemsCount},
};
use splitter::{Boundaries, CheckedParser, Original, Streaming, template};

#[cfg(feature = "bench-alloc")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    let (boundaries, fields, record) = splitter::nominal_storage();
    println!("Checked splitter experiment; all setup outside timing; records are synthetic.");
    println!(
        "Nominal storage: boundaries={boundaries} B, fields={fields} B, raw record={record} B; compiler may eliminate/copy storage."
    );
    println!(
        "Whole-record barriers consume complete outputs. Timed work includes destruction; no I/O/decompression."
    );
    divan::main();
}

#[derive(Debug, Clone, Copy)]
enum Input {
    Short,
    Long,
    Mixed,
    Large,
    Malformed,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

const INPUTS: [Input; 5] = [
    Input::Short,
    Input::Long,
    Input::Mixed,
    Input::Large,
    Input::Malformed,
];

fn inputs(input: Input) -> Corpus {
    let count = std::env::var("CF_SPLITTER_RECORDS").map_or(2048, |count| {
        count
            .parse::<usize>()
            .expect("positive CF_SPLITTER_RECORDS")
    });
    assert!(count > 0);
    let mut corpus = match input {
        Input::Short | Input::Malformed => generate(Profile::Ipv4, count),
        Input::Long => generate(Profile::LongFields, count),
        Input::Mixed => generate(Profile::Mixed, count),
        Input::Large => generate(Profile::Mixed, count * 16),
    };
    if matches!(input, Input::Malformed) {
        for (index, line) in corpus.lines.iter_mut().enumerate() {
            let mut fields: Vec<_> = line.split('\t').map(str::to_owned).collect();
            match index % 7 {
                0 => fields.clear(),
                1 => fields[0] = "#comment".into(),
                2 => {
                    fields.pop();
                }
                3 => fields.push("extra".into()),
                4 => fields[3] = "bad".into(),
                5 => fields[18] = "NaN".into(),
                _ => fields[32].clear(),
            }
            *line = fields.join("\t");
        }
        corpus.bytes = corpus.lines.iter().map(String::len).sum();
    }
    corpus
}

macro_rules! parsed_bench {
    ($name:ident, $conversion:expr) => {
        #[divan::bench(types = [Original, Boundaries, Streaming], args = INPUTS)]
        fn $name<P: CheckedParser>(bencher: Bencher<'_, '_>, input: Input) {
            let corpus = inputs(input);
            let template = template();
            bencher
                .counter(ItemsCount::new(corpus.lines.len()))
                .counter(BytesCount::new(corpus.bytes))
                .bench_local(|| {
                    for line in &corpus.lines {
                        let record =
                            P::parse(black_box(line.as_str()), template).and_then($conversion);
                        let _ = black_box(record);
                    }
                });
        }
    };
}

parsed_bench!(raw_full, Ok::<_, &'static str>);
parsed_bench!(simple_via_raw, ValidatedSimpleLogline::try_from);
#[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
parsed_bench!(
    typed_via_raw,
    cloudfront_logs::ValidatedTypedLogline::try_from
);

fn selected(raw: ValidatedRawLogline<'_>) -> Result<(u16, u64, &str), &'static str> {
    let status = raw
        .sc_status
        .parse::<u16>()
        .map_err(|_| "sc_status invalid")?;
    let bytes = raw
        .sc_bytes
        .parse::<u64>()
        .map_err(|_| "sc_bytes invalid")?;
    Ok((status, bytes, raw.cs_uri_stem))
}

parsed_bench!(selected_fields, selected);

macro_rules! direct_bench {
    ($name:ident, $record:ident) => {
        #[divan::bench(args = INPUTS)]
        fn $name(bencher: Bencher<'_, '_>, input: Input) {
            let corpus = inputs(input);
            bencher
                .counter(ItemsCount::new(corpus.lines.len()))
                .counter(BytesCount::new(corpus.bytes))
                .bench_local(|| {
                    for line in &corpus.lines {
                        let _ = black_box($record::try_from(black_box(line.as_str())));
                    }
                });
        }
    };
}

direct_bench!(simple_direct, ValidatedSimpleLogline);
#[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
use cloudfront_logs::ValidatedTypedLogline;
#[cfg(any(feature = "jiff", feature = "time", feature = "chrono"))]
direct_bench!(typed_direct, ValidatedTypedLogline);
