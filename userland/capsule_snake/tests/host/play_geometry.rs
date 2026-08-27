#[path = "../../src/snake"]
mod snake {
    #[path = "grid.rs"]
    pub mod grid;
    #[path = "ui"]
    pub mod ui {
        #[path = "metrics.rs"]
        pub mod metrics;
        #[path = "play_geom.rs"]
        pub mod play_geom;
        #[path = "play_geom_rows.rs"]
        pub mod play_geom_rows;
        #[path = "rect.rs"]
        pub mod rect;
    }
}

use snake::grid::COLS;
use snake::ui::play_geom::{board, foot_band, hud_band, rail, stage};
use snake::ui::play_geom_rows::{
    foot, foot_at, hud, hud_at, rail_row, FOOT_BTNS, HUD_CARDS, RAIL_ROWS,
};
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
        let (c, hb, fb, rl, st) =
            (content(w, h), hud_band(w, h), foot_band(w, h), rail(w, h), stage(w, h));
        check(inside(c, hb), "hud band escapes content");
        check(inside(c, fb), "foot band escapes content");
        check(disjoint(hb, fb), "hud band overlaps foot band");
        check(inside(c, rl), "rail escapes content");
        check(disjoint(st, rl), "stage overlaps rail");
        let b = board(w, h);
        check(inside(st, (b.x, b.y, b.w, b.h)), "board escapes stage");
        check(b.cell >= 4, "cell collapsed");
        check(b.w == b.cell * COLS as u32, "board width not cell-aligned");
        for i in 0..FOOT_BTNS {
            let r = foot(w, h, i);
            check(inside(fb, r), "foot button escapes band");
            check(foot_at(w, h, centre(r).0, centre(r).1) == Some(i), "foot hit test drifts");
        }
        for i in 0..HUD_CARDS {
            let r = hud(w, h, i);
            check(inside(hb, r), "hud card escapes band");
            check(hud_at(w, h, centre(r).0, centre(r).1) == Some(i), "hud hit test drifts");
        }
        for i in 0..RAIL_ROWS {
            check(inside(rl, rail_row(w, h, i)), "rail row escapes rail");
        }
    }
    println!("[PLAY-GEOM] {}", if fail == 0 { "PASS" } else { "FAIL" });
    std::process::exit(if fail == 0 { 0 } else { 1 });
}
