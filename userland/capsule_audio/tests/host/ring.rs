extern crate alloc;
#[path = "../../src/server/ring.rs"]
mod ring;
use ring::PcmRing;

fn main() {
    let mut r = PcmRing::new();
    assert_eq!(r.push(&[1, 2, 3, 4]), 4);
    assert_eq!(r.len(), 4);
    let mut out = [9i16; 6];
    assert_eq!(r.pop_period(&mut out), 4);
    assert_eq!(out, [1, 2, 3, 4, 0, 0]);
    assert_eq!(r.len(), 0);
    let big = vec![7i16; ring::FEED_SAMPLES + 100];
    assert_eq!(r.push(&big), ring::FEED_SAMPLES);
    println!("HOSTTEST-PASS ring");
}
