// NONOS Operating System (AGPL-3.0-or-later)
//! Card-grid geometry for an explicit track list. The reported defect is text
//! laid out to one width and painted into another, so these check the widths
//! a grid hands its children against the widths the children paint at.

use crate::grid_page::{card_boxes, card_page, CARD_BG};
use crate::render::{render, texts};

const W: u32 = 1200;

// Three equal fr tracks split the content width, minus the two gaps.
#[test]
fn three_fr_tracks_split_the_container_width() {
    let doc = render(&card_page("1fr 1fr 1fr"), W);
    let cards = card_boxes(&doc);
    assert_eq!(cards.len(), 3, "expected three card boxes, got {}", cards.len());
    let want = (W as i32 - 40) / 3;
    for &(_, w) in &cards {
        assert_eq!(w, want, "card width {w} != track width {want}");
    }
    assert_eq!(cards[1].0 - cards[0].0, want + 20);
    assert_eq!(cards[2].0 - cards[1].0, want + 20);
}

// No text may paint outside the card that contains it. A word tail hanging
// past the card edge is the visible defect.
#[test]
fn card_text_stays_inside_its_card() {
    let doc = render(&card_page("1fr 1fr 1fr"), W);
    let cards: alloc::vec::Vec<(i32, i32)> =
        doc.frags.iter().filter(|f| f.bg == CARD_BG && f.w > 0).map(|f| (f.x, f.x + f.w)).collect();
    for (x, _y, w, t) in texts(&doc) {
        let Some(&(cx0, cx1)) = cards.iter().find(|&&(a, b)| x >= a && x < b) else {
            panic!("text {t:?} at x={x} is outside every card");
        };
        assert!(x + w <= cx1, "text {t:?} runs to {} past card edge {cx1}", x + w);
        assert!(x >= cx0, "text {t:?} starts at {x} before card edge {cx0}");
    }
}

// A fixed track keeps its size and the fraction takes what is left, so a
// sidebar layout does not hand its main column the whole width.
#[test]
fn a_fixed_track_beside_a_fraction_keeps_its_size() {
    let doc = render(&card_page("240px 1fr"), W);
    let cards = card_boxes(&doc);
    assert_eq!(cards[0].1, 240, "the fixed track should stay at 240");
    assert_eq!(cards[1].1, W as i32 - 240 - 20, "the fraction takes the rest");
}
