pub mod screens;

use nonos_toolkit::font::render::draw_text;

use crate::state::Context;

const BG: u32 = 0xFF20_3040;
const FG: u32 = 0xFFFF_FFFF;
const DOT_DONE: u32 = 0xFF4F_D1C5;
const DOT_TODO: u32 = 0xFF22_303D;
const STEPS: u32 = 6;
const DOT: u32 = 10;
const GAP: u32 = 18;

fn fill_rect(buf: &mut [u32], spx: usize, h: u32, x: u32, y: u32, w: u32, rh: u32, argb: u32) {
    for yy in y..(y + rh).min(h) {
        for xx in x..(x + w) {
            let idx = yy as usize * spx + xx as usize;
            if idx < buf.len() {
                buf[idx] = argb;
            }
        }
    }
}

pub fn frame(ctx: &Context, title: &[u8], footer: &[u8]) {
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    // SAFETY: ctx.base is a mapped ARGB8888 surface of stride*height bytes,
    // exclusively owned by this capsule; spx*h stays within the mapping.
    let buf = unsafe { core::slice::from_raw_parts_mut(ctx.base as *mut u32, spx * h as usize) };
    for p in buf.iter_mut() {
        *p = BG;
    }
    for i in 0..STEPS {
        let color = if i <= ctx.step as u32 { DOT_DONE } else { DOT_TODO };
        fill_rect(buf, spx, h, 24 + i * GAP, 22, DOT, DOT, color);
    }
    draw_text(buf, spx, w, h, 24, 52, title, FG);
    draw_text(buf, spx, w, h, 24, h.saturating_sub(24), footer, FG);
}
