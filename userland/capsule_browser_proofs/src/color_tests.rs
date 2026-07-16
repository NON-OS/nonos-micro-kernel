// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for CSS color parsing: hex (3/4/6/8 digit), rgb()/rgba(),
//! and named colors, all resolved to 0xAARRGGBB. Colors are on every page, so a
//! wrong shorthand expansion or channel order mis-renders the whole web.

use crate::browser::css::color::parse_color;

fn rgb(sel: &str) -> u32 {
    parse_color(sel).unwrap_or_else(|| panic!("parse {sel:?}")) & 0x00FF_FFFF
}

#[test]
fn hex_shorthand_expands_each_nibble() {
    assert_eq!(rgb("#fff"), 0xFFFFFF, "white");
    assert_eq!(rgb("#000"), 0x000000, "black");
    assert_eq!(rgb("#f00"), 0xFF0000, "red");
    assert_eq!(rgb("#0f0"), 0x00FF00, "green");
    assert_eq!(rgb("#00f"), 0x0000FF, "blue");
    assert_eq!(rgb("#abc"), 0xAABBCC, "each nibble doubled");
}

#[test]
fn hex_full_and_alpha_forms() {
    assert_eq!(rgb("#ffffff"), 0xFFFFFF);
    assert_eq!(rgb("#123456"), 0x123456, "channel order r,g,b");
    assert_eq!(rgb("#12345678"), 0x123456, "8-digit drops the trailing alpha");
    assert_eq!(rgb("#f00f"), 0xFF0000, "4-digit drops the trailing alpha nibble");
    assert!(parse_color("#12").is_none(), "an invalid length is rejected");
    assert!(parse_color("#gggggg").is_none(), "non-hex is rejected");
}

#[test]
fn rgb_and_rgba_functions() {
    assert_eq!(rgb("rgb(255,0,0)"), 0xFF0000);
    assert_eq!(rgb("rgb(0, 128, 255)"), 0x0080FF, "spaces and channel order");
    assert_eq!(rgb("rgba(0,255,0,0.5)"), 0x00FF00, "rgba keeps the color channels");
}

#[test]
fn named_colors_resolve() {
    assert_eq!(rgb("red"), 0xFF0000);
    assert_eq!(rgb("white"), 0xFFFFFF);
    assert_eq!(rgb("black"), 0x000000);
    assert!(parse_color("notacolour").is_none(), "an unknown name is rejected");
}
