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

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

    match prove_threads() {
        Ok(detail) => println!("NONOS std proof PASS threads: {detail}"),
        Err(detail) => println!("NONOS std proof FAIL threads: {detail}"),
    }

    println!("NONOS STD PROOF DONE");
}

// Three spawned threads hammer one mutex-guarded counter and report in
// over an mpsc channel; the parent joins all of them. This exercises
// MTSP spawn, per-thread TLS, the futex-backed Mutex under contention,
// channel park/unpark, sleep, and join.
fn prove_threads() -> Result<String, String> {
    const ROUNDS: u64 = 1000;
    let counter = Arc::new(Mutex::new(0u64));
    let (tx, rx) = mpsc::channel::<u64>();
    let mut handles = Vec::new();
    for id in 1..=3u64 {
        let counter = Arc::clone(&counter);
        let tx = tx.clone();
        let handle = thread::Builder::new()
            .name(format!("worker-{id}"))
            .spawn(move || {
                for _ in 0..ROUNDS {
                    *counter.lock().unwrap_or_else(|e| e.into_inner()) += 1;
                }
                thread::sleep(Duration::from_millis(2));
                let _ = tx.send(id);
            })
            .map_err(|e| format!("spawn worker-{id}: {e}"))?;
        handles.push(handle);
    }
    drop(tx);

    let mut seen: Vec<u64> = rx.iter().collect();
    seen.sort_unstable();

    for handle in handles {
        handle.join().map_err(|_| "join panicked".to_string())?;
    }

    let total = *counter.lock().unwrap_or_else(|e| e.into_inner());
    if total != ROUNDS * 3 {
        return Err(format!("counter={total}, expected {}", ROUNDS * 3));
    }
    if seen != [1, 2, 3] {
        return Err(format!("channel reported {seen:?}, expected [1, 2, 3]"));
    }
    Ok(format!("3 threads joined, counter={total}, channel={seen:?}"))
}
