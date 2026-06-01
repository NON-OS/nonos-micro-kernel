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

use crate::catalog_client::lookup_catalog;
use crate::debug;
use crate::policy_client::{get_wallpaper, lookup_policy};
use crate::state::Context;

use super::apply::apply;

const POLL_EVERY: u32 = 1;

pub fn tick(ctx: &mut Context) {
    ctx.subscriber_ticks = ctx.subscriber_ticks.wrapping_add(1);
    if ctx.subscriber_ticks % POLL_EVERY != 0 {
        return;
    }
    if ctx.policy_port.is_none() {
        ctx.policy_port = lookup_policy();
    }
    if ctx.catalog_port.is_none() {
        ctx.catalog_port = lookup_catalog();
    }
    let policy_port = match ctx.policy_port {
        Some(p) => p,
        None => return,
    };
    let wanted = match get_wallpaper(policy_port) {
        Some(v) => v,
        None => return,
    };
    if ctx.applied_wallpaper == Some(wanted) {
        return;
    }
    if apply(ctx, wanted) {
        ctx.applied_wallpaper = Some(wanted);
    } else {
        debug::marker(b"apply failed");
    }
}
