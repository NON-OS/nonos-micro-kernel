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

use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::call_func::call_func_this;
use super::ctx::Ctx;
use super::natives::dispatch;

pub fn apply(ctx: &mut Ctx, env: &Env, f: Value, argv: Vec<Value>) -> Result<Value, ()> {
    apply_this(ctx, env, f, argv, Value::Undef)
}

// Invoke `f` with an explicit receiver bound as `this` (for method calls).
pub fn apply_this(
    ctx: &mut Ctx,
    env: &Env,
    f: Value,
    argv: Vec<Value>,
    this_val: Value,
) -> Result<Value, ()> {
    let _ = env;
    match f {
        Value::Func(fd) => call_func_this(ctx, &fd, argv, this_val),
        Value::Native(name) => dispatch(ctx, name, argv),
        Value::Bound(kind @ ("__presolve" | "__preject"), id) => {
            let state = if kind == "__presolve" { 1 } else { 2 };
            if let Some(slot) = ctx.promises.get_mut(id) {
                if slot.0 == 0 {
                    *slot = (state, argv.into_iter().next().unwrap_or(Value::Undef));
                }
            }
            Ok(Value::Undef)
        }
        _ => Ok(Value::Undef),
    }
}
