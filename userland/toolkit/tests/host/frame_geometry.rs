#[path = "../../src/decorations/metrics.rs"]
mod metrics;
#[path = "../../src/decorations/rect.rs"]
mod rect;
#[path = "../../src/decorations/frame_rect.rs"]
mod frame_rect;

use frame_rect::*;
use metrics::*;

fn main() {
    let (w, h) = (560u32, 360u32);

    let f = frame_rect(w, h, false);
    assert_eq!((f.x, f.y, f.w, f.h), (10, 10, 540, 340), "frame_rect");

    let c = content_rect(w, h, false);
    assert_eq!((c.x, c.y, c.w, c.h), (11, 50, 538, 299), "content_rect");

    let t = titlebar_rect(w, h, false);
    assert_eq!((t.x, t.y, t.w, t.h), (10, 10, 540, 40), "titlebar_rect");

    for (i, ex) in [26u32, 46, 66].iter().enumerate() {
        let l = light_rect(i as u32, w, h, false);
        assert_eq!((l.x, l.y, l.w, l.h), (*ex, 24, 12, 12), "light {}", i);
    }

    let fm = frame_rect(w, h, true);
    assert_eq!((fm.x, fm.y, fm.w, fm.h), (0, 0, w, h), "maximized frame");
    assert_eq!(margin(true), 0);
    assert_eq!(radius(true), 0);
    let cm = content_rect(w, h, true);
    assert_eq!((cm.x, cm.y, cm.w, cm.h), (1, 40, w - 2, h - 41), "maximized content");

    for (tw, th) in [(0u32, 0u32), (1, 1), (20, 20), (21, 41), (2 * SHADOW_MARGIN, 2 * SHADOW_MARGIN)] {
        let c = content_rect(tw, th, false);
        assert!(c.x + c.w <= tw.max(c.x), "content escapes surface at {}x{}", tw, th);
        let f = frame_rect(tw, th, false);
        assert!(f.w <= tw && f.h <= th, "frame escapes surface at {}x{}", tw, th);
    }

    let inside = frame_rect(w, h, false);
    assert!(!inside.contains(9, 9), "shadow margin must not hit the frame");
    assert!(inside.contains(10, 10), "frame origin must hit");
    assert!(!inside.contains(550, 350), "far margin must not hit");

    let l0 = light_rect(0, w, h, false).inflate(LIGHT_HIT_PAD);
    let l1 = light_rect(1, w, h, false).inflate(LIGHT_HIT_PAD);
    assert!(l0.x + l0.w <= l1.x, "inflated light targets overlap");

    assert!(TITLEBAR_H >= TITLE_PX as u32 + LIGHT_D, "titlebar too short for its contents");

    println!("[FRAME-GEOM] PASS");
}
