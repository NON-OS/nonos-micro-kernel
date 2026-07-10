// Unmodified-std program: the nonos-rt _start shim calls this main.
// Four proofs in one binary, each through a different std subsystem the
// NONOS platform layer backs with real syscalls: crates.io code (serde_json,
// regex, base64 straight from the registry), threads with locks and
// channels, file I/O over the VFS capsule, and a TCP socket through the
// userland net stack. Each section prints one PASS or FAIL line so the
// boot log is the evidence.

use base64::Engine;
use serde_json::Value;

use std::io::{Read, Write};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    report("crates.io", prove_crates());
    report("threads", prove_threads());
    report("env", prove_env());
    report("fs", prove_fs());
    report("socket", prove_socket());
    println!("NONOS STD PROOF DONE");
}

fn report(what: &str, outcome: Result<String, String>) {
    match outcome {
        Ok(detail) => println!("NONOS std proof PASS {what}: {detail}"),
        Err(detail) => println!("NONOS std proof FAIL {what}: {detail}"),
    }
}

fn prove_crates() -> Result<String, String> {
    let raw = r#"{"os":"nonos","crate":"serde_json","nums":[3,7,11,179],"ok":true}"#;
    let v: Value = serde_json::from_str(raw).map_err(|e| format!("json parse: {e}"))?;
    let os = v["os"].as_str().unwrap_or("?");
    let sum: i64 =
        v["nums"].as_array().map(|a| a.iter().filter_map(Value::as_i64).sum()).unwrap_or(-1);
    let re = regex::Regex::new(r"\b[a-z_]{5,}\b").map_err(|e| format!("regex: {e}"))?;
    let hits = re.find_iter("nonos std capsule: serde_json regex base64").count();
    let b64 = base64::engine::general_purpose::STANDARD.encode(b"nonos");
    if os == "nonos" && sum == 200 && hits == 5 && b64 == "bm9ub3M=" {
        Ok(format!("os={os}, sum={sum}, regex hits={hits}, base64={b64}"))
    } else {
        Err(format!("unexpected results: os={os}, sum={sum}, hits={hits}, b64={b64}"))
    }
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
            .spawn(move || {
                for _ in 0..ROUNDS {
                    if let Ok(mut guard) = counter.lock() {
                        *guard += id;
                    }
                }
                thread::sleep(Duration::from_millis(2));
                let _ = tx.send(id);
            })
            .map_err(|e| format!("spawn: {e}"))?;
        handles.push(handle);
    }
    drop(tx);
    let mut reported: Vec<u64> = rx.iter().collect();
    reported.sort_unstable();
    for handle in handles {
        handle.join().map_err(|_| "join: child panicked".to_string())?;
    }
    let total = *counter.lock().map_err(|_| "poisoned lock".to_string())?;
    let expect = ROUNDS * (1 + 2 + 3);
    if total == expect && reported == [1, 2, 3] {
        Ok(format!("3 threads, sum={total}, channel reports={reported:?}"))
    } else {
        Err(format!("sum={total} (want {expect}), reports={reported:?}"))
    }
}

// Set and read back a process-local environment variable, and check the
// fixed working directory and temp dir the capsule model defines.
fn prove_env() -> Result<String, String> {
    std::env::set_var("NONOS_PROOF", "on");
    let val = std::env::var("NONOS_PROOF").map_err(|e| format!("var: {e}"))?;
    let cwd = std::env::current_dir().map_err(|e| format!("cwd: {e}"))?;
    let tmp = std::env::temp_dir();
    if val == "on" && cwd.as_os_str() == "/" && tmp.as_os_str() == "/tmp" {
        Ok(format!("set/get ok, cwd={}, temp_dir={}", cwd.display(), tmp.display()))
    } else {
        Err(format!("val={val}, cwd={}, tmp={}", cwd.display(), tmp.display()))
    }
}

// Retry an operation while the service it needs is still coming up. The
// capsule can be spawned before the vfs and net capsules register, so
// waiting is normal application behavior, not proof theater; a bounded
// deadline keeps a genuinely broken path loud.
fn with_retry<T>(
    what: &str,
    deadline: Duration,
    mut op: impl FnMut() -> Result<T, String>,
) -> Result<T, String> {
    let start = std::time::Instant::now();
    loop {
        match op() {
            Ok(v) => return Ok(v),
            Err(e) if start.elapsed() >= deadline => {
                return Err(format!("{what} (after {:?}): {e}", start.elapsed()));
            }
            Err(_) => thread::sleep(Duration::from_millis(500)),
        }
    }
}

// Write, stat, seek, read back, and delete a scratch file through std::fs;
// every call crosses the IPC boundary into the VFS capsule.
fn prove_fs() -> Result<String, String> {
    use std::io::{Seek, SeekFrom};
    let path = "/std_proof_scratch.txt";
    let payload = "written through std::fs over the NONOS vfs\n";
    with_retry("vfs not up", Duration::from_secs(60), || {
        std::fs::write(path, payload).map_err(|e| format!("write: {e}"))
    })?;
    let meta = std::fs::metadata(path).map_err(|e| format!("stat: {e}"))?;
    let back = std::fs::read_to_string(path).map_err(|e| format!("read: {e}"))?;
    let mut file = std::fs::File::open(path).map_err(|e| format!("reopen: {e}"))?;
    let end = file.seek(SeekFrom::End(0)).map_err(|e| format!("seek end: {e}"))?;
    file.seek(SeekFrom::Start(8)).map_err(|e| format!("seek start: {e}"))?;
    let mut tail = String::new();
    file.read_to_string(&mut tail).map_err(|e| format!("read at offset: {e}"))?;
    drop(file);
    std::fs::remove_file(path).map_err(|e| format!("unlink: {e}"))?;
    let gone = std::fs::metadata(path).is_err();
    if back == payload
        && meta.len() == payload.len() as u64
        && end == payload.len() as u64
        && tail == payload[8..]
        && gone
    {
        Ok(format!("wrote+read {} bytes, seek(end)={end}, offset read ok, unlinked", back.len()))
    } else {
        Err(format!(
            "roundtrip mismatch: read {} bytes, len={}, end={end}, tail={} bytes, gone={gone}",
            back.len(),
            meta.len(),
            tail.len()
        ))
    }
}

// Connect to the QEMU host gateway, send a line, and read the responder's
// reply: TcpStream over net.sockets, through the whole userland stack.
// The proof harness listens on the host side; without it this reports
// FAIL with the connect error rather than pretending.
fn prove_socket() -> Result<String, String> {
    let mut stream = with_retry("net not up", Duration::from_secs(120), || {
        std::net::TcpStream::connect("10.0.2.2:7878").map_err(|e| format!("connect: {e}"))
    })?;
    stream.write_all(b"NONOS std socket hello\n").map_err(|e| format!("send: {e}"))?;
    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf).map_err(|e| format!("recv: {e}"))?;
    let reply = String::from_utf8_lossy(&buf[..n]);
    let reply = reply.trim();
    if reply.contains("std-pong") {
        Ok(format!("sent 23 bytes, reply={reply:?}"))
    } else {
        Err(format!("unexpected reply ({n} bytes): {reply:?}"))
    }
}
