extern crate alloc;
#[path = "."]
mod decode {
    #[path = "../../src/decode/decoder.rs"]
    pub mod decoder;
    #[path = "../../src/decode/minimp3_sys.rs"]
    pub mod minimp3_sys;
    #[path = "../../src/decode/mp3_frame.rs"]
    pub mod mp3_frame;
    #[path = "../../src/decode/mp3.rs"]
    pub mod mp3;
}
use decode::decoder::Decoder;
use decode::mp3::Mp3Decoder;

static BOOT_TONE: &[u8] = include_bytes!("../../../capsule_vfs/src/testassets/boot_tone.mp3");

fn main() {
    let mut dec = Mp3Decoder::new(BOOT_TONE.to_vec()).unwrap();
    let info = dec.info();
    assert_eq!(info.rate, 44100);
    assert_eq!(info.channels, 1);

    let mut pcm: Vec<i16> = Vec::new();
    let mut out = [0i16; 4096];
    loop {
        let n = dec.next(&mut out);
        if n == 0 {
            break;
        }
        pcm.extend_from_slice(&out[..n]);
    }

    let frames = pcm.len() / info.channels as usize;
    assert!(frames > 0, "no frames decoded");
    assert!(
        (18000..=26000).contains(&frames),
        "decoded frame count {} out of tolerance",
        frames
    );

    println!("frames={}", frames);
    println!("HOSTTEST-PASS mp3");
}
