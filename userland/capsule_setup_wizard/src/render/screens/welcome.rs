use crate::render;
use crate::server::step::{default_key, Outcome};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"WELCOME TO NONOS", b"ENTER BEGIN");
}

pub fn on_key(_ctx: &mut Context, code: u32) -> Outcome {
    default_key(code)
}
