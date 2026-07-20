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

use crate::browser::js::value::Value;

use super::ctx::Ctx;
use super::obj::obj;

const MAX_PROMISES: usize = 8192;

// Register a promise with the given state (0 pending, 1 fulfilled, 2 rejected)
// and settled value, returning the object handle scripts hold onto.
pub fn new_promise(ctx: &mut Ctx, state: u8, value: Value) -> Value {
    if ctx.promises.len() >= MAX_PROMISES {
        return value;
    }
    let id = ctx.promises.len();
    ctx.promises.push((state, value));
    obj(&[("__promise_id__", Value::Num(id as f64))])
}

// The promise id carried by a value, if it is a promise handle.
pub fn promise_id(v: &Value) -> Option<usize> {
    if let Value::Object(m) = v {
        if let Some(Value::Num(n)) = m.borrow().get("__promise_id__") {
            return Some(*n as usize);
        }
    }
    None
}
