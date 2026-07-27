extern crate alloc;
#[path = "../../src/waveform.rs"]
mod waveform;
use waveform::{from_samples, Waveform};

fn main() {
    let some: Vec<i16> = (0..4096).map(|i| (i % 100) as i16).collect();
    let w: Waveform = from_samples(&some, 96);
    assert_eq!(w.buckets.len(), 96);

    let silent = from_samples(&[0i16; 4096], 32);
    assert_eq!(silent.buckets.len(), 32);
    assert!(silent.buckets.iter().all(|&b| b == 0));

    let mut spike = vec![0i16; 4096];
    spike[3000] = i16::MAX;
    let sw = from_samples(&spike, 32);
    let spike_bucket = 3000 / (4096 / 32);
    assert_eq!(sw.buckets[spike_bucket], 255);
    assert_eq!(sw.buckets[0], 0);

    let ramp: Vec<i16> = (0..8192)
        .map(|i| ((i as i64 * 30000) / 8192) as i16)
        .collect();
    let rw = from_samples(&ramp, 32);
    assert!(rw.buckets[31] >= rw.buckets[0]);

    let empty = from_samples(&[], 8);
    assert_eq!(empty.buckets.len(), 8);
    assert!(empty.buckets.iter().all(|&b| b == 0));

    println!("HOSTTEST-PASS waveform");
}
