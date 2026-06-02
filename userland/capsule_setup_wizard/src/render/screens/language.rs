use nonos_policy_proto::language_labels::LANGUAGE_LABELS;

use crate::render::{self, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Language", b"j/k or 1-9 to choose", b"ENTER NEXT");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    rows::list(buf, spx, w, h, render::content_x(w), 110, LANGUAGE_LABELS, ctx.lang_sel as usize);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if let Some(o) = list_nav(&mut ctx.lang_sel, LANGUAGE_LABELS.len() as u8, code) {
        return o;
    }
    default_key(code)
}
