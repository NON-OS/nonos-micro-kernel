use nonos_toolkit::font::render::draw_text;

use crate::render::paint::fill_rect;
use crate::render::theme::{ACCENT, FG, ROW_BG, ROW_BORDER, ROW_SEL_BG};

pub fn list(
    buf: &mut [u32],
    spx: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    items: &[&[u8]],
    sel: usize,
) {
    let rw = 360u32;
    let mut yy = y;
    for (i, item) in items.iter().enumerate() {
        let bd = if i == sel { ACCENT } else { ROW_BORDER };
        let bg = if i == sel { ROW_SEL_BG } else { ROW_BG };
        fill_rect(buf, spx, w, h, x, yy, rw, 24, bd);
        fill_rect(buf, spx, w, h, x + 1, yy + 1, rw - 2, 22, bg);
        let dot: &[u8] = if i == sel { b">" } else { b"." };
        draw_text(buf, spx, w, h, x + 8, yy + 8, dot, ACCENT);
        draw_text(buf, spx, w, h, x + 24, yy + 8, item, FG);
        if i == sel {
            draw_text(buf, spx, w, h, x + rw - 16, yy + 8, b"+", ACCENT);
        }
        yy += 30;
    }
}
