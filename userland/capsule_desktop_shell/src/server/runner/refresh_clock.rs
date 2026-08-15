// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use nonos_libc::mk_time_millis;

use crate::compositor_client::push_damage_commit;
use crate::render::layout::menubar_height;
use crate::render::paint_chrome;
use crate::state::indicators::{net, policy};
use crate::state::{Context, NotifyLevel};

pub(super) fn refresh_clock(ctx: &mut Context) {
    if let Some(v) = policy::clock_24h(&mut ctx.policy_port) {
        ctx.clock_24h = v;
    }
    let net_now = net::online();
    if net_now && !ctx.net_was_online {
        ctx.toasts.push(b"network connected", NotifyLevel::Info, mk_time_millis());
    }
    ctx.net_was_online = net_now;
    // paint_chrome redraws the whole overlay backing, but only the top bar's
    // pixels actually change each second. Damage just that strip: a full-screen
    // damage every tick would make the compositor recomposite every layer,
    // including any open app window, which is what made the desktop hitch.
    paint_chrome(ctx);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, 0, 0, ctx.width, menubar_height());
}
