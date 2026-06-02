use crate::render::{self, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

const WALLS: &[&[u8]] = &[b"Deep", b"Slate", b"Night"];

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Wallpaper", b"j/k or 1-3 to choose", b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    rows::list(buf, spx, w, h, render::content_x(w), 110, WALLS, ctx.wall_sel as usize);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if let Some(o) = list_nav(&mut ctx.wall_sel, WALLS.len() as u8, code) {
        return o;
    }
    default_key(code)
}
