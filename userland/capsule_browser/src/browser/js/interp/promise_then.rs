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

// `promise.then(onFulfilled, onRejected)` / `promise.catch(onRejected)`. The
// matching handler for the settled state runs now and its result becomes the
// next promise in the chain; an unmatched state passes through unchanged.
pub fn promise_then(
    ctx: &mut Ctx,
    env: &Env,
    id: usize,
    method: &str,
    argv: &[Value],
) -> Result<Value, ()> {
    let (state, val) = ctx.promises.get(id).cloned().unwrap_or((0, Value::Undef));
    let on_rejected = if method == "catch" { argv.first().cloned() } else { argv.get(1).cloned() };
    let cb = match state {
        1 if method == "then" => argv.first().cloned(),
        2 => on_rejected,
        _ => None,
    };
    if let Some(f @ Value::Func(_)) = cb {
        let r = apply(ctx, env, f, vec![val])?;
        return Ok(chain(ctx, r));
    }
    Ok(new_promise(ctx, state, val))
}

fn chain(ctx: &mut Ctx, r: Value) -> Value {
    if promise_id(&r).is_some() {
        r
    } else {
        new_promise(ctx, 1, r)
    }
}
