use crate::font::{atlas::FontAtlas, glyph::GlyphBitmap};

pub fn draw_glyph(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    glyph: &GlyphBitmap,
    color: u32,
) {
    let w = w as usize;
    let h = h as usize;
    for row in 0..glyph.height as usize {
        for col in 0..glyph.width as usize {
            if glyph.rows[row] & (0x80 >> col) == 0 {
                continue;
            }
            let px = x as usize + col;
            let py = y as usize + row;
            if px >= w || py >= h {
                continue;
            }
            let idx = py.saturating_mul(stride).saturating_add(px);
            if idx < buf.len() {
                buf[idx] = color;
            }
        }
    }
}

pub fn draw_text(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    text: &[u8],
    color: u32,
) {
    let atlas = FontAtlas::default();
    let mut pen_x = x;
    for &ch in text {
        let glyph = atlas.glyph(ch);
        draw_glyph(buf, stride, w, h, pen_x, y, glyph, color);
        pen_x = pen_x.saturating_add(atlas.glyph_width as u32 + atlas.letter_spacing as u32);
    }
}

pub fn draw_glyph_scaled(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    glyph: &GlyphBitmap,
    color: u32,
    scale: u32,
) {
    for row in 0..glyph.height as u32 {
        for col in 0..glyph.width as u32 {
            if glyph.rows[row as usize] & (0x80 >> col) == 0 {
                continue;
            }
            for dy in 0..scale {
                for dx in 0..scale {
                    let px = x + col * scale + dx;
                    let py = y + row * scale + dy;
                    if px >= w || py >= h {
                        continue;
                    }
                    let idx = (py as usize) * stride + px as usize;
                    if idx < buf.len() {
                        buf[idx] = color;
                    }
                }
            }
        }
    }
}

pub fn draw_text_scaled(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    text: &[u8],
    color: u32,
    scale: u32,
) {
    let atlas = FontAtlas::default();
    let mut pen_x = x;
    for &ch in text {
        let glyph = atlas.glyph(ch);
        draw_glyph_scaled(buf, stride, w, h, pen_x, y, glyph, color, scale);
        pen_x = pen_x.saturating_add((atlas.glyph_width as u32 + atlas.letter_spacing as u32) * scale);
    }
}
