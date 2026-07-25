extern crate alloc;
#[path = "../../src/ui/geometry.rs"]
mod geometry;
use geometry::{layout, Layout, Rect};

fn in_bounds(r: &Rect, w: u32, h: u32) -> bool {
    r.x + r.w <= w && r.y + r.h <= h
}

fn overlap(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h
}

fn main() {
    let w = 480u32;
    let h = 320u32;
    let l: Layout = layout(w, h);

    let rects = [
        &l.back, &l.play, &l.stop, &l.fwd, &l.progress, &l.volume, &l.waveform,
    ];
    for r in rects.iter() {
        assert!(in_bounds(r, w, h), "rect out of bounds");
    }

    let btns = [&l.back, &l.play, &l.stop, &l.fwd];
    for i in 0..btns.len() {
        for j in (i + 1)..btns.len() {
            assert!(!overlap(btns[i], btns[j]), "buttons overlap");
        }
    }

    let cx = (l.progress.x + l.progress.w / 2) as i32;
    let cy = (l.progress.y + l.progress.h / 2) as i32;
    assert!(l.progress.contains(cx, cy), "center not contained");
    assert!(!l.progress.contains(-1, cy), "left-outside contained");
    assert!(
        !l.progress.contains(cx, (l.progress.y + l.progress.h) as i32),
        "below-outside contained"
    );

    println!("HOSTTEST-PASS layout");
}
