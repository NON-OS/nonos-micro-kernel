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

use alloc::vec;

use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::apply::apply;
use super::ctx::Ctx;
use super::promise_make::{new_promise, promise_id};

// `new Promise(executor)`: create a pending promise and run the executor with
// resolve/reject callbacks bound to it. An executor that settles synchronously
// leaves the promise fulfilled or rejected by the time construction returns.
pub fn promise_construct(ctx: &mut Ctx, env: &Env, argv: &[Value]) -> Result<Value, ()> {
    let promise = new_promise(ctx, 0, Value::Undef);
    let id = match promise_id(&promise) {
        Some(id) => id,
        None => return Ok(promise),
    };
    if let Some(exec) = argv.first().cloned() {
        if matches!(exec, Value::Func(_)) {
            let res = Value::Bound("__presolve", id);
            let rej = Value::Bound("__preject", id);
            apply(ctx, env, exec, vec![res, rej])?;
        }
    }
    Ok(promise)
}
