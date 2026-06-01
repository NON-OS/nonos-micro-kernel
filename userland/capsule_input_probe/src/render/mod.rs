pub mod font;
mod font_table;

use crate::state::Context;

const SCALE: u32 = 4;
const FG: u32 = 0x00FF_FFFF;
const BG: u32 = 0x0020_3040;

pub fn push_and_draw(ctx: &mut Context, ascii: u8) {
    if ctx.cursor >= ctx.buf.len() {
        ctx.cursor = 0;
    }
    ctx.buf[ctx.cursor] = ascii;
    ctx.cursor += 1;
    redraw(ctx);
}

fn redraw(ctx: &Context) {
    fill(ctx, BG);
    let mut x0: u32 = 8;
    for i in 0..ctx.cursor {
        draw_glyph(ctx, ctx.buf[i], x0, 8);
        x0 += font::GLYPH_W * SCALE + SCALE;
    }
}

fn draw_glyph(ctx: &Context, ascii: u8, ox: u32, oy: u32) {
    let rows = font::rows(ascii);
    for (ry, bits) in rows.iter().enumerate() {
        for rx in 0..8u32 {
            if bits & (0x80 >> rx) != 0 {
                fill_rect(ctx, ox + rx * SCALE, oy + ry as u32 * SCALE, SCALE, SCALE, FG);
            }
        }
    }
}

fn fill(ctx: &Context, argb: u32) {
    fill_rect(ctx, 0, 0, ctx.width, ctx.height, argb);
}

fn fill_rect(ctx: &Context, x: u32, y: u32, w: u32, h: u32, argb: u32) {
    for yy in y..(y + h).min(ctx.height) {
        for xx in x..(x + w).min(ctx.width) {
            let cell = (ctx.base + (yy as u64 * ctx.stride as u64 + xx as u64 * 4)) as *mut u32;
            // SAFETY: ctx.base is a mapped ARGB8888 surface of stride*height bytes
            // exclusively owned by this capsule; the loop clamps xx<width and
            // yy<height so every cell falls inside the mapping.
            unsafe { core::ptr::write_volatile(cell, argb) };
        }
    }
}
