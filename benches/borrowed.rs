mod utilities;
use utilities::*;

fn main() {
    divan::main();
}

mod lines {
    mod a_single_field {
        use crate::*;

        #[divan::bench]
        fn a_default<'a>() -> &'a str {
            fn parse(line: &str) -> &str {
                let item = CheckedRawLogLine::try_from(line).unwrap();
                item.sc_bytes
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn z_unsafe<'a>() -> &'a str {
            fn parse(line: &str) -> &str {
                let item = UnsafeRawLogLine::try_from(line).unwrap();
                item.sc_bytes
            }

            parse(divan::black_box(LOG_LINE_A))
        }
    }

    // note: checks a use-case of mine
    mod b_two_fields {
        use crate::*;

        #[divan::bench]
        fn a_default<'a>() -> [&'a str; 2] {
            fn parse(line: &str) -> [&str; 2] {
                let item = CheckedRawLogLine::try_from(line).unwrap();
                [item.sc_bytes, item.cs_uri_stem]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn z_unsafe<'a>() -> [&'a str; 2] {
            fn parse(line: &str) -> [&str; 2] {
                let item = UnsafeRawLogLine::try_from(line).unwrap();
                [item.sc_bytes, item.cs_uri_stem]
            }

            parse(divan::black_box(LOG_LINE_A))
        }
    }

    mod c_three_fields {
        use crate::*;

        #[divan::bench]
        fn a_default<'a>() -> [&'a str; 3] {
            fn parse(line: &str) -> [&str; 3] {
                let item = CheckedRawLogLine::try_from(line).unwrap();
                [item.sc_bytes, item.cs_bytes, item.sc_content_len]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn z_unsafe<'a>() -> [&'a str; 3] {
            fn parse(line: &str) -> [&str; 3] {
                let item = UnsafeRawLogLine::try_from(line).unwrap();
                [item.sc_bytes, item.cs_bytes, item.sc_content_len]
            }

            parse(divan::black_box(LOG_LINE_A))
        }
    }

    // note: the accessed fields are slightly unordered on purpose
    mod d_ten_fields {
        use crate::*;

        #[divan::bench]
        fn a_default<'a>() -> [&'a str; 10] {
            fn parse(line: &str) -> [&str; 10] {
                let item = CheckedRawLogLine::try_from(line).unwrap();
                [
                    item.date,
                    item.time,
                    item.sc_bytes,
                    item.cs_bytes,
                    item.sc_content_len,
                    item.cs_uri_stem,
                    item.cs_uri_stem,
                    item.cs_referer,
                    item.cs_user_agent,
                    item.c_ip,
                ]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn z_unsafe<'a>() -> [&'a str; 10] {
            fn parse(line: &str) -> [&str; 10] {
                let item = UnsafeRawLogLine::try_from(line).unwrap();
                [
                    item.date,
                    item.time,
                    item.sc_bytes,
                    item.cs_bytes,
                    item.sc_content_len,
                    item.cs_uri_stem,
                    item.cs_uri_stem,
                    item.cs_referer,
                    item.cs_user_agent,
                    item.c_ip,
                ]
            }

            parse(divan::black_box(LOG_LINE_A))
        }
    }
}

mod views {
    mod a_single_field {
        use crate::*;

