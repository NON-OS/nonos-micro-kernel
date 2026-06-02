pub mod admin;
pub mod appearance;
pub mod keygen;
pub mod keyboard;
pub mod language;
pub mod network;
pub mod passphrase;
pub mod persistence;
pub mod privacy;
pub mod review;

use crate::server::step::{default_key, Outcome};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    match ctx.step {
        0 => language::draw(ctx),
        1 => keyboard::draw(ctx),
        2 => keygen::draw(ctx),
        3 => passphrase::draw(ctx),
        4 => persistence::draw(ctx),
        5 => network::draw(ctx),
        6 => admin::draw(ctx),
        7 => privacy::draw(ctx),
        8 => appearance::draw(ctx),
        9 => review::draw(ctx),
        _ => crate::render::frame(ctx, b"Setup", b"", b"ENTER NEXT  ESC BACK"),
    }
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match ctx.step {
        0 => language::on_key(ctx, code),
        1 => keyboard::on_key(ctx, code),
        2 => keygen::on_key(ctx, code),
        3 => passphrase::on_key(ctx, code),
        4 => persistence::on_key(ctx, code),
        5 => network::on_key(ctx, code),
        6 => admin::on_key(ctx, code),
        7 => privacy::on_key(ctx, code),
        8 => appearance::on_key(ctx, code),
        9 => review::on_key(ctx, code),
        _ => default_key(code),
    }
}
