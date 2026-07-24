extern crate alloc;
#[path = "."]
mod server {
    #[path = "../../src/server/ring.rs"]
    pub mod ring;
    #[path = "../../src/server/streams.rs"]
    pub mod streams;
}
use server::streams::*;

fn main() {
    let mut t = StreamTable::new();
    let id1 = t.open(1).unwrap();
    let id2 = t.open(2).unwrap();
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);

    assert_eq!(t.feed(id1, &[1, 2, 3, 4]), 0);

    let huge = alloc::vec![7i16; server::ring::FEED_SAMPLES];
    assert_eq!(t.feed(id1, &huge), -11);

    assert_eq!(t.feed(99, &[1, 2]), -22);

    assert!(t.set_paused(id2, true));
    assert_eq!(t.iter_active().count(), 1);

    assert!(t.close(id1));
    let id3 = t.open(3).unwrap();
    assert_eq!(id3, 3);

    println!("HOSTTEST-PASS streams");
}
