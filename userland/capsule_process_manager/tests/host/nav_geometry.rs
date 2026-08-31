#![allow(dead_code)]

#[path = "../../src/pm"]
mod pm {
    #[path = "state"]
    pub mod state {
        #[path = "screen.rs"]
        mod screen;
        pub use screen::{Screen, SCREENS};
    }

    #[path = "ui"]
    pub mod ui {
        #[path = "metrics.rs"]
        pub mod metrics;
        #[path = "nav_geom.rs"]
        pub mod nav_geom;
    }
}

use pm::state::SCREENS;
use pm::ui::metrics::{NAV_GAP, NAV_H, NAV_TOP, SIDEBAR_W};
use pm::ui::nav_geom::{at, row_x, row_y, row_w};

fn main() {
    for (i, screen) in SCREENS.iter().enumerate() {
        let x = row_x() as i32 + 1;
        let top = row_y(i) as i32;
        assert!(at(x, top + 1) == Some(*screen), "row {i} top edge");
        assert!(at(x, top + NAV_H as i32 - 1) == Some(*screen), "row {i} bottom edge");
    }

    assert!(at(row_x() as i32 + 1, NAV_TOP as i32 - 1) == None, "above the first row");

    if NAV_GAP > 0 {
        let gap_y = row_y(0) as i32 + NAV_H as i32 + (NAV_GAP as i32 / 2);
        assert!(at(row_x() as i32 + 1, gap_y) == None, "inter-row gap");
    }

    assert!(at(SIDEBAR_W as i32, row_y(0) as i32 + 1) == None, "right of the sidebar");
    assert!(at(-1, row_y(0) as i32 + 1) == None, "left of the sidebar");

    let past = row_y(SCREENS.len()) as i32 + 1;
    assert!(at(row_x() as i32 + 1, past) == None, "below the last row");

    assert!(row_x() + row_w() <= SIDEBAR_W, "nav row overflows the sidebar");

    println!("[NAV-GEOM] PASS");
}
