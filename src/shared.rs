use crate::{COMMENT_U8, TABS, TAB_U8};

#[inline]
pub(crate) fn valid_line(line: &str) -> Result<(), &'static str> {
    let bytes = line.as_bytes();
    if bytes.is_empty() {
        return Err("Invalid log line (empty)");
    }
    if bytes[0] == COMMENT_U8 {
        return Err("Invalid log line (comment)");
    }
    if memchr::memchr_iter(TAB_U8, bytes).count() != TABS {
        return Err("Invalid log line (field count)");
    }
    Ok(())
}

#[inline]
pub(crate) fn split(line: &str) -> MemchrTabSplitter<'_> {
    MemchrTabSplitter::new(line)
}

#[derive(Debug, Clone)]
pub(crate) struct MemchrTabSplitter<'a> {
    pub(crate) data: &'a str,
    pub(crate) prev: usize,
    pub(crate) end: usize,
    pub(crate) iter: memchr::Memchr<'a>,
}

impl<'a> MemchrTabSplitter<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        let prev = 0;
        let end = data.len();
        let iter = memchr::memchr_iter(TAB_U8, data.as_bytes());
        Self {
            data,
            prev,
            end,
            iter,
        }
    }
}

impl<'a> Iterator for MemchrTabSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let current_tab = self.iter.next();
        if let Some(tab_idx) = current_tab {
            assert!(tab_idx > 0, "Found tab stop at index 0 (invalid log line)");

            let from = self.prev;
            let to = tab_idx;
            self.prev = to + 1;
            Some(&self.data[from..to])
        } else {
            // get field after the last tab stop
            if self.prev < self.end {
                let from = self.prev;
                self.prev = self.end;
                Some(&self.data[from..])
            } else {
                None
            }
        }
    }
}

// if the input is "-", return Ok(None), otherwise parse the input as T
pub(crate) fn parse_as_option<T: std::str::FromStr>(s: &str) -> Result<Option<T>, T::Err> {
    if s == "-" {
        Ok(None)
    } else {
        s.parse().map(|v| Some(v))
    }
}

// String type extension trait;
// returns None if the input is "-", otherwise Some(String)
pub(crate) trait ToOptionalString {
    fn to_optional_string(&self) -> Option<String>;
}

impl ToOptionalString for &str {
    fn to_optional_string(&self) -> Option<String> {
        if self == &"-" {
            None
        } else {
            Some((*self).to_string())
        }
    }
}
