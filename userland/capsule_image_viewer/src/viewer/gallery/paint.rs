extern crate alloc;
use crate::viewer::gallery::layout::{cell_rect, grid, Grid, HEADER_H, THUMB_H, THUMB_W};
use crate::viewer::gallery::state::{GalleryState, MAX_TILES};
use crate::viewer::scale::{draw_nn, Dst};
use nonos_app_skeleton::PaintBuffer;

const BG: u32 = 0xFF10_1418;
const FG: u32 = 0xFFE6_E6E6;
const PLACEHOLDER: u32 = 0xFF23_282E;
const ERRTILE: u32 = 0xFF3A_1E1E;
const SEL: u32 = 0xFF4A_9EFF;

pub fn paint_gallery(g: &mut GalleryState, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    fb.fill_rect(0, 0, w, h, BG);
    let gr = grid(w);
    let n = g.entries.len().min(MAX_TILES);
    if n == 0 {
        fb.text(w / 2 - 60, h / 2, b"No images found", FG);
        return;
    }
    paint_boxes(fb, g, &gr, n);
    blit_thumbs(fb, g, &gr, n);
    fb.text(10, 8, b"Gallery", FG);
}

fn paint_boxes(fb: &mut PaintBuffer, g: &GalleryState, gr: &Grid, n: usize) {
    let h = fb.height;
    for i in 0..n {
        let (x, y, _cw, ch) = cell_rect(i, g.scroll, gr);
        if y + ch as i32 <= HEADER_H as i32 || y >= h as i32 {
            continue;
        }
        let (ux, uy) = (x.max(0) as u32, y.max(0) as u32);
        let e = &g.entries[i];
        let base = if e.failed {
            ERRTILE
        } else if e.thumb.is_none() {
            PLACEHOLDER
        } else {
            BG
        };
        fb.fill_rect(ux, uy, THUMB_W, THUMB_H, base);
        if i == g.sel {
            fb.fill_rect(ux, uy, THUMB_W, 2, SEL);
            fb.fill_rect(ux, uy + THUMB_H - 2, THUMB_W, 2, SEL);
        }
        fb.text(ux, uy + THUMB_H, label(&e.path), FG);
    }
}

fn blit_thumbs(fb: &mut PaintBuffer, g: &GalleryState, gr: &Grid, n: usize) {
    let (w, h, stride) = (fb.width, fb.height, fb.stride_words);
    let mut dst = Dst { px: fb.pixels, stride, w, h };
    for i in 0..n {
        let (x, y, _cw, ch) = cell_rect(i, g.scroll, gr);
        if y + ch as i32 <= HEADER_H as i32 || y >= h as i32 {
            continue;
        }
        if let Some(thumb) = g.entries[i].thumb.as_ref() {
            let (tw, th) = (g.entries[i].tw, g.entries[i].th);
            let ox = x + ((THUMB_W - tw) / 2) as i32;
            let oy = y + ((THUMB_H - th) / 2) as i32;
            draw_nn(&mut dst, thumb, tw, th, ox, oy, tw, th);
        }
    }
}

fn label(path: &str) -> &[u8] {
    let b = path.as_bytes();
    let start = match b.iter().rposition(|&c| c == b'/') {
        Some(i) => i + 1,
        None => 0,
    };
    let end = (start + 18).min(b.len());
    &b[start..end]
}
