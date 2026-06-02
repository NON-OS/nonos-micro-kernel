use crate::render;
use crate::server::step::{default_key, Outcome};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    let label: &[u8] = if ctx.kbd_sel == 0 {
        b"KEYBOARD  1 US  2 INTL  -> US"
    } else {
        b"KEYBOARD  1 US  2 INTL  -> INTL"
    };
    render::frame(ctx, label, b"1/2 SELECT  ENTER NEXT  ESC BACK");
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match code {
        0x31 => {
            ctx.kbd_sel = 0;
            Outcome::Stay
        }
        0x32 => {
            ctx.kbd_sel = 1;
            Outcome::Stay
        }
        _ => default_key(code),
    }
}
