#![allow(dead_code)]

#[path = "../../src/pm"]
mod pm {
    #[path = "state"]
    pub mod state {
        #[path = "sort.rs"]
        mod sort;
        pub use sort::Sort;
    }

    #[path = "ui"]
    pub mod ui {
        #[path = "metrics.rs"]
        pub mod metrics;
        #[path = "table_geom.rs"]
        pub mod table_geom;
    }
}

use pm::ui::metrics::{NAME_MIN_W, TBL_HEAD_H};
use pm::ui::table_geom::{
    col_w, col_x, in_head, index_at, name_w, row_y, sort_at_x, visible_rows,
    COLS_FULL, COLS_OVERVIEW,
};

const OVERVIEW_W: u32 = 724;
const FULL_W: u32 = 1004;
const PANE_H: u32 = 620;

fn main() {
    let rows = visible_rows(PANE_H);
    assert!(rows > 1, "the pane fits {rows} rows");
    let total = rows + 40;

    for scroll in [0usize, 13] {
        for slot in 0..rows {
            let y = row_y(slot) as i32 + 1;
            assert!(!in_head(y), "row {slot} lands in the header band");
            assert!(
                index_at(y, scroll, total, PANE_H) == Some(scroll + slot),
                "row {slot} at scroll {scroll} does not round-trip"
            );
        }
    }

    assert!(in_head(TBL_HEAD_H as i32 - 1), "the header band ends early");
    assert!(index_at(TBL_HEAD_H as i32 - 1, 0, total, PANE_H) == None, "the header selects a row");
    assert!(index_at(row_y(rows) as i32 + 1, 0, total, PANE_H) == None, "a row past the pane");
    assert!(index_at(row_y(0) as i32 + 1, 0, 0, PANE_H) == None, "an empty table selects a row");

    for (cols, table_w) in [(&COLS_OVERVIEW[..], OVERVIEW_W), (&COLS_FULL[..], FULL_W)] {
        for pair in cols.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            assert!(
                col_x(cols, table_w, b) == col_x(cols, table_w, a) + col_w(cols, table_w, a),
                "columns do not tile at width {table_w}"
            );
        }
        let last = cols[cols.len() - 1];
        let right = col_x(cols, table_w, last) + col_w(cols, table_w, last);
        assert!(right <= table_w, "columns overflow {table_w} by {}", right - table_w);

        for c in cols.iter().copied() {
            let x = col_x(cols, table_w, c) as i32 + 1;
            assert!(sort_at_x(cols, table_w, x) == Some(c), "a header click misses its column");
        }
    }

    assert!(name_w(&COLS_FULL, 320) == NAME_MIN_W, "the name column sinks below its floor");
    assert!(name_w(&COLS_FULL, FULL_W) > NAME_MIN_W, "the name column never flexes");

    println!("[TABLE-GEOM] PASS");
}
