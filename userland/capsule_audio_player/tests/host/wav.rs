extern crate alloc;
#[path = "."]
mod decode {
    #[path = "../../src/decode/decoder.rs"]
    pub mod decoder;
    #[path = "../../src/decode/wav.rs"]
    pub mod wav;
    #[path = "../../src/decode/wav_pcm.rs"]
    pub mod wav_pcm;
}
use decode::decoder::Decoder;
use decode::wav::WavDecoder;

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
    let mut pcm = Vec::new();
    for i in 0..16i16 {
        pcm.extend_from_slice(&i.to_le_bytes());
    }
    let bytes = wav_bytes(2, 16, 48000, &pcm);
    let mut dec = WavDecoder::new(bytes).unwrap();
    let info = dec.info();
    assert_eq!(info.rate, 48000);
    assert_eq!(info.channels, 2);
    assert_eq!(info.total_frames, Some(8));
    let mut out = [0i16; 16];
    assert_eq!(dec.next(&mut out), 16);
    assert_eq!(out[0], 0);
    assert_eq!(out[15], 15);
    let mut tail = [0i16; 4];
    assert_eq!(dec.next(&mut tail), 0);

    let mono8: Vec<u8> = (0..8u8).collect();
    let bytes8 = wav_bytes(1, 8, 22050, &mono8);
    let mut dec8 = WavDecoder::new(bytes8).unwrap();
    assert_eq!(dec8.info().channels, 1);
    assert_eq!(dec8.info().total_frames, Some(8));
    let mut out8 = [0i16; 8];
    assert_eq!(dec8.next(&mut out8), 8);
    assert_eq!(out8[0], -128 << 8);

    let sample24 = [0x00u8, 0x01, 0x02];
    assert_eq!(
        decode::wav_pcm::decode_sample(&sample24, 24),
        i16::from_le_bytes([0x01, 0x02])
    );

    let bad_bits = wav_bytes(1, 4, 8000, &[0u8; 4]);
    assert!(WavDecoder::new(bad_bits).is_err());

    println!("HOSTTEST-PASS wav");
}
