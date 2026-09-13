//! Synthetic, deterministic inputs shared by benchmarks and ingestion examples.
//! No file access, private data, randomness, or benchmark framework dependency.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    Mixed,
    Ipv4,
    Ipv6,
    Forwarded,
    UnknownResults,
    LongFields,
    OptionalValues,
}

pub const PROFILES: [Profile; 7] = [
    Profile::Mixed,
    Profile::Ipv4,
    Profile::Ipv6,
    Profile::Forwarded,
    Profile::UnknownResults,
    Profile::LongFields,
    Profile::OptionalValues,
];

pub struct Corpus {
    pub lines: Vec<String>,
    /// Input bytes excluding line terminators, matching the parser's input.
    pub bytes: usize,
}

pub fn generate(profile: Profile, records: usize) -> Corpus {
    let lines: Vec<_> = (0..records)
        .map(|index| {
            // Coprime stride permutes each group of 100 status ranks. Filtering
            // on status < 200 + rate selects exactly rate% for multiples of 100.
            let rank = (index * 37 + 17) % 100;
            let variant = if profile == Profile::Mixed {
                PROFILES[1 + index % (PROFILES.len() - 1)]
            } else {
                profile
            };
            let mut fields = vec![
                format!(
                    "{:04}-{:02}-{:02}",
                    2000 + index % 25,
                    1 + index % 12,
                    1 + index % 28
                ),
                format!(
                    "{:02}:{:02}:{:02}",
                    index % 24,
                    (index / 24) % 60,
                    (index * 7) % 60
                ),
                format!("TEST{}-C{}", index % 31, index % 4),
                (392 + index * 13).to_string(),
                format!("192.0.2.{}", 1 + index % 254),
                "GET".into(),
                "synthetic.example.net".into(),
                format!("/objects/{index}/index.html"),
                (200 + rank).to_string(),
                "-".into(),
                "SyntheticAgent/1.0".into(),
                "-".into(),
                "-".into(),
                "Hit".into(),
                format!("synthetic-request-{index:016x}"),
                "synthetic.example.net".into(),
                "https".into(),
                (23 + index % 2000).to_string(),
                format!("{}.{:03}", index % 3, index % 1000),
                "-".into(),
                "TLSv1.2".into(),
                "ECDHE-RSA-AES128-GCM-SHA256".into(),
                "Hit".into(),
                "HTTP/2.0".into(),
                "-".into(),
                "-".into(),
                (1024 + index % 64000).to_string(),
                format!("0.{:03}", index % 1000),
                "Hit".into(),
                "text/html".into(),
                (78 + index % 10000).to_string(),
                "-".into(),
                "-".into(),
            ];
            match variant {
                Profile::Ipv6 => fields[4] = format!("2001:db8::{:x}", 1 + index % 65534),
                Profile::Forwarded => {
                    fields[19] = match index % 3 {
                        0 => "192.0.2.7",
                        1 => "192.0.2.7,\\x202001:db8::7",
                        _ => "001.002.003.004,\\x20[2001:db8::7]:443,\\x20198.51.100.9:8080",
                    }
                    .into();
                }
                Profile::UnknownResults => {
                    fields[13] = format!("FutureResult{}", index % 19);
                    fields[22] = format!("FutureResponse{}", index % 11);
                    fields[28] = format!("FutureDetail{}", index % 23);
                }
                Profile::LongFields => {
                    fields[7] = format!("/objects/{index}/{}", "segment/".repeat(128));
                    fields[10] = "SyntheticAgent%20".repeat(128);
                    fields[11] = format!("id={index}&payload={}", "a1b2c3".repeat(256));
                }
                Profile::OptionalValues => {
                    fields[9] = "https://referrer.example/".into();
                    fields[11] = format!("id={index}");
                    fields[12] = "session=synthetic".into();
                    fields[24] = "Processed".into();
                    fields[25] = "2".into();
                    fields[31] = "0".into();
                    fields[32] = "77".into();
                }
                Profile::Mixed | Profile::Ipv4 => {}
            }
            fields.join("\t")
        })
        .collect();
    let bytes = lines.iter().map(String::len).sum();
    Corpus { lines, bytes }
}
