pub const BTN_W: u32 = 40;
pub const BTN_H: u32 = 64;
pub const MARGIN: u32 = 8;
pub const SWIPE_MIN: i32 = 80;

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

pub fn button_rects(vw: u32, vh: u32) -> (Rect, Rect) {
    let y = (vh.saturating_sub(BTN_H) / 2) as i32;
    let left = Rect { x: MARGIN as i32, y, w: BTN_W, h: BTN_H };
    let right = Rect { x: vw.saturating_sub(MARGIN + BTN_W) as i32, y, w: BTN_W, h: BTN_H };
    (left, right)
}

fn inside(r: &Rect, x: i32, y: i32) -> bool {
    x >= r.x && x < r.x + r.w as i32 && y >= r.y && y < r.y + r.h as i32
}

pub fn hit_nav(x: i32, y: i32, vw: u32, vh: u32) -> Option<i32> {
    let (l, r) = button_rects(vw, vh);
    if inside(&l, x, y) {
        Some(-1)
    } else if inside(&r, x, y) {
        Some(1)
    } else {
        None
    }
}

pub fn swipe_delta(dx: i32, dy: i32, pannable: bool) -> Option<i32> {
    if pannable || dx.abs() <= SWIPE_MIN || dx.abs() <= dy.abs() {
        return None;
    }
    if dx < 0 {
        Some(1)
    } else {
        Some(-1)
    }
}
