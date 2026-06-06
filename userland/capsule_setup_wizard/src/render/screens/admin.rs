use crate::render::{self, widgets};
use crate::server::step::{default_key, Outcome, K_BACKSPACE, K_ENTER, K_ENTER_LF};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    render::frame(
        ctx,
        b"Administration password",
        b"Used to authorise privileged actions",
        b"TYPE  BACKSPACE EDIT  ENTER NEXT  ESC BACK",
    );
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    widgets::field::masked(
        buf,
        spx,
        w,
        h,
        render::content_x(w),
        120,
        ctx.admin_len,
        widgets::field::strength_of(ctx.admin_len),
    );
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match code {
        K_ENTER | K_ENTER_LF => Outcome::Advance,
        K_BACKSPACE => {
            ctx.admin_len = ctx.admin_len.saturating_sub(1);
            Outcome::Stay
        }
        0x20..=0x7E => {
            if ctx.admin_len < ctx.admin_buf.len() {
                ctx.admin_buf[ctx.admin_len] = code as u8;
                ctx.admin_len += 1;
            }
            Outcome::Stay
        }
        _ => default_key(code),
    }
}
