#![allow(dead_code)]

#[path = "../../src/pm"]
mod pm {
    #[path = "security"]
    pub mod security {
        #[path = "sensitive.rs"]
        pub mod sensitive;
    }

    #[path = "ui"]
    pub mod ui {
        #[path = "matrix_geom.rs"]
        pub mod matrix_geom;
    }
}

use pm::ui::matrix_geom::{
    cell_at, cell_w, cell_x, row_at, row_y, visible_rows, HEAD_H, LEGEND_H, MATRIX, NAME_W, PAD_X,
    ROW_H,
};

const PANE_W: u32 = 1004;
const PANE_H: u32 = 672;

fn main() {
    let rows = visible_rows(PANE_H);
    assert!(rows > 0, "the authority pane fits no rows");
    let total = rows + 40;

    for scroll in [0usize, 7] {
        for slot in 0..rows {
            let y = row_y(slot) as i32 + (ROW_H / 2) as i32;
            assert!(
                row_at(y, PANE_H, scroll, total) == Some(scroll + slot),
                "row {slot} at scroll {scroll} does not round-trip"
            );
            for col in 0..MATRIX.len() {
                let x = cell_x(PANE_W, col) as i32 + (cell_w(PANE_W) / 2) as i32;
                assert!(
                    cell_at(PANE_W, x, y, PANE_H, scroll, total) == Some((scroll + slot, col)),
                    "cell ({slot}, {col}) at scroll {scroll} does not round-trip"
                );
            }
        }
    }

    let body = row_y(0) as i32 + (ROW_H / 2) as i32;
    assert!(row_at(HEAD_H as i32 - 1, PANE_H, 0, total) == None, "the header resolves as a row");
    assert!(row_at(row_y(rows) as i32 + 1, PANE_H, 0, total) == None, "a row past the pane resolves");
    assert!(row_at(body, PANE_H, 0, 0) == None, "an empty list resolves a row");
    assert!(cell_at(PANE_W, PAD_X as i32 + 1, body, PANE_H, 0, total) == None, "the name column is a cell");

    let right = cell_x(PANE_W, MATRIX.len() - 1) + cell_w(PANE_W);
    assert!(right <= PANE_W - PAD_X, "the grid overflows the pane: {right} > {}", PANE_W - PAD_X);
    assert!(cell_x(PANE_W, 0) >= PAD_X + NAME_W, "the first column overlaps the name column");
    assert!(row_y(rows) + LEGEND_H <= PANE_H, "the legend does not fit under the last row");

    println!("[MATRIX-GEOM] PASS");
}
