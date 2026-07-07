// Unmodified-std program: the nonos-rt _start shim calls this main.
// serde_json, regex, and base64 are pulled straight from crates.io
// with no source edits, which is the whole point of the proof.
use base64::Engine;
use serde_json::Value;

fn main() {
    let raw = r#"{"os":"nonos","crate":"serde_json","nums":[3,7,11,179],"ok":true}"#;
    let Ok(v): Result<Value, _> = serde_json::from_str(raw) else {
        println!("NONOS std proof failed: json parse");
        return;
    };
    let os = v["os"].as_str().unwrap_or("?");
    let sum: i64 =
        v["nums"].as_array().map(|a| a.iter().filter_map(Value::as_i64).sum()).unwrap_or(-1);
    let ok = v["ok"].as_bool().unwrap_or(false);
    let hay = "nonos std capsule: serde_json regex base64";
    let Ok(re) = regex::Regex::new(r"\b[a-z_]{5,}\b") else {
        println!("NONOS std proof failed: regex compile");
        return;
    };
    let hits = re.find_iter(hay).count();
    let b64 = base64::engine::general_purpose::STANDARD.encode(b"nonos");
    println!(
        "NONOS ran crates.io serde_json+regex+base64: os={os}, nums sum={sum}, ok={ok}, regex hits={hits}, base64={b64}"
    );
}
