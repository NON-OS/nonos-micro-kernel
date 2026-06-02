use crate::render::{self, widgets::toggles};
use crate::server::step::{default_key, Outcome};
use crate::state::Context;

const ITEMS: &[&[u8]] = &[b"MAC address randomization", b"Secure-wipe RAM on shutdown", b"Telemetry"];

fn focus(ctx: &Context) -> usize {
    (ctx.privacy >> 8) as usize % ITEMS.len()
}

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Privacy & hardening", b"SPACE toggles, j/k moves", b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    toggles::list(buf, spx, w, h, render::content_x(w), 110, ITEMS, ctx.privacy & 0xFF, focus(ctx));
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    let f = focus(ctx);
    match code {
        0x20 => {
            ctx.privacy ^= 1 << f;
            Outcome::Stay
        }
        0x6B => {
            let nf = (f + ITEMS.len() - 1) % ITEMS.len();
            ctx.privacy = (ctx.privacy & 0xFF) | ((nf as u16) << 8);
            Outcome::Stay
        }
        0x6A => {
            let nf = (f + 1) % ITEMS.len();
            ctx.privacy = (ctx.privacy & 0xFF) | ((nf as u16) << 8);
            Outcome::Stay
        }
        _ => default_key(code),
    }
}
