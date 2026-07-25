extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[path = "../../src/decode/mod.rs"]
mod decode;
#[path = "../../src/resample.rs"]
mod resample;
mod audio_client {
    pub enum FeedResult { Fed, WouldBlock }
}
#[path = "../../src/transport.rs"]
mod transport;

use decode::{AudioInfo, Decoder};
use transport::{FeedSink, State, Transport};
use audio_client::FeedResult;

struct FakeDecoder { frames_left: usize }
impl Decoder for FakeDecoder {
    fn info(&self) -> AudioInfo { AudioInfo { rate: 48_000, channels: 2, total_frames: Some(4) } }
    fn next(&mut self, out: &mut [i16]) -> usize {
        if self.frames_left == 0 { return 0; }
        let n = out.len().min(self.frames_left * 2);
        for i in 0..n { out[i] = 1000; }
        self.frames_left -= n / 2;
        n
    }
}

struct MockSink { fed: Vec<i16>, block_after: usize }
impl FeedSink for MockSink {
    fn feed(&mut self, pcm: &[i16]) -> FeedResult {
        if self.fed.len() >= self.block_after { return FeedResult::WouldBlock; }
        self.fed.extend_from_slice(pcm);
        FeedResult::Fed
    }
}

fn drive(t: &mut Transport<MockSink>, times: usize) {
    for _ in 0..times { t.pump(); if t.state() == State::Stopped { break; } }
}

fn main() {
    let mut t = Transport::new(MockSink { fed: Vec::new(), block_after: usize::MAX });
    t.open(Box::new(FakeDecoder { frames_left: 4 }));
    assert_eq!(t.dur_frames(), 4, "dur_frames from total_frames");
    t.play();
    drive(&mut t, 10);
    assert!(t.state() == State::Stopped, "must stop at EOF");
    assert_eq!(t.pos_frames(), 4, "pos should equal fed output frames");
    assert_eq!(t.sink().fed.len(), 8, "4 frames = 8 interleaved samples");
    assert!(t.sink().fed.iter().all(|&s| s == 1000), "unity volume passthrough");

    let mut z = Transport::new(MockSink { fed: Vec::new(), block_after: usize::MAX });
    z.open(Box::new(FakeDecoder { frames_left: 4 }));
    z.set_volume(0);
    z.play();
    drive(&mut z, 10);
    assert_eq!(z.sink().fed.len(), 8, "volume 0 still feeds all samples");
    assert!(z.sink().fed.iter().all(|&s| s == 0), "volume 0 zeroes every sample");

    let mut b = Transport::new(MockSink { fed: Vec::new(), block_after: 4 });
    b.open(Box::new(FakeDecoder { frames_left: 2048 }));
    b.play();
    b.pump();
    assert_eq!(b.sink().fed.len(), 2048, "first block accepted");
    assert_eq!(b.pending_len(), 0, "nothing pending yet");
    b.pump();
    assert_eq!(b.sink().fed.len(), 2048, "second block blocked, none dropped");
    assert_eq!(b.pending_len(), 2048, "blocked block retained in pending");
    let before = b.sink().fed.len() + b.pending_len();
    b.pump();
    assert_eq!(b.sink().fed.len() + b.pending_len(), before, "no samples lost on retry");
    assert_eq!(b.pending_len(), 2048, "pending retained while blocked");

    println!("HOSTTEST-PASS transport");
}
