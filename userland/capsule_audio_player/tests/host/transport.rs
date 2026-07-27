extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use std::cell::RefCell;
use std::rc::Rc;

mod mark {
    pub fn mark(_m: &str) {}
}
#[path = "."]
mod decode {
    #[path = "../../src/decode/decoder.rs"]
    mod decoder;
    pub use decoder::{AudioInfo, Decoder};
}
#[path = "."]
mod resample {
    #[path = "../../src/resample.rs"]
    mod inner;
    pub use inner::{Resampler, OUT_RATE};
}
#[path = "."]
mod transport {
    #[path = "../../src/transport/defs.rs"]
    pub mod defs;
    #[path = "../../src/transport/machine.rs"]
    pub mod machine;
    #[path = "../../src/transport/pump.rs"]
    pub mod pump;
}

use decode::{AudioInfo, Decoder};
use transport::defs::{Fed, FeedSink, State};
use transport::machine::Transport;

struct FakeDecoder {
    frames_left: usize,
}
impl Decoder for FakeDecoder {
    fn info(&self) -> AudioInfo {
        AudioInfo { rate: 48_000, channels: 2, total_frames: Some(4) }
    }
    fn next(&mut self, out: &mut [i16]) -> usize {
        if self.frames_left == 0 {
            return 0;
        }
        let n = out.len().min(self.frames_left * 2);
        for i in 0..n {
            out[i] = 1000;
        }
        self.frames_left -= n / 2;
        n
    }
}

struct MockSink {
    fed: Rc<RefCell<Vec<i16>>>,
}
impl FeedSink for MockSink {
    fn open(&mut self, _format: u16) -> Result<(), &'static str> {
        Ok(())
    }
    fn feed(&mut self, pcm: &[i16]) -> Fed {
        self.fed.borrow_mut().extend_from_slice(pcm);
        Fed::Accepted
    }
    fn pause(&mut self) {}
    fn close(&mut self) {}
}

fn drive(t: &mut Transport, times: usize) {
    for _ in 0..times {
        t.pump();
        if t.state() == State::Stopped {
            break;
        }
    }
}

fn main() {
    let fed = Rc::new(RefCell::new(Vec::new()));
    let mut t = Transport::new(Box::new(MockSink { fed: fed.clone() }));
    t.open(Box::new(FakeDecoder { frames_left: 4 })).unwrap();
    t.play();
    drive(&mut t, 10);
    assert!(t.state() == State::Stopped, "must stop at EOF");
    assert_eq!(fed.borrow().len(), 8, "4 stereo frames = 8 i16 fed at 48k passthrough");
    assert!(fed.borrow().iter().all(|&s| s == 1000), "unity volume passthrough");

    let zfed = Rc::new(RefCell::new(Vec::new()));
    let mut z = Transport::new(Box::new(MockSink { fed: zfed.clone() }));
    z.open(Box::new(FakeDecoder { frames_left: 4 })).unwrap();
    z.set_volume(0);
    z.play();
    drive(&mut z, 10);
    assert!(z.state() == State::Stopped, "must stop at EOF");
    assert_eq!(zfed.borrow().len(), 8, "volume 0 still feeds all samples");
    assert!(zfed.borrow().iter().all(|&s| s == 0), "volume 0 zeroes every sample");

    println!("HOSTTEST-PASS transport");
}
