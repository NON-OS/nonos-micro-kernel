pub fn fill_rect(
    buf: &mut [u32],
    spx: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    rw: u32,
    rh: u32,
    argb: u32,
) {
    let x1 = (x + rw).min(w);
    for yy in y..(y + rh).min(h) {
        let row = yy as usize * spx;
        for xx in x..x1 {
            let idx = row + xx as usize;
            if idx < buf.len() {
                buf[idx] = argb;
            }
        }
    }
}

pub fn lerp_argb(a: u32, b: u32, num: u32, den: u32) -> u32 {
    let chan = |sh: u32| -> u32 {
        let ca = ((a >> sh) & 0xFF) as i32;
        let cb = ((b >> sh) & 0xFF) as i32;
        (ca + (cb - ca) * num as i32 / den as i32) as u32
    };
    0xFF00_0000 | (chan(16) << 16) | (chan(8) << 8) | chan(0)
}

pub fn gradient_v(
    buf: &mut [u32],
    spx: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    rw: u32,
    rh: u32,
    top: u32,
    bot: u32,
) {
    let den = rh.max(1);
    for row in 0..rh {
        let c = lerp_argb(top, bot, row, den);
        fill_rect(buf, spx, w, h, x, y + row, rw, 1, c);
    }
}
