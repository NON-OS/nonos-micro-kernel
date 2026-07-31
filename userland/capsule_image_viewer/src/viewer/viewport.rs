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
    if v <= 0.0 {
        0
    } else {
        (v + 0.5) as u32
    }
}

fn round_i(v: f32) -> i32 {
    if v < 0.0 {
        (v - 0.5) as i32
    } else {
        (v + 0.5) as i32
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FitMode {
    Fit,
    Actual,
    Fill,
}

pub fn fit_scale(sw: u32, sh: u32, vw: u32, vh: u32) -> f32 {
    if sw == 0 || sh == 0 {
        return 1.0;
    }
    let rx = vw as f32 / sw as f32;
    let ry = vh as f32 / sh as f32;
    if rx < ry {
        rx
    } else {
        ry
    }
}

pub fn base_scale(mode: FitMode, sw: u32, sh: u32, vw: u32, vh: u32) -> f32 {
    if sw == 0 || sh == 0 {
        return 1.0;
    }
    match mode {
        FitMode::Fit => fit_scale(sw, sh, vw, vh),
        FitMode::Actual => 1.0,
        FitMode::Fill => {
            let rx = vw as f32 / sw as f32;
            let ry = vh as f32 / sh as f32;
            if rx > ry {
                rx
            } else {
                ry
            }
        }
    }
}

pub fn place_mode(mode: FitMode, sw: u32, sh: u32, vw: u32, vh: u32, view: &View) -> Placement {
    let s = base_scale(mode, sw, sh, vw, vh) * view.zoom;
    let dw = round_pos(sw as f32 * s).max(1);
    let dh = round_pos(sh as f32 * s).max(1);
    let cx = (vw as f32 - dw as f32) / 2.0 + view.pan_x;
    let cy = (vh as f32 - dh as f32) / 2.0 + view.pan_y;
    Placement { dx: round_i(cx), dy: round_i(cy), dw, dh }
}

pub fn clamp_pan_mode(view: &mut View, mode: FitMode, sw: u32, sh: u32, vw: u32, vh: u32) {
    let s = base_scale(mode, sw, sh, vw, vh) * view.zoom;
    let dw = sw as f32 * s;
    let dh = sh as f32 * s;
    let lim_x = ((dw - vw as f32).abs()) / 2.0;
    let lim_y = ((dh - vh as f32).abs()) / 2.0;
    if view.pan_x > lim_x {
        view.pan_x = lim_x;
    }
    if view.pan_x < -lim_x {
        view.pan_x = -lim_x;
    }
    if view.pan_y > lim_y {
        view.pan_y = lim_y;
    }
    if view.pan_y < -lim_y {
        view.pan_y = -lim_y;
    }
}

pub fn zoom_at(
    view: &mut View,
    mode: FitMode,
    sw: u32,
    sh: u32,
    vw: u32,
    vh: u32,
    px: i32,
    py: i32,
    factor: f32,
) {
    let before = place_mode(mode, sw, sh, vw, vh, view);
    let ix = (px as f32 - before.dx as f32) / before.dw.max(1) as f32;
    let iy = (py as f32 - before.dy as f32) / before.dh.max(1) as f32;
    view.zoom = (view.zoom * factor).max(0.05).min(32.0);
    let after = place_mode(mode, sw, sh, vw, vh, &View { zoom: view.zoom, pan_x: 0.0, pan_y: 0.0 });
    let want_dx = px as f32 - ix * after.dw as f32;
    let want_dy = py as f32 - iy * after.dh as f32;
    let centre_dx = (vw as f32 - after.dw as f32) / 2.0;
    let centre_dy = (vh as f32 - after.dh as f32) / 2.0;
    view.pan_x = want_dx - centre_dx;
    view.pan_y = want_dy - centre_dy;
    clamp_pan_mode(view, mode, sw, sh, vw, vh);
}