        #[divan::bench]
        fn a_default<'a>() -> &'a str {
            fn parse(line: &str) -> &str {
                let item = CheckedRawLogLineView::new(line).unwrap();
                item.sc_bytes()
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn s_smart<'a>() -> &'a str {
            fn parse(line: &str) -> &str {
                let item = SmartRawLogLineView::new(line).unwrap();
                item.sc_bytes()
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        // #[divan::bench]
        // fn z_unsafe<'a>() -> &'a str {
        //     fn parse<>(line: & str) -> & str {
        //         let item = UnsafeRawLogLineView::new(line).unwrap();
        //         item.sc_bytes()
        //     }

        //     parse(divan::black_box(LOG_LINE_A))
        // }
    }

    // note: checks a use-case of mine
    mod b_two_fields {
        use crate::*;

        #[divan::bench]
        fn a_default<'a>() -> [&'a str; 2] {
            fn parse(line: &str) -> [&str; 2] {
                let item = CheckedRawLogLineView::new(line).unwrap();
                [item.sc_bytes(), item.cs_uri_stem()]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn s_smart<'a>() -> [&'a str; 2] {
            fn parse(line: &str) -> [&str; 2] {
                let item = SmartRawLogLineView::new(line).unwrap();
                [item.sc_bytes(), item.cs_uri_stem()]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        // #[divan::bench]
        // fn z_unsafe<'a>() -> [&'a str; 2] {
        //     fn parse<>(line: & str) -> [& str; 2] {
        //         let item = UnsafeRawLogLineView::new(line).unwrap();
        //         [item.sc_bytes(), item.cs_uri_stem()]
        //     }

        //     parse(divan::black_box(LOG_LINE_A))
        // }
    }

    mod c_three_fields {
        use crate::*;

        #[divan::bench]
        fn a_default<'a>() -> [&'a str; 3] {
            fn parse(line: &str) -> [&str; 3] {
                let item = CheckedRawLogLineView::new(line).unwrap();
                [item.sc_bytes(), item.cs_bytes(), item.sc_content_len()]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn s_smart<'a>() -> [&'a str; 3] {
            fn parse(line: &str) -> [&str; 3] {
                let item = SmartRawLogLineView::new(line).unwrap();
                [item.sc_bytes(), item.cs_bytes(), item.sc_content_len()]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        // #[divan::bench]
        // fn z_unsafe<'a>() -> [&'a str; 3] {
        //     fn parse<>(line: & str) -> [& str; 3] {
        //         let item = UnsafeRawLogLineView::new(line).unwrap();
        //         [item.sc_bytes(), item.cs_bytes(), item.sc_content_len()]
        //     }

        //     parse(divan::black_box(LOG_LINE_A))
        // }
    }

    // note: the accessed fields are slightly unordered on purpose
    mod d_ten_fields {
        use crate::*;

        #[divan::bench]
        fn a_default<'a>() -> [&'a str; 10] {
            fn parse(line: &str) -> [&str; 10] {
                let item = CheckedRawLogLineView::new(line).unwrap();
                [
                    item.date(),
                    item.time(),
                    item.sc_bytes(),
                    item.cs_bytes(),
                    item.sc_content_len(),
                    item.cs_uri_stem(),
                    item.cs_uri_stem(),
                    item.cs_referer(),
                    item.cs_user_agent(),
                    item.c_ip(),
                ]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        #[divan::bench]
        fn s_smart<'a>() -> [&'a str; 10] {
            fn parse(line: &str) -> [&str; 10] {
                let item = SmartRawLogLineView::new(line).unwrap();
                [
                    item.date(),
                    item.time(),
                    item.sc_bytes(),
                    item.cs_bytes(),
                    item.sc_content_len(),
                    item.cs_uri_stem(),
                    item.cs_uri_stem(),
                    item.cs_referer(),
                    item.cs_user_agent(),
                    item.c_ip(),
                ]
            }

            parse(divan::black_box(LOG_LINE_A))
        }

        // #[divan::bench]
        // fn z_unsafe<'a>() -> [&'a str; 10] {
        //     fn parse<>(line: & str) -> [& str; 10] {
        //         let item = UnsafeRawLogLineView::new(line).unwrap();
        //         [
        //             item.date(),
        //             item.time(),
        //             item.sc_bytes(),
        //             item.cs_bytes(),
        //             item.sc_content_len(),
        //             item.cs_uri_stem(),
        //             item.cs_uri_stem(),
        //             item.cs_referer(),
        //             item.cs_user_agent(),
        //             item.c_ip(),
        //         ]
        //     }

        //     parse(divan::black_box(LOG_LINE_A))
        // }
    }
}
