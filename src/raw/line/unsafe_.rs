use crate::shared::*;

/// The raw and untyped log line representation,
/// all fields are slices of the original log line.
/// Use this struct for an efficient and low-level access to the log line fields.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LogLineX<'a> {
    pub date: &'a str,
    pub time: &'a str,
    pub x_edge_location: &'a str,
    pub sc_bytes: &'a str,
    pub c_ip: &'a str,
    pub cs_method: &'a str,
    pub cs_host: &'a str,
    pub cs_uri_stem: &'a str,
    pub sc_status: &'a str,
    pub cs_referer: &'a str,
    pub cs_user_agent: &'a str,
    pub cs_uri_query: &'a str,
    pub cs_cookie: &'a str,
    pub x_edge_result_type: &'a str,
    pub x_edge_request_id: &'a str,
    pub x_host_header: &'a str,
    pub cs_protocol: &'a str,
    pub cs_bytes: &'a str,
    pub time_taken: &'a str,
    pub x_forwarded_for: &'a str,
    pub ssl_protocol: &'a str,
    pub ssl_cipher: &'a str,
    pub x_edge_response_result_type: &'a str,
    pub cs_protocol_version: &'a str,
    pub fle_status: &'a str,
    pub fle_encrypted_fields: &'a str,
    pub c_port: &'a str,
    pub time_to_first_byte: &'a str,
    pub x_edge_detailed_result_type: &'a str,
    pub sc_content_type: &'a str,
    pub sc_content_len: &'a str,
    pub sc_range_start: &'a str,
    pub sc_range_end: &'a str,
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LogLine<'a>(pub(crate) super::LogLine<'a>);

impl<'a> std::ops::Deref for LogLine<'a> {
    type Target = super::LogLine<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LogLine<'_> {
    pub fn inner(&self) -> &super::LogLine<'_> {
        &self.0
    }
}

impl<'a> TryFrom<&'a str> for LogLine<'a> {
    type Error = &'static str;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        crate::shared::valid_line(line)?;

        // SAFETY: the memory layout of `LogLine` should be the same as `LogLineArray`
        // note: I don't know if those guarantees are 100 % correct
        Ok(unsafe {
            use collect_array::CollectArray;
            std::mem::transmute::<[&'a str; crate::FIELDS], Self>(split(line).collect_array())
        })
    }
}

// <https://www.reddit.com/r/learnrust/comments/lfw6uy/comment/gn16m4o/>
mod collect_array {
    pub trait CollectArray: Sized + Iterator {
        fn collect_array<const N: usize>(self) -> [Self::Item; N] {
            assert!(N > 0 && std::mem::size_of::<Self::Item>() > 0);
            let mut array = std::mem::MaybeUninit::uninit();
            let array_ptr = array.as_mut_ptr() as *mut Self::Item;
            let mut i = 0;
            // todo: SAFETY note - why is this okay?
            unsafe {
                for item in self {
                    assert!(i < N);
                    array_ptr.add(i).write(item);
                    i += 1;
                }
                assert!(i == N);
                array.assume_init()
            }
        }
    }

    impl<T> CollectArray for T where T: Iterator {}
}
