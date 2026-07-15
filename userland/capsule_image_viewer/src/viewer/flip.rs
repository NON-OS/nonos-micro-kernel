extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

pub fn flip_h(src: &[u32], w: u32, h: u32) -> Vec<u32> {
    let mut out = vec![0u32; (w * h) as usize];
    let mut y = 0u32;
    while y < h {
        let mut x = 0u32;
        while x < w {
            out[(y * w + x) as usize] = src[(y * w + (w - 1 - x)) as usize];
            x += 1;
        }
        y += 1;
    }
    out
}

pub fn flip_v(src: &[u32], w: u32, h: u32) -> Vec<u32> {
    let mut out = vec![0u32; (w * h) as usize];
    let mut y = 0u32;
    while y < h {
        out[(y * w) as usize..((y + 1) * w) as usize]
            .copy_from_slice(&src[((h - 1 - y) * w) as usize..((h - y) * w) as usize]);
        y += 1;
    }
    out
}
