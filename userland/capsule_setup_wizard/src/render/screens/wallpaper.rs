use crate::render;
use crate::server::step::{default_key, Outcome};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    let label: &[u8] = match ctx.wall_sel {
        0 => b"WALLPAPER  1 DEEP  2 SLATE  3 NIGHT  -> DEEP",
        1 => b"WALLPAPER  1 DEEP  2 SLATE  3 NIGHT  -> SLATE",
        _ => b"WALLPAPER  1 DEEP  2 SLATE  3 NIGHT  -> NIGHT",
    };
    render::frame(ctx, label, b"Pick a desktop wallpaper", b"1-3 SELECT  ENTER NEXT  ESC BACK");
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match code {
        0x31 => {
            ctx.wall_sel = 0;
            Outcome::Stay
        }
        0x32 => {
            ctx.wall_sel = 1;
            Outcome::Stay
        }
        0x33 => {
            ctx.wall_sel = 2;
            Outcome::Stay
        }
        _ => default_key(code),
    }
}
