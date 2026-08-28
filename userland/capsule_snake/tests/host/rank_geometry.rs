#[path = "../../src/snake"]
mod snake {
    #[path = "ui"]
    pub mod ui {
        #[path = "metrics.rs"]
        pub mod metrics;
        #[path = "rank_geom.rs"]
        pub mod rank_geom;
        #[path = "rank_geom_cols.rs"]
        pub mod rank_geom_cols;
        #[path = "rect.rs"]
        pub mod rect;
    }
}

use snake::ui::metrics::RANK_ROWS;
use snake::ui::rank_geom::{award_row, awards, back, back_at, head, row, row_at, table, AWARD_ROWS};
use snake::ui::rank_geom_cols::{column, COLUMNS};
use snake::ui::rect::{content, Rect};

const SIZES: [(u32, u32); 4] = [(1240, 752), (1200, 692), (2560, 1412), (900, 600)];

fn inside(o: Rect, r: Rect) -> bool {
    r.0 >= o.0 && r.1 >= o.1 && r.0 + r.2 <= o.0 + o.2 && r.1 + r.3 <= o.1 + o.3
}

fn disjoint(a: Rect, b: Rect) -> bool {
    a.0 + a.2 <= b.0 || b.0 + b.2 <= a.0 || a.1 + a.3 <= b.1 || b.1 + b.3 <= a.1
}

fn centre(r: Rect) -> (i32, i32) {
    ((r.0 + r.2 / 2) as i32, (r.1 + r.3 / 2) as i32)
}

fn main() {
    let mut fail = 0usize;
    for (w, h) in SIZES {
        let mut check = |ok: bool, what: &str| {
            if !ok {
                println!("{}x{}: {}", w, h, what);
                fail += 1;
            }
        };
        let (c, tb, aw) = (content(w, h), table(w, h), awards(w, h));
        check(inside(c, tb), "table escapes content");
        check(inside(c, aw), "awards escape content");
        check(disjoint(tb, aw), "table overlaps awards");
        check(inside(tb, head(w, h)), "head escapes table");
        for i in 0..RANK_ROWS {
            let r = row(w, h, i);
            check(inside(tb, r), "rank row escapes table");
            check(row_at(w, h, centre(r).0, centre(r).1) == Some(i), "rank hit test drifts");
        }
        for i in 0..AWARD_ROWS {
            check(inside(aw, award_row(w, h, i)), "award row escapes awards panel");
        }
        let b = back(w, h);
        check(inside(c, b), "back button escapes content");
        check(back_at(w, h, centre(b).0, centre(b).1), "back hit test drifts");
        let mut prev_end = 0u32;
        for i in 0..COLUMNS {
            let (x, cw) = column(w, h, i);
            check(x >= prev_end, "rank columns overlap");
            check(x + cw <= tb.0 + tb.2, "rank column escapes table");
            prev_end = x + cw;
        }
    }
    println!("[RANK-GEOM] {}", if fail == 0 { "PASS" } else { "FAIL" });
    std::process::exit(if fail == 0 { 0 } else { 1 });
}
