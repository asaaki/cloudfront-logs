//! Reusable example helper; this is not part of the public library API.

use cloudfront_logs::ValidatedRawLogline;
use std::{
    fmt,
    io::{self, BufRead},
    ops::ControlFlow,
};

#[derive(Debug)]
pub enum Error<E> {
    Io { line: usize, source: io::Error },
    Parse { line: usize, message: &'static str },
    TooLong { line: usize, limit: usize },
    Callback { line: usize, source: E },
}

impl<E: fmt::Display> fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { line, source } => write!(f, "line {line}: {source}"),
            Self::Parse { line, message } => write!(f, "line {line}: {message}"),
            Self::TooLong { line, limit } => write!(f, "line {line}: exceeds {limit} bytes"),
            Self::Callback { line, source } => write!(f, "line {line}: {source}"),
        }
    }
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for Error<E> {}

/// Process raw records in order, reusing storage after each callback returns.
///
/// `max_line_bytes` includes the LF or CRLF terminator, including on comments.
/// Empty non-comment lines are parse errors. Only an LF and its preceding CR are
/// removed; other trailing whitespace is data. Raw parsing validates structure;
/// the callback must check any selected values, or construct a structured record.
/// The callback cannot retain borrowed fields after returning. Copy selected
/// fields, convert the raw record to an existing owned record, or return owned
/// data through `ControlFlow::Break` to retain it. An error terminates processing.
/// A break leaves the next line unread in `reader`, including buffered input.
pub fn process<R: BufRead + ?Sized, F, B, E>(
    reader: &mut R,
    max_line_bytes: usize,
    mut callback: F,
) -> Result<ControlFlow<B>, Error<E>>
where
    F: for<'line> FnMut(usize, ValidatedRawLogline<'line>) -> Result<ControlFlow<B>, E>,
{
    let mut buffer = Vec::new();
    let mut line_number = 0usize;
    loop {
        buffer.clear();
        line_number += 1;
        loop {
            let available = match reader.fill_buf() {
                Ok(bytes) => bytes,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(source) => {
                    return Err(Error::Io {
                        line: line_number,
                        source,
                    });
                }
            };
            if available.is_empty() {
                if buffer.is_empty() {
                    return Ok(ControlFlow::Continue(()));
                }
                break;
            }
            let newline = memchr::memchr(b'\n', available);
            let count = newline.map_or(available.len(), |n| n + 1);
            if count > max_line_bytes - buffer.len() {
                return Err(Error::TooLong {
                    line: line_number,
                    limit: max_line_bytes,
                });
            }
            buffer.extend_from_slice(&available[..count]);
            reader.consume(count);
            if newline.is_some() {
                break;
            }
        }

        if buffer.last() == Some(&b'\n') {
            buffer.pop();
            if buffer.last() == Some(&b'\r') {
                buffer.pop();
            }
        }
        let text = std::str::from_utf8(&buffer).map_err(|source| Error::Io {
            line: line_number,
            source: io::Error::new(io::ErrorKind::InvalidData, source),
        })?;
        if text.starts_with('#') {
            continue;
        }
        let raw = ValidatedRawLogline::try_from(text).map_err(|message| Error::Parse {
            line: line_number,
            message,
        })?;
        match callback(line_number, raw).map_err(|source| Error::Callback {
            line: line_number,
            source,
        })? {
            ControlFlow::Continue(()) => {}
            ControlFlow::Break(value) => return Ok(ControlFlow::Break(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor, Read};

    const LINE: &str = "2024-02-29\t23:59:59\tLAX1\t392\t192.0.2.1\tGET\td.example\t/\t200\t-\tagent\t-\t-\tHit\tid\td.example\thttps\t23\t0.001\t-\tTLSv1.2\tcipher\tHit\tHTTP/2.0\t-\t-\t11040\t0.001\tHit\ttext/html\t78\t-\t-";

    #[test]
    fn preserves_physical_line_numbers_crlf_and_final_record() {
        let input = format!("#Version: 1.0\r\n{LINE}\r\n#another comment\n{LINE}");
        let mut reader = BufReader::with_capacity(7, Cursor::new(input));
        let mut seen = Vec::new();
        let result = process(&mut reader, LINE.len() + 2, |n, raw| {
            seen.push((n, raw.date.to_owned(), raw.sc_range_end.to_owned()));
            Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
        })
        .unwrap();
        assert_eq!(result, ControlFlow::Continue(()));
        assert_eq!(
            seen,
            [
                (2, "2024-02-29".into(), "-".into()),
                (4, "2024-02-29".into(), "-".into())
            ]
        );
    }

    #[test]
    fn early_break_retains_owned_fields_and_leaves_next_line_unread() {
        let input = format!("{LINE}\n{LINE}\n");
        let mut reader = BufReader::new(Cursor::new(input));
        let result = process(&mut reader, 1024, |_, raw| {
            Ok::<_, &'static str>(ControlFlow::Break(raw.cs_host.to_owned()))
        })
        .unwrap();
        assert_eq!(result, ControlFlow::Break("d.example".into()));
        let mut remaining = String::new();
        reader.read_to_string(&mut remaining).unwrap();
        assert_eq!(remaining, format!("{LINE}\n"));
    }

    #[test]
    fn blank_and_wrong_field_counts_report_parse_line() {
        for invalid in ["\n", "a\tb\n"] {
            let input = format!("#comment\n{invalid}{LINE}");
            let result = process(&mut Cursor::new(input), 1024, |_, _| {
                Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
            });
            assert!(matches!(result, Err(Error::Parse { line: 2, .. })));
        }
    }

    #[test]
    fn bounds_records_and_comments_before_reading_entire_line() {
        for data in [b"0123456789".as_slice(), b"#123456789\n"] {
            let mut reader = BufReader::with_capacity(3, Cursor::new(data));
            let result = process(&mut reader, 5, |_, _| {
                Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
            });
            assert!(matches!(result, Err(Error::TooLong { line: 1, limit: 5 })));
            assert!(reader.get_ref().position() <= 6);
        }
    }

    #[test]
    fn accepts_exact_limit_including_terminator_and_rejects_one_extra_byte() {
        let input = format!("{LINE}\r\n");
        for (limit, want_ok) in [(input.len(), true), (input.len() - 1, false)] {
            let result = process(&mut Cursor::new(&input), limit, |_, _| {
                Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
            });
            assert_eq!(result.is_ok(), want_ok);
        }
    }

    #[test]
    fn callback_error_is_not_swallowed_or_followed_by_another_record() {
        let mut calls = 0;
        let input = format!("#comment\n{LINE}\n{LINE}");
        let result = process(&mut Cursor::new(input), 1024, |_, _| {
            calls += 1;
            Err::<ControlFlow<()>, _>("sc_status invalid")
        });
        assert!(matches!(
            result,
            Err(Error::Callback {
                line: 2,
                source: "sc_status invalid"
            })
        ));
        assert_eq!(calls, 1);
    }

    #[test]
    fn invalid_utf8_is_an_io_error_with_the_correct_line() {
        let result = process(&mut Cursor::new(b"#comment\n\xff\n"), 1024, |_, _| {
            Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
        });
        assert!(
            matches!(result, Err(Error::Io { line: 2, source }) if source.kind() == io::ErrorKind::InvalidData)
        );
    }

    #[test]
    fn preserves_io_error_and_partial_line_number() {
        struct FailsAfterComment(Cursor<Vec<u8>>);
        impl Read for FailsAfterComment {
            fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
                self.0.read(out)
            }
        }
        impl BufRead for FailsAfterComment {
            fn fill_buf(&mut self) -> io::Result<&[u8]> {
                if self.0.position() == self.0.get_ref().len() as u64 {
                    Err(io::Error::other("disk failed"))
                } else {
                    self.0.fill_buf()
                }
            }
            fn consume(&mut self, amount: usize) {
                self.0.consume(amount);
            }
        }
        let mut reader = FailsAfterComment(Cursor::new(b"#comment\npartial".to_vec()));
        let result = process(&mut reader, 1024, |_, _| {
            Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
        });
        assert!(
            matches!(result, Err(Error::Io { line: 2, source }) if source.to_string() == "disk failed")
        );
    }

    #[test]
    fn empty_stream_succeeds_even_with_zero_limit() {
        let result = process(&mut Cursor::new(b""), 0, |_, _| {
            panic!("empty stream must not call callback");
            #[allow(unreachable_code)]
            Ok::<_, &'static str>(ControlFlow::<()>::Continue(()))
        })
        .unwrap();
        assert_eq!(result, ControlFlow::Continue(()));
    }
}
