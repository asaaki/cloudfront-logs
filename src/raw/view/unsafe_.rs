/// View into a borrowed log line
///
/// Unlike [`RawLogLine`](crate::raw::RawLogLine), this struct does not compute all the fields upfront,
/// but instead provides methods to access the fields on demand.
///
/// This can be useful when you need to access only a few fields from a log line.
#[derive(Debug)]
pub struct LogLineView<'a> {
    // line: &'a str,
    bytes: &'a [u8],
    idx: Vec<usize>,
    // iter: Windows<'a, usize>,
}

impl<'a> LogLineView<'a> {
    /// Creates a new `RawLogLineView` from a borrowed log line
    ///
    /// The line is checked for the correct number of fields and
    /// that it's not a comment line (like the version or fields header).
    ///
    pub fn new(line: &'a str) -> Result<Self, &'static str> {
        crate::valid_line(line)?;
        let bytes = line.as_bytes();

    let mut m: std::collections::VecDeque<_> = memchr::memmem::find_iter(bytes, b"\t").collect();
    m.push_front(0);
    m.push_back(bytes.len());
    // let m = m.make_contiguous();

        let idx = m.make_contiguous().to_vec();
        // let iter = mm.windows(2);
        Ok(LogLineView { bytes, idx })
    }

    fn field(&self, n: usize) -> &'a str {
        let w = self.idx.windows(2).clone().nth(n).unwrap();
        let a = if w[0] == 0 { 0 } else { w[0] + 1 };
        unsafe { std::str::from_utf8_unchecked(&self.bytes[a..w[1]]) }
    }

    /// Returns the date field of the log line
    ///
    /// This is the unchecked version (using unwrap).
    /// Since the struct was checked for the correct number of fields,
    /// this method should never panic.
    pub fn date(&self) -> &'a str {
        // self.field(0)
        self.field(0)
    }

    pub fn time(&self) -> &'a str {
        // self.field(1)
        self.field(1)
    }

    pub fn x_edge_location(&self) -> &'a str {
        // self.field(2)
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
