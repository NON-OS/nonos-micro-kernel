use nonos_avi::{AviError, AviFile};

const CLIP: &[u8] = include_bytes!("../../capsule_video_player/assets/clip.avi");

#[test]
fn rejects_a_non_riff_file() {
    let mut bad = CLIP.to_vec();
    bad[0] = b'X';
    assert!(matches!(AviFile::parse(&bad), Err(AviError::NotRiff)));
}

#[test]
fn rejects_a_riff_that_is_not_avi() {
    let mut bad = CLIP.to_vec();
    bad[8] = b'W';
    assert!(matches!(AviFile::parse(&bad), Err(AviError::NotAvi)));
}

#[test]
fn rejects_an_empty_and_a_stub_file() {
    assert!(AviFile::parse(&[]).is_err());
    assert!(AviFile::parse(b"RIFF").is_err());
    assert!(AviFile::parse(b"RIFF\0\0\0\0AVI ").is_err());
}

#[test]
fn no_panic_on_any_truncation() {
    for n in 0..CLIP.len() {
        let _ = AviFile::parse(&CLIP[..n]);
    }
}

#[test]
fn no_panic_when_a_single_byte_is_corrupted() {
    for i in (0..CLIP.len()).step_by(7) {
        let mut bad = CLIP.to_vec();
        bad[i] ^= 0xFF;
        let _ = AviFile::parse(&bad);
    }
}

#[test]
fn a_declared_frame_length_past_eof_is_still_bounded() {
    let f = AviFile::parse(CLIP).unwrap();
    for fr in f.index.iter() {
        assert!(fr.offset.checked_add(fr.len as u64).unwrap() <= CLIP.len() as u64);
    }
}
