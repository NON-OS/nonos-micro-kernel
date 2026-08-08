pub struct Dst<'a> {
    pub px: &'a mut [u32],
    pub stride: u32,
    pub w: u32,
    pub h: u32,
}

pub fn draw_nn(dst: &mut Dst, src: &[u32], sw: u32, sh: u32, dx: i32, dy: i32, dw: u32, dh: u32) {
    if sw == 0 || sh == 0 || dw == 0 || dh == 0 {
        return;
    }
    let x0 = dx.max(0);
    let y0 = dy.max(0);
    let x1 = (dx + dw as i32).min(dst.w as i32);
    let y1 = (dy + dh as i32).min(dst.h as i32);
    let mut yy = y0;
    while yy < y1 {
        let sy = (((yy - dy) as u32) * sh / dh).min(sh - 1);
        let mut xx = x0;
        while xx < x1 {
            let sx = (((xx - dx) as u32) * sw / dw).min(sw - 1);
            let sample = src[sy as usize * sw as usize + sx as usize];
            let idx = yy as usize * dst.stride as usize + xx as usize;
            dst.px[idx] = 0xFF00_0000 | (sample & 0x00FF_FFFF);
            xx += 1;
        }
        yy += 1;
    }
}

pub fn draw_bilinear(
    dst: &mut Dst,
    src: &[u32],
    sw: u32,
    sh: u32,
    dx: i32,
    dy: i32,
    dw: u32,
    dh: u32,
) {
    if sw == 0 || sh == 0 || dw == 0 || dh == 0 {
        return;
    }
    let x0 = dx.max(0);
    let y0 = dy.max(0);
    let x1 = (dx + dw as i32).min(dst.w as i32);
    let y1 = (dy + dh as i32).min(dst.h as i32);
    let mut yy = y0;
    while yy < y1 {
        let fy = ((yy - dy) as f32 + 0.5) * sh as f32 / dh as f32 - 0.5;
        let (sy0, wy) = split(fy, sh);
        let sy1 = (sy0 + 1).min(sh - 1);
        let mut xx = x0;
        while xx < x1 {
            let fx = ((xx - dx) as f32 + 0.5) * sw as f32 / dw as f32 - 0.5;
            let (sx0, wx) = split(fx, sw);
            let sx1 = (sx0 + 1).min(sw - 1);
            let c00 = src[sy0 as usize * sw as usize + sx0 as usize];
            let c01 = src[sy0 as usize * sw as usize + sx1 as usize];
            let c10 = src[sy1 as usize * sw as usize + sx0 as usize];
            let c11 = src[sy1 as usize * sw as usize + sx1 as usize];
            let top = lerp_px(c00, c01, wx);
            let bot = lerp_px(c10, c11, wx);
            let px = lerp_px(top, bot, wy);
            dst.px[yy as usize * dst.stride as usize + xx as usize] =
                0xFF00_0000 | (px & 0x00FF_FFFF);
            xx += 1;
        }
        yy += 1;
    }
}

fn split(f: f32, n: u32) -> (u32, f32) {
    if f <= 0.0 {
        return (0, 0.0);
    }
    let i = f as u32;
    if i >= n - 1 {
        return (n - 1, 0.0);
    }
    (i, f - i as f32)
}

fn lerp_px(a: u32, b: u32, t: f32) -> u32 {
    let l = |sh: u32| {
        let ca = ((a >> sh) & 0xFF) as f32;
        let cb = ((b >> sh) & 0xFF) as f32;
        (ca + (cb - ca) * t + 0.5) as u32 & 0xFF
    };
    (l(16) << 16) | (l(8) << 8) | l(0)
}
