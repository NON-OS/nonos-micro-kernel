use nonos_toolkit::font::render::draw_text;

use crate::render::paint::fill_rect;
use crate::render::theme::{ACCENT, DOT_DONE, HINT, ROW_BORDER};

pub fn busy(buf: &mut [u32], spx: usize, w: u32, h: u32, x: u32, y: u32, tasks: &[(&[u8], bool)], pct: u32) {
    let mut yy = y;
    for (label, done) in tasks {
        let (mark, color): (&[u8], u32) = if *done { (b"+", DOT_DONE) } else { (b".", HINT) };
        draw_text(buf, spx, w, h, x, yy, mark, color);
        draw_text(buf, spx, w, h, x + 16, yy, label, color);
        yy += 20;
    }
    fill_rect(buf, spx, w, h, x, yy + 8, 360, 6, ROW_BORDER);
    fill_rect(buf, spx, w, h, x, yy + 8, 360 * pct.min(100) / 100, 6, ACCENT);
}
