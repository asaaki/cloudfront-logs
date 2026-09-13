//! Separate from methodology v2: full-record, filtering, and lifecycle costs.

use crate::corpus::{Corpus, PROFILES, Profile, generate};
use crate::{CorpusOwned, CorpusRaw, CorpusReferential, CorpusReferentialRaw, CorpusSimple};
use divan::{
    Bencher, black_box,
    counter::{BytesCount, ItemsCount},
};

pub fn large_records() -> usize {
    std::env::var("CF_BENCH_RECORDS").map_or(32_700, |value| {
        let count: usize = value
            .parse()
            .expect("CF_BENCH_RECORDS must be a positive integer");
        assert!(count > 0, "CF_BENCH_RECORDS must be positive");
        count
    })
}

pub fn describe() {
    println!(
        "Representative v1: synthetic corpus, 100-record hot batches; {}-record MixedLarge batches.",
        large_records()
    );
    println!(
        "Full/filter times and allocations are per batch; divide by records. Lifecycle results are per record."
    );
    println!(
        "Allocation profiler: {}. No file I/O, decompression, or encoding.",
        if cfg!(feature = "bench-alloc") {
            "enabled (instrumented timings)"
        } else {
            "disabled"
        }
    );
    println!(
        "Representative groups are opt-in: use --ignored with explicit --sample-count and --sample-size."
    );
}

#[derive(Debug, Clone, Copy)]
enum Workload {
    MixedHot,
    MixedLarge,
    Ipv4,
    Ipv6,
    Forwarded,
    UnknownResults,
    LongFields,
    OptionalValues,
}

impl std::fmt::Display for Workload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

const WORKLOADS: [Workload; 8] = [
    Workload::MixedHot,
    Workload::MixedLarge,
    Workload::Ipv4,
    Workload::Ipv6,
    Workload::Forwarded,
    Workload::UnknownResults,
    Workload::LongFields,
    Workload::OptionalValues,
];

fn corpus(workload: Workload) -> Corpus {
    let profile = match workload {
        Workload::MixedHot | Workload::MixedLarge => Profile::Mixed,
        Workload::Ipv4 => Profile::Ipv4,
        Workload::Ipv6 => Profile::Ipv6,
        Workload::Forwarded => Profile::Forwarded,
        Workload::UnknownResults => Profile::UnknownResults,
        Workload::LongFields => Profile::LongFields,
        Workload::OptionalValues => Profile::OptionalValues,
    };
    generate(
        profile,
        if matches!(workload, Workload::MixedLarge) {
            large_records()
        } else {
            100
        },
    )
}

// Observe every source field and the complete record, including derived fields
// such as Parquet datetime and Vec/String payloads through references.
// This deliberately includes the observer cost, but no formatting/serialization.
macro_rules! consume {
    ($record:expr) => {{
        let record = &$record;
        black_box(&record.date);
        black_box(&record.time);
        black_box(&record.x_edge_location);
        black_box(&record.sc_bytes);
        black_box(&record.c_ip);
        black_box(&record.cs_method);
        black_box(&record.cs_host);
        black_box(&record.cs_uri_stem);
        black_box(&record.sc_status);
        black_box(&record.cs_referer);
        black_box(&record.cs_user_agent);
        black_box(&record.cs_uri_query);
        black_box(&record.cs_cookie);
        black_box(&record.x_edge_result_type);
        black_box(&record.x_edge_request_id);
        black_box(&record.x_host_header);
        black_box(&record.cs_protocol);
        black_box(&record.cs_bytes);
        black_box(&record.time_taken);
        black_box(&record.x_forwarded_for);
        black_box(&record.ssl_protocol);
        black_box(&record.ssl_cipher);
        black_box(&record.x_edge_response_result_type);
        black_box(&record.cs_protocol_version);
        black_box(&record.fle_status);
        black_box(&record.fle_encrypted_fields);
        black_box(&record.c_port);
        black_box(&record.time_to_first_byte);
        black_box(&record.x_edge_detailed_result_type);
        black_box(&record.sc_content_type);
        black_box(&record.sc_content_len);
        black_box(&record.sc_range_start);
        black_box(&record.sc_range_end);
        black_box(record);
    }};
}

macro_rules! full {
    ($name:ident, $record:ident) => {
        #[divan::bench(args = WORKLOADS, sample_size = 1, sample_count = 30)]
        fn $name(bencher: Bencher<'_, '_>, workload: Workload) {
            let corpus = corpus(workload);
            bencher
                .counter(ItemsCount::new(corpus.lines.len()))
                .counter(BytesCount::new(corpus.bytes))
                .bench_local(|| {
                    for line in &corpus.lines {
                        let record = $record::try_from(black_box(line.as_str()))
                            .expect("valid synthetic corpus");
                        consume!(record);
                        // Record destruction is intentionally included in full scans.
                    }
                });
        }
    };
}

#[divan::bench_group(ignore = true)]
mod full_record {
    use super::*;
    full!(raw, CorpusRaw);
    full!(simple, CorpusSimple);
    #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
    use crate::CorpusTyped;
    #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
    full!(typed, CorpusTyped);
    #[cfg(feature = "parquet")]
    use crate::CorpusParquet;
    #[cfg(feature = "parquet")]
    full!(parquet, CorpusParquet);
}

