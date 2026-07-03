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

use alloc::vec::Vec;

use crate::browser::dom::Dom;
use crate::browser::js::world::{Timer, World};

use super::absorb::absorb;
use super::apply::apply;
use super::ctx::Ctx;

const PUMP_BUDGET: u64 = 1_000_000;

// One 50ms tick: age every timer, run the due ones, requeue intervals.
// Returns (any callback ran, DOM went dirty).
pub fn pump_timers(dom: &mut Dom, world: &mut World) -> (bool, bool) {
    if world.timers.is_empty() {
        return (false, false);
    }
    let mut due: Vec<Timer> = Vec::new();
    for t in world.timers.iter_mut() {
        t.left = t.left.saturating_sub(1);
    }
    let mut keep: Vec<Timer> = Vec::new();
    for t in core::mem::take(&mut world.timers) {
        if t.left == 0 {
            due.push(t);
        } else {
            keep.push(t);
        }
    }
    world.timers = keep;
    if due.is_empty() {
        return (false, false);
    }
    let mut ctx = Ctx::new(dom, PUMP_BUDGET);
    let mut fired = false;
    for t in due {
        if ctx.steps >= ctx.budget {
            break;
        }
        fired = true;
        let _ = apply(&mut ctx, &world.env, t.cb.clone(), Vec::new());
        if let Some(every) = t.every {
            world.timers.push(Timer { cb: t.cb, left: every.max(1), every: t.every });
        }
    }
    let dirty = ctx.dirty;
    absorb(world, ctx);
    (fired, dirty)
}
