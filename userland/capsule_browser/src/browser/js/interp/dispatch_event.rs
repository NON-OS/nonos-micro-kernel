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

use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use crate::browser::dom::Dom;
use crate::browser::js::value::Value;
use crate::browser::js::world::World;

use super::absorb::absorb;
use super::apply::apply;
use super::ctx::Ctx;
use super::obj::obj;

const EVENT_BUDGET: u64 = 1_000_000;

// Bubble an event from `target` to the root, running every matching
// listener. Returns (any listener ran, DOM went dirty).
pub fn dispatch_event(
    dom: &mut Dom,
    world: &mut World,
    target: usize,
    event: &str,
) -> (bool, bool) {
    let mut ctx = Ctx::new(dom, EVENT_BUDGET);
    let mut fired = false;
    let mut cur = target;
    let mut hops = 0u32;
    loop {
        let cbs: Vec<Value> = world
            .listeners
            .iter()
            .filter(|(n, e, _)| *n == cur && e == event)
            .map(|(_, _, f)| f.clone())
            .collect();
        for cb in cbs {
            let ev = obj(&[
                ("target", Value::Node(target)),
                ("type", Value::Str(Rc::new(event.to_string()))),
                ("preventDefault", Value::Native("noop")),
                ("stopPropagation", Value::Native("noop")),
            ]);
            fired = true;
            let _ = apply(&mut ctx, &world.env, cb, vec![ev]);
            if ctx.steps >= ctx.budget {
                break;
            }
        }
        if cur == 0 || hops >= 512 {
            break;
        }
        cur = match ctx.dom.nodes.get(cur) {
            Some(n) => n.parent,
            None => break,
        };
        hops += 1;
    }
    let dirty = ctx.dirty;
    absorb(world, ctx);
    (fired, dirty)
}
