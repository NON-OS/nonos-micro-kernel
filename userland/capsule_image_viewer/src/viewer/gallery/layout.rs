pub const THUMB_W: u32 = 160;
pub const THUMB_H: u32 = 120;
pub const GAP: u32 = 12;
pub const HEADER_H: u32 = 28;
pub const LABEL_H: u32 = 16;

pub struct Grid {
    pub cols: u32,
    pub cell_w: u32,
    pub cell_h: u32,
}

pub fn grid(win_w: u32) -> Grid {
    let cell_w = THUMB_W + GAP;
    let cell_h = THUMB_H + LABEL_H + GAP;
    let cols = (win_w.saturating_sub(GAP) / cell_w).max(1);
    Grid { cols, cell_w, cell_h }
}

pub fn cell_rect(i: usize, scroll_rows: usize, g: &Grid) -> (i32, i32, u32, u32) {
    let col = (i as u32) % g.cols;
    let row = (i as u32) / g.cols;
    let x = (GAP + col * g.cell_w) as i32;
    let y = HEADER_H as i32 + GAP as i32 + (row as i32 - scroll_rows as i32) * g.cell_h as i32;
    (x, y, THUMB_W, THUMB_H + LABEL_H)
}

pub fn rows(count: usize, g: &Grid) -> usize {
    let cols = g.cols.max(1) as usize;
    (count + cols - 1) / cols
}

pub fn max_scroll(count: usize, win_h: u32, g: &Grid) -> usize {
    let total = rows(count, g);
    let visible = (win_h.saturating_sub(HEADER_H) / g.cell_h).max(1) as usize;
    total.saturating_sub(visible)
}
