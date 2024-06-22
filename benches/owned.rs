mod utilities;
use utilities::*;

fn main() {
    divan::main();
}

mod simple {
    mod from_str {
        use crate::*;

        #[divan::bench]
        fn a_single_field() -> u64 {
            fn parse(line: &str) -> u64 {
                let item = SimpleLogLine::try_from(line).unwrap();
                item.sc_bytes
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn b_two_fields() -> (u64, String) {
            fn parse(line: &str) -> (u64, String) {
                let item = SimpleLogLine::try_from(line).unwrap();
                (item.sc_bytes, item.cs_uri_stem)
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn c_three_fields() -> [u64; 3] {
            fn parse(line: &str) -> [u64; 3] {
                let item = SimpleLogLine::try_from(line).unwrap();
                [item.sc_bytes, item.cs_bytes, item.sc_content_len]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn d_ten_fields() -> (
            String,
            String,
            u64,
            u64,
            u64,
            String,
            Option<String>,
            Option<String>,
            String,
            IpAddr,
        ) {
            fn parse(
                line: &str,
            ) -> (
                String,
                String,
                u64,
                u64,
                u64,
                String,
                Option<String>,
                Option<String>,
                String,
                IpAddr,
            ) {
                let item = SimpleLogLine::try_from(line).unwrap();
                (
                    item.date,
                    item.time,
                    item.sc_bytes,
                    item.cs_bytes,
                    item.sc_content_len,
                    item.cs_uri_stem,
                    item.cs_uri_query,
                    item.cs_referer,
                    item.cs_user_agent,
                    item.c_ip,
                )
            }

            parse(divan::black_box(LOG_LINE_A))
        }
    }

    mod from_raw_line {
        use crate::*;

        #[divan::bench]
        fn a_single_field() -> u64 {
            fn parse(line: &str) -> u64 {
                let raw = CheckedRawLogLine::try_from(line).unwrap();
                let item = SimpleLogLine::try_from(raw).unwrap();
                item.sc_bytes
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn b_two_fields() -> (u64, String) {
            fn parse(line: &str) -> (u64, String) {
                let raw = CheckedRawLogLine::try_from(line).unwrap();
                let item = SimpleLogLine::try_from(raw).unwrap();
                (item.sc_bytes, item.cs_uri_stem)
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn c_three_fields() -> [u64; 3] {
            fn parse(line: &str) -> [u64; 3] {
                let raw = CheckedRawLogLine::try_from(line).unwrap();
                let item = SimpleLogLine::try_from(raw).unwrap();
                [item.sc_bytes, item.cs_bytes, item.sc_content_len]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn d_ten_fields() -> (
            String,
            String,
            u64,
            u64,
            u64,
            String,
            Option<String>,
            Option<String>,
            String,
            IpAddr,
        ) {
            fn parse(
                line: &str,
            ) -> (
                String,
                String,
                u64,
                u64,
                u64,
                String,
                Option<String>,
                Option<String>,
                String,
                IpAddr,
            ) {
                let raw = CheckedRawLogLine::try_from(line).unwrap();
                let item = SimpleLogLine::try_from(raw).unwrap();
                (
                    item.date,
                    item.time,
                    item.sc_bytes,
                    item.cs_bytes,
                    item.sc_content_len,
                    item.cs_uri_stem,
                    item.cs_uri_query,
                    item.cs_referer,
                    item.cs_user_agent,
                    item.c_ip,
                )
            }

            parse(divan::black_box(LOG_LINE_A))
        }
    }

    mod try_from_with_raw {
        use crate::*;

        #[divan::bench]
        fn a_single_field() -> u64 {
            fn parse(line: &str) -> u64 {
                let item = SimpleLogLine::try_from_with_raw(line).unwrap();
                item.sc_bytes
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn b_two_fields() -> (u64, String) {
            fn parse(line: &str) -> (u64, String) {
                let item = SimpleLogLine::try_from_with_raw(line).unwrap();
                (item.sc_bytes, item.cs_uri_stem)
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn c_three_fields() -> [u64; 3] {
            fn parse(line: &str) -> [u64; 3] {
                let item = SimpleLogLine::try_from_with_raw(line).unwrap();
                [item.sc_bytes, item.cs_bytes, item.sc_content_len]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn d_ten_fields() -> (
            String,
            String,
            u64,
            u64,
            u64,
            String,
            Option<String>,
            Option<String>,
            String,
            IpAddr,
        ) {
            fn parse(
                line: &str,
            ) -> (
                String,
                String,
                u64,
                u64,
                u64,
                String,
                Option<String>,
                Option<String>,
                String,
                IpAddr,
            ) {
                let item = SimpleLogLine::try_from_with_raw(line).unwrap();
                (
                    item.date,
                    item.time,
                    item.sc_bytes,
                    item.cs_bytes,
                    item.sc_content_len,
                    item.cs_uri_stem,
                    item.cs_uri_query,
                    item.cs_referer,
                    item.cs_user_agent,
                    item.c_ip,
                )
            }

            parse(divan::black_box(LOG_LINE_A))
        }
    }
}

mod typed {
    mod from_str {
        use crate::*;

        #[divan::bench]
        fn a_single_field() -> u64 {
            fn parse(line: &str) -> u64 {
                let item = TypedLogLine::try_from(line).unwrap();
                item.sc_bytes
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn b_two_fields() -> (u64, String) {
            fn parse(line: &str) -> (u64, String) {
                let item = TypedLogLine::try_from(line).unwrap();
                (item.sc_bytes, item.cs_uri_stem)
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn c_three_fields() -> [u64; 3] {
            fn parse(line: &str) -> [u64; 3] {
                let item = TypedLogLine::try_from(line).unwrap();
                [item.sc_bytes, item.cs_bytes, item.sc_content_len]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn d_ten_fields() -> (
            Date,
            Time,
            u64,
            u64,
            u64,
            String,
            Option<String>,
            Option<String>,
            String,
            IpAddr,
        ) {
            fn parse(
                line: &str,
            ) -> (
                Date,
                Time,
                u64,
                u64,
                u64,
                String,
                Option<String>,
                Option<String>,
                String,
                IpAddr,
            ) {
                let item = TypedLogLine::try_from(line).unwrap();
                (
                    item.date,
                    item.time,
                    item.sc_bytes,
                    item.cs_bytes,
                    item.sc_content_len,
                    item.cs_uri_stem,
                    item.cs_uri_query,
                    item.cs_referer,
                    item.cs_user_agent,
                    item.c_ip,
                )
            }

            parse(divan::black_box(LOG_LINE_A))
        }
    }

    mod from_raw_line {
        use crate::*;

        #[divan::bench]
        fn a_single_field() -> u64 {
            fn parse(line: &str) -> u64 {
                let raw = CheckedRawLogLine::try_from(line).unwrap();
                let item = TypedLogLine::try_from(raw).unwrap();
                item.sc_bytes
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn b_two_fields() -> (u64, String) {
            fn parse(line: &str) -> (u64, String) {
                let raw = CheckedRawLogLine::try_from(line).unwrap();
                let item = TypedLogLine::try_from(raw).unwrap();
                (item.sc_bytes, item.cs_uri_stem)
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn c_three_fields() -> [u64; 3] {
            fn parse(line: &str) -> [u64; 3] {
                let raw = CheckedRawLogLine::try_from(line).unwrap();
                let item = TypedLogLine::try_from(raw).unwrap();
                [item.sc_bytes, item.cs_bytes, item.sc_content_len]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn d_ten_fields() -> (
            Date,
            Time,
            u64,
            u64,
            u64,
            String,
            Option<String>,
            Option<String>,
            String,
            IpAddr,
        ) {
            fn parse(
                line: &str,
            ) -> (
                Date,
                Time,
                u64,
                u64,
                u64,
                String,
                Option<String>,
                Option<String>,
                String,
                IpAddr,
            ) {
                let raw = CheckedRawLogLine::try_from(line).unwrap();
                let item = TypedLogLine::try_from(raw).unwrap();
                (
                    item.date,
                    item.time,
                    item.sc_bytes,
                    item.cs_bytes,
                    item.sc_content_len,
                    item.cs_uri_stem,
                    item.cs_uri_query,
                    item.cs_referer,
                    item.cs_user_agent,
                    item.c_ip,
                )
            }

            parse(divan::black_box(LOG_LINE_A))
        }
    }
}
