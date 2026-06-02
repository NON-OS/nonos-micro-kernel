use nonos_toolkit::font::render::draw_text;

use crate::render::paint::fill_rect;
use crate::render::theme::{ACCENT, FG, ROW_BG, ROW_BORDER};

pub fn masked(buf: &mut [u32], spx: usize, w: u32, h: u32, x: u32, y: u32, len: usize, strength: u32) {
    let fw = 360u32;
    fill_rect(buf, spx, w, h, x, y, fw, 26, ACCENT);
    fill_rect(buf, spx, w, h, x + 1, y + 1, fw - 2, 24, ROW_BG);
    let dots = [b'*'; 32];
    let n = len.min(32);
    draw_text(buf, spx, w, h, x + 8, y + 9, &dots[..n], FG);
    fill_rect(buf, spx, w, h, x, y + 34, fw, 6, ROW_BORDER);
    let fillw = fw * strength.min(100) / 100;
    fill_rect(buf, spx, w, h, x, y + 34, fillw, 6, ACCENT);
}

pub fn strength_of(len: usize) -> u32 {
    (len as u32 * 12).min(100)
}
