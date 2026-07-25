#[path = "../../src/ui/geometry.rs"]
mod geometry;
pub use geometry::Layout;

#[path = "../../src/ui/control.rs"]
mod control;

use control::{control_at, Control};
use geometry::{layout, Rect};

fn center(r: &Rect) -> (i32, i32) {
    ((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32)
}

fn main() {
    let l = layout(480, 320);

    let (px, py) = center(&l.play);
    assert_eq!(control_at(&l, px, py), Some(Control::PlayPause), "play");
    let (sx, sy) = center(&l.stop);
    assert_eq!(control_at(&l, sx, sy), Some(Control::Stop), "stop");
    let (bx, by) = center(&l.back);
    assert_eq!(control_at(&l, bx, by), Some(Control::SeekBackSecs(10)), "back");
    let (fx, fy) = center(&l.fwd);
    assert_eq!(control_at(&l, fx, fy), Some(Control::SeekFwdSecs(10)), "fwd");

    let py2 = (l.progress.y + l.progress.h / 2) as i32;
    match control_at(&l, l.progress.x as i32, py2) {
        Some(Control::SeekPermille(p)) => assert!(p <= 3, "progress far-left {}", p),
        other => panic!("progress far-left {:?}", other),
    }
    match control_at(&l, (l.progress.x + l.progress.w - 1) as i32, py2) {
        Some(Control::SeekPermille(p)) => assert!(p >= 997, "progress far-right {}", p),
        other => panic!("progress far-right {:?}", other),
    }
    match control_at(&l, (l.progress.x + l.progress.w / 2) as i32, py2) {
        Some(Control::SeekPermille(p)) => assert!((495..=505).contains(&p), "progress mid {}", p),
        other => panic!("progress mid {:?}", other),
    }

    let vy = (l.volume.y + l.volume.h / 2) as i32;
    match control_at(&l, (l.volume.x + l.volume.w / 2) as i32, vy) {
        Some(Control::VolumePermille(p)) => assert!((495..=505).contains(&p), "volume mid {}", p),
        other => panic!("volume mid {:?}", other),
    }

    assert_eq!(control_at(&l, 0, 0), None, "dead space");

    println!("HOSTTEST-PASS event");
}
