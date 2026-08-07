// NONOS Operating System (AGPL-3.0-or-later)
//! Grid items are laid out at the container top and dropped to their row
//! afterwards. Anything resolved against the pre-drop position has to move
//! with them, or a card paints its background in row two and clips its text
//! against row one, which reads as a black box.

use crate::grid_page::CARD_BG;
use crate::render::{render, text_of};

const W: u32 = 1200;

// Two columns and four cards, so the second row is genuinely shifted down.
// The cards clip their own overflow, which is what a card with a fixed
// height and a rounded corner does on a real page.
fn clipped_cards() -> alloc::string::String {
    alloc::format!(
        "<html><head><style>body{{margin:0}}\
         .grid{{display:grid;grid-template-columns:1fr 1fr;gap:20px}}\
         .card{{background:#202020;overflow:hidden;height:80px}}\
         </style></head><body><div class=\"grid\">\
         <div class=\"card\"><p>one</p></div><div class=\"card\"><p>two</p></div>\
         <div class=\"card\"><p>three</p></div><div class=\"card\"><p>four</p></div>\
         </div></body></html>"
    )
}

// Every card's text must sit inside the clip its own card established. A
// second-row card whose clip still names the first row hides its own text.
#[test]
fn a_shifted_card_clips_against_its_own_row() {
    let doc = render(&clipped_cards(), W);
    let mut seen = 0;
    for f in &doc.frags {
        let Some(t) = text_of(f) else { continue };
        let Some(clip) = f.clip else { continue };
        seen += 1;
        assert!(
            f.y >= clip[1] && f.y + f.h <= clip[3],
            "text {t:?} at y={}..{} is outside its clip {}..{}",
            f.y,
            f.y + f.h,
            clip[1],
            clip[3]
        );
    }
    assert!(seen >= 4, "expected a clip on each card's text, saw {seen}");
}

// The cards themselves must land on two distinct rows, otherwise the test
// above is not exercising a shift at all.
#[test]
fn the_clipped_cards_really_occupy_two_rows() {
    let doc = render(&clipped_cards(), W);
    let mut rows: alloc::vec::Vec<i32> =
        doc.frags.iter().filter(|f| f.bg == CARD_BG && text_of(f).is_none()).map(|f| f.y).collect();
    rows.sort_unstable();
    rows.dedup();
    assert_eq!(rows.len(), 2, "expected two card rows, got {rows:?}");
    assert!(rows[1] > 0, "the second row should be below the first");
}
