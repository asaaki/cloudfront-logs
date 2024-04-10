use crate::shared::*;
use std::cell::RefCell;

/// View into a borrowed log line
///
/// Unlike [`RawLogLine`](crate::raw::RawLogLine), this struct does not compute all the fields upfront,
/// but instead provides methods to access the fields on demand.
///
/// This can be useful when you need to access only one or two fields from a log line.
/// Performance gets worse if you need to access many fields;
/// cloning the iterator becomes more expensive compared to a pre-computed struct like [`RawLogLine`](crate::raw::RawLogLine).
#[derive(Debug)]
pub struct LogLineView<'a> {
    line: &'a str,
    iter: RefCell<MemchrTabSplitter<'a>>,
    prev: RefCell<usize>,
    last: RefCell<&'a str>,
}

impl<'a> LogLineView<'a> {
    /// Creates a new `RawLogLineView` from a borrowed log line
    ///
    /// The line is checked for the correct number of fields and
    /// that it's not a comment line (like the version or fields header).
    ///
    pub fn new(line: &'a str) -> Result<Self, &'static str> {
        valid_line(line)?;

        let iter = RefCell::new(MemchrTabSplitter::new(line));
        let prev = RefCell::new(0);
        let last = RefCell::new(line);

        Ok(LogLineView {
            line,
            iter,
            prev,
            last,
        })
    }

    #[inline]
    fn field(&self, index: usize) -> &'a str {
        use std::cmp::Ordering;

        let mut prev = self.prev.borrow_mut();
        let mut last = self.last.borrow_mut();

        let v = match index.cmp(&*prev) {
            Ordering::Greater => {
                let rel = index - *prev - 1;
                *prev = index;
                self.iter.borrow_mut().nth(rel).unwrap()
            }
            Ordering::Equal => *last,
            Ordering::Less => {
                let mut iter = self.iter.borrow_mut();
                *iter = split(self.line);
                *prev = index;
                iter.nth(index).unwrap()
            }
        };
        *last = v;
        v
    }

    /// Returns the date field of the log line
    ///
    /// This is the fallible version, though it should never fail,
    /// as the line has been checked for the correct number of fields
    /// when the struct was initialised
    pub fn date(&self) -> &'a str {
        self.field(0)
    }

    pub fn time(&self) -> &'a str {
        self.field(1)
    }

    pub fn x_edge_location(&self) -> &'a str {
        self.field(2)
    }

    pub fn sc_bytes(&self) -> &'a str {
        self.field(3)
    }

    pub fn c_ip(&self) -> &'a str {
        self.field(4)
    }

    pub fn cs_method(&self) -> &'a str {
        self.field(5)
    }

    pub fn cs_host(&self) -> &'a str {
        self.field(6)
    }

    pub fn cs_uri_stem(&self) -> &'a str {
        self.field(7)
    }

    pub fn sc_status(&self) -> &'a str {
        self.field(8)
    }

    pub fn cs_referer(&self) -> &'a str {
        self.field(9)
    }

    pub fn cs_user_agent(&self) -> &'a str {
        self.field(10)
    }

    pub fn cs_uri_query(&self) -> &'a str {
        self.field(11)
    }

    pub fn cs_cookie(&self) -> &'a str {
        self.field(12)
    }

    pub fn x_edge_result_type(&self) -> &'a str {
        self.field(13)
    }

    pub fn x_edge_request_id(&self) -> &'a str {
        self.field(14)
    }

    pub fn x_host_header(&self) -> &'a str {
        self.field(15)
    }

    pub fn cs_protocol(&self) -> &'a str {
        self.field(16)
    }

    pub fn cs_bytes(&self) -> &'a str {
        self.field(17)
    }

    pub fn time_taken(&self) -> &'a str {
        self.field(18)
    }

    pub fn x_forwarded_for(&self) -> &'a str {
        self.field(19)
    }

    pub fn ssl_protocol(&self) -> &'a str {
        self.field(20)
    }

    pub fn ssl_cipher(&self) -> &'a str {
        self.field(21)
    }

    pub fn x_edge_response_result_type(&self) -> &'a str {
        self.field(22)
    }

    pub fn cs_protocol_version(&self) -> &'a str {
        self.field(23)
    }

    pub fn fle_status(&self) -> &'a str {
        self.field(24)
    }

    pub fn fle_encrypted_fields(&self) -> &'a str {
        self.field(25)
    }

    pub fn c_port(&self) -> &'a str {
        self.field(26)
    }

    pub fn time_to_first_byte(&self) -> &'a str {
        self.field(27)
    }

    pub fn x_edge_detailed_result_type(&self) -> &'a str {
        self.field(28)
    }

    pub fn sc_content_type(&self) -> &'a str {
        self.field(29)
    }

    pub fn sc_content_len(&self) -> &'a str {
        self.field(30)
    }

    pub fn sc_range_start(&self) -> &'a str {
        self.field(31)
    }

    pub fn sc_range_end(&self) -> &'a str {
        self.field(32)
    }
}
