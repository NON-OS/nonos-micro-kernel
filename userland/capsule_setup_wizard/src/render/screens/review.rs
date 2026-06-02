use nonos_policy_proto::Field;

use crate::clients::policy;
use crate::debug;
use crate::render;
use crate::server::step::{default_key, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Review", b"Confirm and finish setup", b"ENTER FINISH  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    let lines: [(&[u8], bool); 3] = [(b"Identity keys", ctx.keys_done), (b"Passphrase set", ctx.pass_len > 0), (b"Layout/wallpaper chosen", true)];
    render::widgets::progress::busy(buf, spx, w, h, render::content_x(w), 120, &lines, 0);
}

fn commit(ctx: &Context) {
    let p = ctx.policy_port;
    if p == 0 {
        return;
    }
    let _ = policy::set_u8(p, Field::Language as u32, ctx.lang_sel);
    let _ = policy::set_u8(p, Field::KeyboardLayout as u32, ctx.kbd_sel);
    let _ = policy::set_i8(p, Field::Timezone as u32, ctx.tz_off);
    let _ = policy::set_u8(p, Field::Wallpaper as u32, ctx.wall_sel);
    let _ = policy::set_u8(p, Field::Theme as u32, ctx.theme_sel);
    let _ = policy::set_bool(p, Field::SystemKeysGenerated as u32, true);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if code == K_ENTER || code == K_ENTER_LF {
        commit(ctx);
        debug::marker(b"finish");
        return Outcome::Advance;
    }
    default_key(code)
}
