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
    #[path = "../../src/decode/wav.rs"]
    pub mod wav;
    #[path = "../../src/decode/wav_pcm.rs"]
    pub mod wav_pcm;
    #[path = "../../src/decode/sniff.rs"]
    pub mod sniff;
}
use decode::sniff::open;

fn wav_bytes(channels: u16, bits: u16, rate: u32, frame_bytes: &[u8]) -> Vec<u8> {
    let block_align = channels * (bits / 8);
    let data_len = frame_bytes.len() as u32;
    let mut v = Vec::new();
    v.extend_from_slice(b"RIFF");
    v.extend_from_slice(&(36 + data_len).to_le_bytes());
    v.extend_from_slice(b"WAVE");
    v.extend_from_slice(b"fmt ");
    v.extend_from_slice(&16u32.to_le_bytes());
    v.extend_from_slice(&1u16.to_le_bytes());
    v.extend_from_slice(&channels.to_le_bytes());
    v.extend_from_slice(&rate.to_le_bytes());
    v.extend_from_slice(&(rate * block_align as u32).to_le_bytes());
    v.extend_from_slice(&block_align.to_le_bytes());
    v.extend_from_slice(&bits.to_le_bytes());
    v.extend_from_slice(b"data");
    v.extend_from_slice(&data_len.to_le_bytes());
    v.extend_from_slice(frame_bytes);
    v
}

fn main() {
    let wav = wav_bytes(1, 16, 24000, &[0u8; 8]);
    let mut w = open(wav).unwrap();
    assert_eq!(w.info().rate, 24000);
    assert_eq!(w.info().channels, 1);

    let mp3 = include_bytes!("../../../capsule_vfs/src/testassets/boot_tone.mp3").to_vec();
    let mut m = open(mp3).unwrap();
    assert_eq!(m.info().rate, 44100);
    assert_eq!(m.info().channels, 1);
    let ch = m.info().channels as u64;
    let mut buf = [0i16; 1152];
    let mut frames = 0u64;
    loop {
        let n = m.next(&mut buf);
        if n == 0 {
            break;
        }
        frames += (n as u64) / ch;
    }
    assert!(frames > 0);

    assert!(open(vec![0u8, 1, 2, 3]).is_err());

    println!("HOSTTEST-PASS sniff");
}
