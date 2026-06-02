pub mod keyboard;
pub mod review;
pub mod welcome;

use crate::server::step::{default_key, Outcome};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    match ctx.step {
        0 => welcome::draw(ctx),
        1 => keyboard::draw(ctx),
        5 => review::draw(ctx),
        _ => crate::render::frame(ctx, b"SETUP STEP", b"ENTER NEXT  ESC BACK"),
    }
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match ctx.step {
        0 => welcome::on_key(ctx, code),
        1 => keyboard::on_key(ctx, code),
        5 => review::on_key(ctx, code),
        _ => default_key(code),
    }
}
