//! In-memory ingestion comparisons; excludes disk I/O and decompression.

#![forbid(unsafe_code)]

#[allow(dead_code)]
#[path = "../benches/corpus.rs"]
mod corpus;
#[path = "support/stream.rs"]
mod stream;

use cloudfront_logs::ValidatedRawLogline;
use divan::{
    Bencher, black_box,
    counter::{BytesCount, ItemsCount},
};
use std::{
    io::{self, BufRead, BufReader, Cursor, Read},
    ops::ControlFlow,
};

#[cfg(feature = "bench-alloc")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

fn input() -> String {
    let corpus = corpus::generate(corpus::Profile::Mixed, 4096);
    let mut text = String::with_capacity(corpus.bytes + corpus.lines.len());
    for line in corpus.lines {
        text.push_str(&line);
        text.push('\n');
    }
    text
}

fn consume(raw: ValidatedRawLogline<'_>) {
    black_box(raw.sc_status.parse::<u16>().unwrap());
    black_box(raw.sc_bytes.parse::<u64>().unwrap());
}

#[divan::bench]
fn allocation_per_line(bencher: Bencher<'_, '_>) {
    let text = input();
    bencher
        .counter(BytesCount::new(text.len()))
        .counter(ItemsCount::new(4096usize))
        .bench_local(|| {
            let reader = BufReader::new(Cursor::new(black_box(text.as_bytes())));
            for line in reader.lines() {
                let line = line.unwrap();
                consume(ValidatedRawLogline::try_from(line.as_str()).unwrap());
            }
        });
}

#[divan::bench]
fn manually_reused_buffer(bencher: Bencher<'_, '_>) {
    let text = input();
    bencher
        .counter(BytesCount::new(text.len()))
        .counter(ItemsCount::new(4096usize))
        .bench_local(|| {
            let mut reader = BufReader::new(Cursor::new(black_box(text.as_bytes())));
            let mut line = String::new();
            loop {
                line.clear();
                if reader.read_line(&mut line).unwrap() == 0 {
                    break;
                }
                let text = line.strip_suffix('\n').unwrap_or(&line);
                let text = text.strip_suffix('\r').unwrap_or(text);
                consume(ValidatedRawLogline::try_from(text).unwrap());
            }
        });
}

#[divan::bench]
fn bounded_callback(bencher: Bencher<'_, '_>) {
    let text = input();
    bencher
        .counter(BytesCount::new(text.len()))
        .counter(ItemsCount::new(4096usize))
        .bench_local(|| {
            let mut reader = BufReader::new(Cursor::new(black_box(text.as_bytes())));
            let result = stream::process(&mut reader, 1024 * 1024, |_, raw| {
                consume(raw);
                Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
            })
            .unwrap();
            let _ = black_box(result);
        });
}

// Repeats a small immutable input without retaining the full stream. This makes
// allocations and peak process memory comparable as stream length increases.
struct Repeated<'a> {
    text: &'a [u8],
    position: usize,
    remaining: usize,
}

impl Read for Repeated<'_> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let input = self.fill_buf()?;
        let count = output.len().min(input.len());
        output[..count].copy_from_slice(&input[..count]);
        self.consume(count);
        Ok(count)
    }
}

impl BufRead for Repeated<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.remaining == 0 {
            return Ok(&[]);
        }
        Ok(&self.text[self.position..])
    }
    fn consume(&mut self, amount: usize) {
        self.position += amount;
        if self.position == self.text.len() && self.remaining != 0 {
            self.position = 0;
            self.remaining -= 1;
        }
    }
}

#[divan::bench(args = [1usize, 16, 256])]
fn long_stream_bounded(bencher: Bencher<'_, '_>, repetitions: usize) {
    let text = input();
    bencher
        .counter(BytesCount::new(text.len() * repetitions))
        .counter(ItemsCount::new(4096 * repetitions))
        .bench_local(|| {
            let mut reader = Repeated {
                text: black_box(text.as_bytes()),
                position: 0,
                remaining: repetitions,
            };
            let result = stream::process(&mut reader, 1024 * 1024, |_, raw| {
                consume(raw);
                Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
            })
            .unwrap();
            let _ = black_box(result);
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_reader_produces_exact_number_of_bytes_and_records() {
        let mut reader = Repeated {
            text: b"abc\n",
            position: 0,
            remaining: 3,
        };
        let mut output = Vec::new();
        reader.read_to_end(&mut output).unwrap();
        assert_eq!(output, b"abc\nabc\nabc\n");
    }
}
