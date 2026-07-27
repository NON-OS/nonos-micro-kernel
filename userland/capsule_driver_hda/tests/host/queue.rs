extern crate alloc;
#[path = "../../src/audio/queue.rs"]
mod queue;
use queue::*;

fn main() {
    let mut q = PcmQueue::new();
    assert_eq!(q.push(&[1, 2, 3, 4]), 4);
    assert_eq!(q.len(), 4);

    let mut out = [9u8; 6];
    let got = q.pop_into(&mut out);
    assert_eq!(got, 4);
    assert_eq!(&out[..4], &[1, 2, 3, 4]);
    assert_eq!(&out[4..], &[0, 0]);
    assert_eq!(q.len(), 0);

    let big = alloc::vec![7u8; QUEUE_BYTES + 100];
    let acc = q.push(&big);
    assert!(acc <= QUEUE_BYTES && acc < big.len());
    assert!(q.is_full_for(1));

    q.clear();
    assert_eq!(q.len(), 0);
    assert!(!q.is_full_for(QUEUE_BYTES));

    println!("HOSTTEST-PASS queue");
}
