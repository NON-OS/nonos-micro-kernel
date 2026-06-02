use crate::clients::policy;
use crate::debug;
use crate::render;
use crate::server::step::{default_key, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

const SYSTEM_KEYS_GENERATED: u32 = 0x010F;

pub fn draw(ctx: &Context) {
    let label: &[u8] = if ctx.kbd_sel == 0 {
        b"REVIEW  KBD US  KEYS OK  PASS OK"
    } else {
        b"REVIEW  KBD INTL  KEYS OK  PASS OK"
    };
    render::frame(ctx, label, b"Confirm and finish setup", b"ENTER FINISH  ESC BACK");
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if code == K_ENTER || code == K_ENTER_LF {
        if ctx.policy_port != 0 {
            let _ = policy::set_bool(ctx.policy_port, SYSTEM_KEYS_GENERATED, true);
        }
        debug::marker(b"finish");
        return Outcome::Advance;
    }
    default_key(code)
}
