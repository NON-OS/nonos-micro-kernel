pub struct Dst<'a> { pub px: &'a mut [u32], pub stride: u32, pub w: u32, pub h: u32 }

pub fn draw_nn(dst: &mut Dst, src: &[u32], sw: u32, sh: u32,
               dx: i32, dy: i32, dw: u32, dh: u32) {
    if sw == 0 || sh == 0 || dw == 0 || dh == 0 { return; }
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
