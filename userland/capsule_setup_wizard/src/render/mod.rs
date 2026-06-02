pub mod chrome;
pub mod paint;
pub mod screens;
pub mod theme;
pub mod widgets;

use nonos_toolkit::font::render::draw_text;

use crate::render::paint::fill_rect;
use crate::render::theme::{BACKDROP, CARD_BG, FG, HINT};
use crate::state::Context;

pub fn buffer(ctx: &Context) -> &'static mut [u32] {
    let spx = ctx.stride as usize / 4;
    unsafe { core::slice::from_raw_parts_mut(ctx.base as *mut u32, spx * ctx.height as usize) }
}

pub fn content_x(w: u32) -> u32 {
    w * 38 / 100 + 30
}

pub fn frame(ctx: &Context, title: &[u8], sub: &[u8], footer: &[u8]) {
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = buffer(ctx);
    for p in buf.iter_mut() {
        *p = BACKDROP;
    }
    let pw = w * 38 / 100;
    fill_rect(buf, spx, w, h, pw + 2, 0, w.saturating_sub(pw + 2), h, CARD_BG);
    chrome::panel(buf, spx, w, h, ctx);
    draw_text(buf, spx, w, h, content_x(w), 44, title, FG);
    draw_text(buf, spx, w, h, content_x(w), 70, sub, HINT);
    draw_text(buf, spx, w, h, content_x(w), h - 40, footer, HINT);
}
