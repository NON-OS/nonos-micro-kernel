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

use super::super::ctx::Ctx;
use super::super::promise_make::{new_promise, promise_id};

// Promise.resolve(v) and Promise.reject(e). Resolving an existing promise
// returns it unchanged, matching the spec's pass-through behaviour.
pub(super) fn promise_static(ctx: &mut Ctx, name: &str, argv: &[Value]) -> Value {
    let v = argv.first().cloned().unwrap_or(Value::Undef);
    match name {
        "Promise.resolve" if promise_id(&v).is_some() => v,
        "Promise.resolve" => new_promise(ctx, 1, v),
        "Promise.reject" => new_promise(ctx, 2, v),
        _ => Value::Undef,
    }
}
