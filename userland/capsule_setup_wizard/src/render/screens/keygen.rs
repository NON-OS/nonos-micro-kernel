use crate::debug;
use crate::render;
use crate::server::step::{default_key, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    let label: &[u8] = if ctx.keys_done {
        b"SYSTEM KEYS  GENERATED"
    } else {
        b"SYSTEM KEYS  ENTER TO GENERATE"
    };
    render::frame(ctx, label, b"Generate this machine's identity keys", b"ENTER NEXT  ESC BACK");
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if code == K_ENTER || code == K_ENTER_LF {
        if !ctx.keys_done {
            ctx.keys_done = true;
            debug::marker(b"keygen ok");
        }
        return Outcome::Advance;
    }
    default_key(code)
}
