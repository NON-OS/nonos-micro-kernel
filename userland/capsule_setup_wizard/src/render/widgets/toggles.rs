use nonos_toolkit::font::render::draw_text;

use crate::render::paint::fill_rect;
use crate::render::theme::{ACCENT, FG, ROW_BG, ROW_BORDER, ROW_SEL_BG};

pub fn list(buf: &mut [u32], spx: usize, w: u32, h: u32, x: u32, y: u32, items: &[&[u8]], bits: u16, focus: usize) {
    let rw = 360u32;
    let mut yy = y;
    for (i, item) in items.iter().enumerate() {
        let bd = if i == focus { ACCENT } else { ROW_BORDER };
        fill_rect(buf, spx, w, h, x, yy, rw, 24, bd);
        fill_rect(buf, spx, w, h, x + 1, yy + 1, rw - 2, 22, ROW_BG);
        draw_text(buf, spx, w, h, x + 10, yy + 8, item, FG);
        let on = bits & (1 << i) != 0;
        let sx = x + rw - 34;
        fill_rect(buf, spx, w, h, sx, yy + 6, 26, 13, if on { ACCENT } else { ROW_SEL_BG });
        let knob = if on { sx + 15 } else { sx + 2 };
        fill_rect(buf, spx, w, h, knob, yy + 8, 9, 9, ROW_BG);
        yy += 30;
    }
}
