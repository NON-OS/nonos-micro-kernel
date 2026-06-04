use nonos_toolkit::font::render::{draw_text, draw_text_scaled};

use crate::render::paint::{fill_rect, gradient_v};
use crate::render::theme::{
    ACCENT, DOT_CUR, DOT_DONE, DOT_TODO, GRAD_BOT, GRAD_TOP, HINT, STEP_LABELS,
};
use crate::state::Context;

const WORDMARK: &[u8] = b"N\xD8NOS";

pub fn panel(buf: &mut [u32], spx: usize, w: u32, h: u32, ctx: &Context) {
    let pw = w * 38 / 100;
    gradient_v(buf, spx, w, h, 0, 0, pw, h, GRAD_TOP, GRAD_BOT);
    fill_rect(buf, spx, w, h, pw, 0, 2, h, ACCENT);
    draw_text_scaled(buf, spx, w, h, 32, 40, WORDMARK, ACCENT, 3);
    draw_text(buf, spx, w, h, 34, 96, b"FIRST-BOOT SETUP", HINT);
    let mut y = 150u32;
    for (i, label) in STEP_LABELS.iter().enumerate() {
        let (mark, color) = mark_for(ctx.step as usize, i);
        draw_text(buf, spx, w, h, 34, y, mark, color);
        draw_text(buf, spx, w, h, 52, y, label, color);
        y += 22;
    }
}

fn mark_for(cur: usize, i: usize) -> (&'static [u8], u32) {
    if i < cur {
        (b"+", DOT_DONE)
    } else if i == cur {
        (b">", DOT_CUR)
    } else {
        (b".", DOT_TODO)
    }
}
