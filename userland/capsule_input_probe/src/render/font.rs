use super::font_table;

pub const GLYPH_W: u32 = 8;
pub const GLYPH_H: u32 = 8;

pub fn rows(c: u8) -> [u8; 8] {
    let c = c.to_ascii_uppercase();
    match c {
        b'A'..=b'Z' => font_table::letter(c),
        b'0'..=b'9' => font_table::digit(c),
        b' ' => [0x00; 8],
        _ => font_table::FALLBACK,
    }
}
