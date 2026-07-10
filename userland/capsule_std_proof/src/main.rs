// Unmodified-std app: the nonos-rt _start shim calls this main. It is a small
// real tool built only from crates.io libraries with no source edits, packaged
// and installed live on NONOS through the std platform layer and run only after
// the kernel verifies its signature, manifest hash, and zero-knowledge proof.
//
// Given a JSON document (as the first argument, or a built-in sample), it uses
// serde_json to parse it and report real results: the record count, the sum and
// max of a numeric field, and a base64 digest of the document. This shows an
// off-the-shelf Rust library doing real work on real input, attested and live.

use base64::Engine;
use serde_json::Value;

const SAMPLE: &str = r#"{"service":"nonos","records":[
    {"id":1,"amount":40},{"id":2,"amount":65},{"id":3,"amount":95}]}"#;

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| SAMPLE.to_string());

    let doc: Value = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(e) => {
            println!("std_proof: not valid JSON ({e})");
            return;
        }
    };

    let records = doc["records"].as_array().cloned().unwrap_or_default();
    let amounts: Vec<i64> = records.iter().filter_map(|r| r["amount"].as_i64()).collect();
    let count = records.len();
    let sum: i64 = amounts.iter().sum();
    let max = amounts.iter().copied().max().unwrap_or(0);
    let service = doc["service"].as_str().unwrap_or("unknown");
    let digest = base64::engine::general_purpose::STANDARD.encode(input.as_bytes());

    println!("std_proof: crates.io serde_json + base64, running installed on NONOS");
    println!("  service : {service}");
    println!("  records : {count}");
    println!("  amount  : sum={sum} max={max}");
    println!("  digest  : {}", &digest[..digest.len().min(44)]);
    println!("NONOS STD PROOF DONE");
}
