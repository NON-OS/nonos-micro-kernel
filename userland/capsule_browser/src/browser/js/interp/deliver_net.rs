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
use alloc::string::String;
use alloc::vec;

use crate::browser::dom::Dom;
use crate::browser::js::value::Value;
use crate::browser::js::world::World;

use super::absorb::absorb;
use super::apply::apply;
use super::ctx::Ctx;
use super::natives::json_of;
use super::obj::obj;

const NET_BUDGET: u64 = 1_000_000;

// Hand a completed script request to its callback: status, body text, and
// the parsed json when the body carries any. Returns whether the DOM went
// dirty and needs a relayout.
pub fn deliver_net(dom: &mut Dom, world: &mut World, cb: Value, status: u16, body: String) -> bool {
    let mut ctx = Ctx::new(dom, NET_BUDGET);
    let json = json_of(&body);
    let resp = obj(&[
        ("status", Value::Num(status as f64)),
        ("ok", Value::Bool((200..300).contains(&status))),
        ("body", Value::Str(Rc::new(body))),
        ("json", json),
    ]);
    let _ = apply(&mut ctx, &world.env, cb, vec![resp]);
    let dirty = ctx.dirty;
    absorb(world, ctx);
    dirty
}
