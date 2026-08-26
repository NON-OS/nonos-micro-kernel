use nonos_avi::AviFile;

const CLIP: &[u8] = include_bytes!("../../capsule_video_player/assets/clip.avi");

#[test]
fn parses_the_authored_clip() {
    let f = AviFile::parse(CLIP).expect("fixture must parse");
    assert_eq!((f.video.width, f.video.height), (160, 120));
    assert_eq!(f.index.len(), 12);
}

#[test]
fn reports_ten_frames_per_second() {
    let f = AviFile::parse(CLIP).unwrap();
    assert_eq!(f.fps_milli(), 10_000);
}

#[test]
fn every_index_entry_lands_on_a_jpeg_soi() {
    let f = AviFile::parse(CLIP).unwrap();
    for (i, fr) in f.index.iter().enumerate() {
        let start = fr.offset as usize;
        let end = start + fr.len as usize;
        assert!(end <= CLIP.len(), "frame {i} runs past EOF");
        assert!(fr.len > 2, "frame {i} is too short to be a JPEG");
        assert_eq!(&CLIP[start..start + 2], &[0xFF, 0xD8], "frame {i} is not SOI");
        assert_eq!(&CLIP[end - 2..end], &[0xFF, 0xD9], "frame {i} is not EOI");
    }
}

#[test]
fn frames_do_not_overlap_and_advance() {
    let f = AviFile::parse(CLIP).unwrap();
    let mut prev_end = 0u64;
    for fr in f.index.iter() {
        assert!(fr.offset >= prev_end, "index is not monotonic");
        prev_end = fr.offset + fr.len as u64;
    }
}
