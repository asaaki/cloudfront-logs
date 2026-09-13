//! Compare selected date/time parsing with its original backend implementation.
//! cargo run --release --no-default-features --features jiff --example selected-datetime-performance -- --bench --sample-count 100 --sample-size 10
#![forbid(unsafe_code)]

#[cfg(any(feature = "jiff", feature = "time"))]
mod experiment {
    use cloudfront_logs::{
        Validated,
        borrowed::structured::{DateTimeBackend, Logline, Selected},
    };
    use divan::{Bencher, black_box};
    #[cfg(feature = "jiff")]
    use jiff::civil::{Date, Time};
    #[cfg(feature = "time")]
    use time::{Date, Time};

    struct Original;

    impl DateTimeBackend for Original {
        type Date<'a> = Date;
        type Time<'a> = Time;

        fn parse<'a>(date: &'a str, time: &'a str) -> Result<(Date, Time), &'static str> {
            #[cfg(feature = "jiff")]
            let parsed = (
                date.parse().map_err(|_error| "date invalid")?,
                time.parse().map_err(|_error| "time invalid")?,
            );
            #[cfg(feature = "time")]
            let parsed = (
                Date::parse(
                    date,
                    time::macros::format_description!("[year]-[month]-[day]"),
                )
                .map_err(|_error| "date invalid")?,
                Time::parse(
                    time,
                    time::macros::format_description!("[hour]:[minute]:[second]"),
                )
                .map_err(|_error| "time invalid")?,
            );
            Ok(parsed)
        }
    }

    fn corpus(fallback: bool) -> Vec<(String, String, String)> {
        (0..100)
            .map(|i| {
                let year = 1970 + i;
                let month = 1 + i * 7 % 12;
                let day = 1 + i * 13 % 28;
                #[cfg(feature = "jiff")]
                let date = if fallback {
                    format!("-{year:06}-{month:02}-{day:02}")
                } else {
                    format!("{year:04}-{month:02}-{day:02}")
                };
                #[cfg(feature = "time")]
                let date = format!("{}{year:04}-{month:02}-{day:02}", if fallback { "-" } else { "" });
                let time = format!("{:02}:{:02}:{:02}", i * 7 % 24, i * 17 % 60, i * 23 % 60);
                // Jiff accepts fractional seconds; the time backend's configured
                // format does not, so its fallback workload changes only the date.
                #[cfg(feature = "jiff")]
                let time = if fallback { format!("{time}.123456789") } else { time };
                let line = format!("{date}\t{time}\tLAX1\t392\t192.0.2.100\tGET\td.example\t/index.html\t200\t-\tagent\t-\t-\tHit\tid\td.example\thttps\t23\t0.001\t-\tTLSv1.2\tcipher\tHit\tHTTP/2.0\t-\t-\t11040\t0.001\tHit\ttext/html\t78\t-\t-");
                assert_eq!(Selected::parse(&date, &time), Original::parse(&date, &time));
                assert!(Selected::parse(&date, &time).is_ok());
                (date, time, line)
            })
            .collect()
    }

    #[divan::bench(args = [false, true], types = [Selected, Original])]
    fn helper<D: DateTimeBackend>(bencher: Bencher<'_, '_>, fallback: bool) {
        let rows = corpus(fallback);
        bencher.bench_local(|| {
            for (date, time, _) in &rows {
                black_box(D::parse(black_box(date), black_box(time)).unwrap());
            }
        });
    }

    #[divan::bench(args = [false, true], types = [Selected, Original])]
    fn full_record<D: DateTimeBackend>(bencher: Bencher<'_, '_>, fallback: bool) {
        let rows = corpus(fallback);
        bencher.bench_local(|| {
            for (_, _, line) in &rows {
                let record = Logline::<Validated, D>::try_from(black_box(line.as_str())).unwrap();
                black_box(&record);
            }
        });
    }
}

fn main() {
    println!("Times are per 100 varied records; construction and destruction included.");
    println!("false = exact format; true = negative year (plus fractional seconds for Jiff).");
    println!("Corpus generation and equality checks are outside timing.");
    #[cfg(any(feature = "jiff", feature = "time"))]
    divan::main();
    #[cfg(not(any(feature = "jiff", feature = "time")))]
    println!("Enable jiff or time to run this experiment.");
}
