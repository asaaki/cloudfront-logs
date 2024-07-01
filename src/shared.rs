use crate::{COMMENT_U8, TABS, TAB_U8};

/// Validates a log line
///
/// This function checks if
/// * the line is not empty,
/// * not a comment line
/// * and has the correct number of fields.
///
/// # Examples
///
/// ```rust
/// use cloudfront_logs::validate_line;
///
/// let okay_line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-	11040	0.001	Hit	text/html	78	-	-";
/// let broken_line = "2019-12-04	21:02:31	LAX1	392	192.0.2.100	GET	d111111abcdef8.cloudfront.net	/index.html	200	-	Mozilla/5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/78.0.3904.108%20Safari/537.36	-	-	Hit	SOX4xwn4XV6Q4rgb7XiVGOHms_BGlTAC4KyHmureZmBNrjGdRLiNIQ==	d111111abcdef8.cloudfront.net	https	23	0.001	-	TLSv1.2	ECDHE-RSA-AES128-GCM-SHA256	Hit	HTTP/2.0	-	-";
/// let empty_line = "";
/// let comment_line = "#Version: 1.0";
///
/// assert!(validate_line(okay_line).is_ok());
/// assert!(validate_line(broken_line).is_err());
/// assert!(validate_line(empty_line).is_err());
/// assert!(validate_line(comment_line).is_err());
/// ```
#[inline]
pub fn validate_line(line: &str) -> Result<(), &'static str> {
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

// if the input is "-", return Ok(None), otherwise parse the input as T;
// -> parse_as_option(iter.next().unwrap()).map_err(|_e| "…")?
// -> parse_as_option(str_input).map_err(|_e| "…")?
pub(crate) fn parse_as_option<T: std::str::FromStr>(s: &str) -> Result<Option<T>, T::Err> {
    if s == "-" {
        Ok(None)
    } else {
        s.parse().map(|v| Some(v))
    }
}

// better chainable version of parse_as_option;
// -> iter.next().and_then(as_optional_t).transpose().map_err(|_e| "…")?
pub(crate) fn as_optional_t<T: std::str::FromStr>(s: &str) -> Option<Result<T, T::Err>> {
    if s == "-" {
        None
    } else {
        Some(s.parse())
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

// str type extension trait;
// returns None if the input is "-", otherwise Some(&str)
#[cfg(feature = "parquet")]
pub(crate) trait AsOptionalStr {
    fn as_optional_str(&self) -> Option<&str>;
}

#[cfg(feature = "parquet")]
impl AsOptionalStr for str {
    fn as_optional_str(&self) -> Option<&str> {
        if self == "-" {
            None
        } else {
            Some(self)
        }
    }
}
