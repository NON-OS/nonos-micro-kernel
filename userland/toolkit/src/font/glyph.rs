#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlyphBitmap {
    pub width: u8,
    pub height: u8,
    pub rows: [u8; 8],
}

impl GlyphBitmap {
    pub const fn new(rows: [u8; 8]) -> Self {
        Self { width: 8, height: 8, rows }
    }
}

const GLYPH_UNKNOWN: GlyphBitmap =
    GlyphBitmap::new([0x7E, 0x81, 0xA5, 0x81, 0x99, 0x81, 0x7E, 0x00]);
const GLYPH_OSLASH: GlyphBitmap =
    GlyphBitmap::new([0x3C, 0x42, 0x46, 0x4A, 0x52, 0x62, 0x3C, 0x00]);
const GLYPH_CHEVRON: GlyphBitmap =
    GlyphBitmap::new([0x60, 0x30, 0x18, 0x0C, 0x18, 0x30, 0x60, 0x00]);
const GLYPH_SPACE: GlyphBitmap = GlyphBitmap::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
const GLYPH_0: GlyphBitmap = GlyphBitmap::new([0x3C, 0x42, 0x46, 0x4A, 0x52, 0x62, 0x3C, 0x00]);
const GLYPH_1: GlyphBitmap = GlyphBitmap::new([0x08, 0x18, 0x28, 0x08, 0x08, 0x08, 0x3E, 0x00]);
const GLYPH_2: GlyphBitmap = GlyphBitmap::new([0x3C, 0x42, 0x02, 0x0C, 0x30, 0x40, 0x7E, 0x00]);
const GLYPH_3: GlyphBitmap = GlyphBitmap::new([0x3C, 0x42, 0x02, 0x1C, 0x02, 0x42, 0x3C, 0x00]);
const GLYPH_4: GlyphBitmap = GlyphBitmap::new([0x0C, 0x14, 0x24, 0x44, 0x7E, 0x04, 0x04, 0x00]);
const GLYPH_5: GlyphBitmap = GlyphBitmap::new([0x7E, 0x40, 0x7C, 0x02, 0x02, 0x42, 0x3C, 0x00]);
const GLYPH_6: GlyphBitmap = GlyphBitmap::new([0x1C, 0x20, 0x40, 0x7C, 0x42, 0x42, 0x3C, 0x00]);
const GLYPH_7: GlyphBitmap = GlyphBitmap::new([0x7E, 0x42, 0x04, 0x08, 0x10, 0x10, 0x10, 0x00]);
const GLYPH_8: GlyphBitmap = GlyphBitmap::new([0x3C, 0x42, 0x42, 0x3C, 0x42, 0x42, 0x3C, 0x00]);
const GLYPH_9: GlyphBitmap = GlyphBitmap::new([0x3C, 0x42, 0x42, 0x3E, 0x02, 0x04, 0x38, 0x00]);

pub fn digit_glyph(ascii: u8) -> Option<&'static GlyphBitmap> {
    match ascii {
        b'0' => Some(&GLYPH_0),
        b'1' => Some(&GLYPH_1),
        b'2' => Some(&GLYPH_2),
        b'3' => Some(&GLYPH_3),
        b'4' => Some(&GLYPH_4),
        b'5' => Some(&GLYPH_5),
        b'6' => Some(&GLYPH_6),
        b'7' => Some(&GLYPH_7),
        b'8' => Some(&GLYPH_8),
        b'9' => Some(&GLYPH_9),
        _ => None,
    }
}

pub fn glyph_for_ascii(ascii: u8) -> &'static GlyphBitmap {
    if ascii == b' ' {
        return &GLYPH_SPACE;
    }
    if ascii == 0xD8 {
        return &GLYPH_OSLASH;
    }
    if ascii == 0x10 {
        return &GLYPH_CHEVRON;
    }
    if let Some(g) = digit_glyph(ascii) {
        return g;
    }
    if ascii.is_ascii_uppercase() {
        return super::upper::glyph(ascii);
    }
    if ascii.is_ascii_lowercase() {
        return super::lower::glyph(ascii);
    }
    if let Some(g) = super::punct::glyph(ascii) {
        return g;
    }
    &GLYPH_UNKNOWN
}
