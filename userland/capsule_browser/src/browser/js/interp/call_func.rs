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

use crate::browser::js::value::{FuncData, Value};

use super::ctx::Ctx;
use super::exec::exec;
use super::flow::Flow;
use super::promise_make::{new_promise, promise_id};

// Invoke `fd` with `this_val` bound as `this` in its scope, so method bodies and
// constructors can read and mutate the receiver.
pub fn call_func_this(
    ctx: &mut Ctx,
    fd: &FuncData,
    argv: Vec<Value>,
    this_val: Value,
) -> Result<Value, ()> {
    ctx.depth += 1;
    if ctx.depth > 400 {
        ctx.depth -= 1;
        return Err(());
    }
    let scope = fd.env.child();
    scope.define("this", this_val);
    for (i, p) in fd.params.iter().enumerate() {
        scope.define(p, argv.get(i).cloned().unwrap_or(Value::Undef));
    }
    let result = exec(ctx, &scope, &fd.body);
    ctx.depth -= 1;
    if fd.is_async {
        return async_result(ctx, result);
    }
    let value = match result? {
        Flow::Return(v) => v,
        _ => Value::Undef,
    };
    Ok(value)
}

// Settle an async function's completion into a promise: a normal return
// fulfils it, a thrown exception rejects it, a hard abort propagates.
fn async_result(ctx: &mut Ctx, result: Result<Flow, ()>) -> Result<Value, ()> {
    match result {
        Ok(flow) => {
            let v = match flow {
                Flow::Return(v) => v,
                _ => Value::Undef,
            };
            if promise_id(&v).is_some() {
                Ok(v)
            } else {
                Ok(new_promise(ctx, 1, v))
            }
        }
        Err(()) => match ctx.exception.take() {
            Some(exc) => Ok(new_promise(ctx, 2, exc)),
            None => Err(()),
        },
    }
}
