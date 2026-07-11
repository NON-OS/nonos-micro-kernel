pub struct View {
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
}

pub struct Placement {
    pub dx: i32,
    pub dy: i32,
    pub dw: u32,
    pub dh: u32,
}

fn round_pos(v: f32) -> u32 {
    if v <= 0.0 { 0 } else { (v + 0.5) as u32 }
}

fn round_i(v: f32) -> i32 {
    if v < 0.0 { (v - 0.5) as i32 } else { (v + 0.5) as i32 }
}

pub fn fit_scale(sw: u32, sh: u32, vw: u32, vh: u32) -> f32 {
    if sw == 0 || sh == 0 {
        return 1.0;
    }
    let rx = vw as f32 / sw as f32;
    let ry = vh as f32 / sh as f32;
    if rx < ry { rx } else { ry }
}

pub fn place(sw: u32, sh: u32, vw: u32, vh: u32, view: &View) -> Placement {
    let s = fit_scale(sw, sh, vw, vh) * view.zoom;
    let dw = round_pos(sw as f32 * s).max(1);
    let dh = round_pos(sh as f32 * s).max(1);
    let cx = (vw as f32 - dw as f32) / 2.0 + view.pan_x;
    let cy = (vh as f32 - dh as f32) / 2.0 + view.pan_y;
    Placement { dx: round_i(cx), dy: round_i(cy), dw, dh }
}

pub fn clamp_pan(view: &mut View, sw: u32, sh: u32, vw: u32, vh: u32) {
    let s = fit_scale(sw, sh, vw, vh) * view.zoom;
    let dw = sw as f32 * s;
    let dh = sh as f32 * s;
    let lim_x = ((dw - vw as f32).abs()) / 2.0;
    let lim_y = ((dh - vh as f32).abs()) / 2.0;
    if view.pan_x > lim_x { view.pan_x = lim_x; }
    if view.pan_x < -lim_x { view.pan_x = -lim_x; }
    if view.pan_y > lim_y { view.pan_y = lim_y; }
    if view.pan_y < -lim_y { view.pan_y = -lim_y; }
}
