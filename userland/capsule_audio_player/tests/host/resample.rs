extern crate alloc;
#[path = "../../src/resample.rs"]
mod resample;
use resample::{Resampler, OUT_RATE};

fn main() {
    let mut mono = Vec::new();
    for i in 0..100i16 {
        mono.push(i * 100);
    }
    let mut rs = Resampler::new(24_000, 1);
    let mut out = Vec::new();
    rs.process(&mono, &mut out);
    let expect_frames = mono.len() * 2;
    let got_frames = out.len() / 2;
    assert!(
        (got_frames as isize - expect_frames as isize).abs() <= 1,
        "got {} expected ~{}",
        got_frames,
        expect_frames
    );
    for f in out.chunks(2) {
        assert_eq!(f[0], f[1], "mono dup must give L==R");
    }

    let mut stereo = Vec::new();
    for i in 0..64i16 {
        stereo.push(i);
        stereo.push(-i);
    }
    let mut rs2 = Resampler::new(OUT_RATE, 2);
    let mut out2 = Vec::new();
    rs2.process(&stereo, &mut out2);
    assert_eq!(out2, stereo, "48k stereo passthrough must be bit-exact");

    let mut rs3 = Resampler::new(OUT_RATE, 2);
    let mut out3 = Vec::new();
    let (a, b) = stereo.split_at(stereo.len() / 2);
    rs3.process(a, &mut out3);
    rs3.process(b, &mut out3);
    assert_eq!(out3, stereo, "chunked 48k stereo passthrough must stay bit-exact");

    let mut ramp = Vec::new();
    for i in 0..8i16 {
        ramp.push(i * 1000);
    }
    let mut whole = Vec::new();
    Resampler::new(24_000, 1).process(&ramp, &mut whole);
    let mut seam = Vec::new();
    let mut rs5 = Resampler::new(24_000, 1);
    let (h1, h2) = ramp.split_at(4);
    rs5.process(h1, &mut seam);
    rs5.process(h2, &mut seam);
    assert_eq!(seam, whole, "chunked upsample must match single call (gapless seam)");

    println!("HOSTTEST-PASS resample");
}