macro_rules! selection {
    ($module:ident, $record:ident) => {
        mod $module {
            use super::*;

            #[divan::bench(args = [0, 1, 10, 50, 100], sample_size = 1, sample_count = 30)]
            fn eager(bencher: Bencher<'_, '_>, rate: u16) {
                let corpus = corpus(Workload::MixedLarge);
                bencher
                    .counter(ItemsCount::new(corpus.lines.len()))
                    .counter(BytesCount::new(corpus.bytes))
                    .bench_local(|| {
                        let mut selected = 0;
                        for line in &corpus.lines {
                            let record = $record::try_from(black_box(line.as_str()))
                                .expect("valid synthetic corpus");
                            if record.sc_status < black_box(200 + rate) {
                                consume!(record);
                                selected += 1;
                            }
                        }
                        black_box(selected)
                    });
            }

            #[divan::bench(args = [0, 1, 10, 50, 100], sample_size = 1, sample_count = 30)]
            fn raw_then_convert(bencher: Bencher<'_, '_>, rate: u16) {
                let corpus = corpus(Workload::MixedLarge);
                bencher
                    .counter(ItemsCount::new(corpus.lines.len()))
                    .counter(BytesCount::new(corpus.bytes))
                    .bench_local(|| {
                        let mut selected = 0;
                        for line in &corpus.lines {
                            let raw = CorpusRaw::try_from(black_box(line.as_str()))
                                .expect("valid synthetic corpus");
                            let status = raw
                                .sc_status
                                .parse::<u16>()
                                .expect("valid synthetic status");
                            if status < black_box(200 + rate) {
                                let record =
                                    $record::try_from(raw).expect("valid synthetic corpus");
                                consume!(record);
                                selected += 1;
                            }
                        }
                        black_box(selected)
                    });
            }
        }
    };
}

#[divan::bench_group(ignore = true)]
mod filtering {
    use super::*;
    selection!(simple, CorpusSimple);
    #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
    use crate::CorpusTyped;
    #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
    selection!(typed, CorpusTyped);
}

// Divan generates inputs and destroys returned outputs outside the timed region.
// The source records cycle through the same 100 deterministic inputs per profile.
macro_rules! lifecycle {
    ($module:ident, $record:ident) => {
        mod $module {
            use super::*;

            #[divan::bench(args = PROFILES, sample_count = 30)]
            fn construct(bencher: Bencher<'_, '_>, profile: Profile) {
                let corpus = generate(profile, 100);
                let mut lines = corpus.lines.iter().cycle();
                bencher
                    .counter(ItemsCount::new(1_u8))
                    .with_inputs(|| lines.next().unwrap().as_str())
                    .input_counter(|line: &&str| BytesCount::new(line.len()))
                    .bench_local_values(|line| {
                        $record::try_from(black_box(line)).expect("valid synthetic corpus")
                    });
            }

            #[divan::bench(args = PROFILES, sample_count = 30)]
            #[allow(clippy::clone_on_copy)] // Raw clone is a useful copy-only control.
            fn clone(bencher: Bencher<'_, '_>, profile: Profile) {
                let corpus = generate(profile, 100);
                let mut lines = corpus.lines.iter().cycle();
                bencher
                    .counter(ItemsCount::new(1_u8))
                    .with_inputs(|| {
                        $record::try_from(lines.next().unwrap().as_str())
                            .expect("valid synthetic corpus")
                    })
                    .bench_local_refs(|record| black_box(record).clone());
            }

            #[divan::bench(args = PROFILES, sample_count = 30)]
            #[allow(dropping_copy_types)] // Raw destruction is a no-drop control.
            fn destroy(bencher: Bencher<'_, '_>, profile: Profile) {
                let corpus = generate(profile, 100);
                let mut lines = corpus.lines.iter().cycle();
                bencher
                    .counter(ItemsCount::new(1_u8))
                    .with_inputs(|| {
                        $record::try_from(lines.next().unwrap().as_str())
                            .expect("valid synthetic corpus")
                    })
                    .bench_local_values(|record| drop(black_box(record)));
            }
        }
    };
}

#[divan::bench_group(ignore = true)]
mod lifecycle {
    use super::*;
    lifecycle!(borrowed_raw, CorpusRaw);
    lifecycle!(referential_raw, CorpusReferentialRaw);
    lifecycle!(borrowed_simple, CorpusSimple);
    lifecycle!(owned_simple, CorpusOwned);
    lifecycle!(referential_simple, CorpusReferential);
    #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
    use crate::{CorpusReferentialTyped, CorpusTyped};
    #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
    lifecycle!(borrowed_typed, CorpusTyped);
    #[cfg(any(feature = "time", feature = "chrono", feature = "jiff"))]
    lifecycle!(referential_typed, CorpusReferentialTyped);
    #[cfg(feature = "parquet")]
    use crate::{CorpusOwnedParquet, CorpusParquet, CorpusReferentialParquet};
    #[cfg(feature = "parquet")]
    lifecycle!(borrowed_parquet, CorpusParquet);
    #[cfg(feature = "parquet")]
    lifecycle!(owned_parquet, CorpusOwnedParquet);
    #[cfg(feature = "parquet")]
    lifecycle!(referential_parquet, CorpusReferentialParquet);
}
