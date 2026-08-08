// NONOS Operating System (AGPL-3.0-or-later)
//! The card-grid fixture the grid proofs render, and the two readings they
//! take from it. A card carries padding and a background so a defect shows up
//! as a box in the wrong place rather than as text alone.

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::layout::boxmodel::{BoxDocument, Content};

pub const CARD_BG: u32 = 0xff202020;

pub fn card_page(template: &str) -> String {
    alloc::format!(
        "<html><head><style>\
         body {{ margin: 0; }}\
         .grid {{ display: grid; grid-template-columns: {template}; gap: 20px; }}\
         .card {{ background: #202020; padding: 16px; }}\
         </style></head><body><div class=\"grid\">\
         <div class=\"card\"><p>alpha beta gamma delta epsilon</p></div>\
         <div class=\"card\"><p>zeta eta theta iota kappa</p></div>\
         <div class=\"card\"><p>lambda mu nu xi omicron</p></div>\
         </div></body></html>"
    )
}

// Each card's (x, width), in paint order. A text run inside a card carries
// the card's own background, so the box fragments are the ones with no
// content of their own.
pub fn card_boxes(doc: &BoxDocument) -> Vec<(i32, i32)> {
    doc.frags
        .iter()
        .filter(|f| f.bg == CARD_BG && f.w > 0 && matches!(f.content, Content::None))
        .map(|f| (f.x, f.w))
        .collect()
}
