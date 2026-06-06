use crate::render::{self, widgets};
use crate::server::step::{default_key, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    render::frame(
        ctx,
        b"Identity keys",
        b"Ed25519 + ML-DSA-65 keypair",
        b"ENTER GENERATE/NEXT  ESC BACK",
    );
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    let tasks: [(&[u8], bool); 2] = [
        (b"Ed25519 keypair", ctx.keygen_stage >= 1),
        (b"ML-DSA-65 keypair", ctx.keygen_stage >= 2),
    ];
    let pct = (ctx.keygen_stage as u32) * 50;
    widgets::progress::busy(buf, spx, w, h, render::content_x(w), 120, &tasks, pct);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if code == K_ENTER || code == K_ENTER_LF {
        if ctx.keygen_stage < 2 {
            ctx.keygen_stage += 1;
            if ctx.keygen_stage == 2 {
                ctx.keys_done = true;
            }
            return Outcome::Stay;
        }
        return Outcome::Advance;
    }
    default_key(code)
}
