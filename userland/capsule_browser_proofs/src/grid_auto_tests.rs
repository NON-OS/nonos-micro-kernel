// NONOS Operating System (AGPL-3.0-or-later)
//! repeat(auto-fill | auto-fit, ...), the form every responsive card grid is
//! written in. The count is not in the stylesheet, it comes from how many
//! floors fit across the container, so these pin the count and the widths.

use crate::grid_page::{card_boxes, card_page};
use crate::render::render;

const W: u32 = 1200;

// 1200 of width with a 20 gap fits three 300 floors, and the tracks then
// share the width evenly because the max side is a fraction.
#[test]
fn auto_fill_minmax_resolves_to_real_tracks() {
    let doc = render(&card_page("repeat(auto-fill, minmax(300px, 1fr))"), W);
    let cards = card_boxes(&doc);
    assert_eq!(cards.len(), 3);
    let want = (W as i32 - 40) / 3;
    for &(_, w) in &cards {
        assert_eq!(w, want, "auto-fill track width {w} != {want}");
    }
    assert_eq!(cards[1].0 - cards[0].0, want + 20);
}

// A floor only two tracks clear gives two columns, and the third item wraps.
#[test]
fn auto_fill_track_count_follows_the_floor() {
    let doc = render(&card_page("repeat(auto-fill, minmax(500px, 1fr))"), W);
    let cards = card_boxes(&doc);
    assert_eq!(cards.len(), 3, "three items still paint three cards");
    assert_eq!(cards[0].1, (W as i32 - 20) / 2, "two tracks share the width");
    assert_eq!(cards[2].0, cards[0].0, "the third item wraps under the first");
}

// auto-fit drops the tracks no item lands in, so two items fill the row that
// auto-fill would leave a third of empty.
#[test]
fn auto_fit_collapses_the_tracks_no_item_uses() {
    let two = "<html><head><style>body{margin:0}\
               .grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(300px,1fr));\
               gap:20px}.card{background:#202020}</style></head><body>\
               <div class=\"grid\"><div class=\"card\">a</div><div class=\"card\">b</div>\
               </div></body></html>";
    let cards = card_boxes(&render(two, W));
    assert_eq!(cards.len(), 2);
    assert_eq!(cards[0].1, (W as i32 - 20) / 2, "two items should share the whole row");
    assert_eq!(cards[1].0, (W as i32 - 20) / 2 + 20, "the second starts after the gap");
}

// A percentage floor resolves against the container before the count is
// worked out. A track costs its floor plus a gap, so 20% of 1200 fits four
// times and not five: five 240s plus four 20s is 1280, past the container.
#[test]
fn a_percentage_floor_resolves_against_the_container() {
    let wide = card_boxes(&render(&card_page("repeat(auto-fill, minmax(45%, 1fr))"), W));
    assert_eq!(wide[0].1, (W as i32 - 20) / 2, "45% of 1200 fits two tracks");
    let tight = card_boxes(&render(&card_page("repeat(auto-fill, minmax(20%, 1fr))"), W));
    assert_eq!(tight[0].1, (W as i32 - 60) / 4, "20% fits four");
}
