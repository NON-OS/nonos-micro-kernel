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

use crate::browser::js::world::{Timer, World};

use super::ctx::Ctx;

const TICK_MS: u32 = 50;
const MAX_LISTENERS: usize = 512;
const MAX_TIMERS: usize = 256;
const MAX_NET: usize = 16;

// Fold what a script run registered back into the page World: new
// listeners, timer requests converted to tick counts, and fetches that
// actually received a callback.
pub fn absorb(world: &mut World, ctx: Ctx) {
    for l in ctx.listeners {
        if world.listeners.len() >= MAX_LISTENERS {
            break;
        }
        world.listeners.push(l);
    }
    for t in ctx.timers {
        if world.timers.len() >= MAX_TIMERS {
            break;
        }
        let ticks = (t.ms / TICK_MS).max(1);
        world.timers.push(Timer {
            cb: t.cb,
            left: ticks,
            every: if t.repeat { Some(ticks) } else { None },
        });
    }
    for (url, cb) in ctx.net {
        if world.net.len() >= MAX_NET {
            break;
        }
        if let Some(cb) = cb {
            world.net.push((url, cb));
        }
    }
}
