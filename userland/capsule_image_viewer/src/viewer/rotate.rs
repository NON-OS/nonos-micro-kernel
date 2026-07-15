extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

pub fn rotate_cw(src: &[u32], w: u32, h: u32) -> (Vec<u32>, u32, u32) {
    let (nw, nh) = (h, w);
    let mut out = vec![0u32; (nw * nh) as usize];
    let mut y = 0u32;
    while y < h {
        let mut x = 0u32;
        while x < w {
            let s = src[(y * w + x) as usize];
            let (ox, oy) = (h - 1 - y, x);
            out[(oy * nw + ox) as usize] = s;
            x += 1;
        }
        y += 1;
    }
    (out, nw, nh)
}
