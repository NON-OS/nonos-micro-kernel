use crate::render::{self, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

const MODES: &[&[u8]] = &[b"Amnesic (RAM only)", b"Persistent encrypted store"];

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Persistence", b"Keep data across reboots?", b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    rows::list(buf, spx, w, h, render::content_x(w), 110, MODES, ctx.persist_sel as usize);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if let Some(o) = list_nav(&mut ctx.persist_sel, MODES.len() as u8, code) {
        return o;
    }
    default_key(code)
}
