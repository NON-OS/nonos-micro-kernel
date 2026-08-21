#[path = "../../src/settings/ui/metrics.rs"]
mod metrics;
#[path = "../../src/settings/ui/results_geom.rs"]
mod results_geom;

use metrics::*;
use results_geom::*;

fn main() {
    assert_eq!(card_y(), PANE_PAD_TOP + HEAD_H, "card follows the page header");
    assert_eq!(row_y(0), card_y() + CARD_HEAD_H, "first result follows the card header");
    assert_eq!(card_h(0), card_h(1), "an empty result set still draws a card body");

    for n in 1..40usize {
        assert_eq!(card_h(n), CARD_HEAD_H + ROW_H * n as u32, "card_h({})", n);
        assert_eq!(row_y(n - 1) + ROW_H, card_y() + card_h(n), "last row must end the card");
        for scroll in [0u32, 7, ROW_H, 500] {
            for i in 0..n {
                let top = row_y(i) as i32 - scroll as i32;
                assert_eq!(index_at(top, scroll, n), Some(i), "top of row {} at scroll {}", i, scroll);
                let bottom = top + ROW_H as i32 - 1;
                assert_eq!(index_at(bottom, scroll, n), Some(i), "bottom of row {}", i);
            }
            let above = row_y(0) as i32 - scroll as i32 - 1;
            assert_eq!(index_at(above, scroll, n), None, "click above the first row");
            let below = row_y(n - 1) as i32 - scroll as i32 + ROW_H as i32;
            assert_eq!(index_at(below, scroll, n), None, "click below the last row");
        }
    }

    assert_eq!(max_scroll(1, u32::MAX), 0, "a tall view never scrolls");
    assert!(max_scroll(40, 400) > 0, "forty results must scroll in a 400px view");
    assert_eq!(content_h(0), card_y() + card_h(0) + CARD_GAP, "content_h");

    println!("[RESULTS-GEOM] PASS");
}
