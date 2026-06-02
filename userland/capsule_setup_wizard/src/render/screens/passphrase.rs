use crate::debug;
use crate::render;
use crate::server::step::{default_key, Outcome, K_BACKSPACE, K_ENTER, K_ENTER_LF};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    let prefix = b"PASSPHRASE  ";
    let mut buf = [0u8; 48];
    buf[..prefix.len()].copy_from_slice(prefix);
    let n = ctx.pass_len.min(buf.len() - prefix.len());
    for i in 0..n {
        buf[prefix.len() + i] = b'*';
    }
    render::frame(ctx, &buf[..prefix.len() + n], b"TYPE  BACKSPACE EDIT  ENTER NEXT  ESC BACK");
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match code {
        K_ENTER | K_ENTER_LF => {
            debug::marker(b"passphrase set");
            Outcome::Advance
        }
        K_BACKSPACE => {
            ctx.pass_len = ctx.pass_len.saturating_sub(1);
            Outcome::Stay
        }
        0x20..=0x7E => {
            if ctx.pass_len < ctx.pass_buf.len() {
                ctx.pass_buf[ctx.pass_len] = code as u8;
                ctx.pass_len += 1;
            }
            Outcome::Stay
        }
        _ => default_key(code),
    }
}
