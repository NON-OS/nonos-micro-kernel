use nonos_policy_proto::keyboard_layout_labels::KEYBOARD_LAYOUT_LABELS;

use crate::render::{self, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Keyboard layout", b"j/k or 1-9 to choose", b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    rows::list(
        buf,
        spx,
        w,
        h,
        render::content_x(w),
        110,
        KEYBOARD_LAYOUT_LABELS,
        ctx.kbd_sel as usize,
    );
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    let n = KEYBOARD_LAYOUT_LABELS.len() as u8;
    if let Some(o) = list_nav(&mut ctx.kbd_sel, n, code) {
        return o;
    }
    default_key(code)
}
