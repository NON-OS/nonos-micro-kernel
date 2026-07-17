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
use super::promise_make::promise_id;

// `await expr`: if the value is a settled promise, unwrap its value; a pending
// promise yields undefined, and a non-promise passes through unchanged.
pub fn await_value(ctx: &Ctx, v: Value) -> Value {
    match promise_id(&v) {
        Some(id) => ctx.promises.get(id).map(|(_, val)| val.clone()).unwrap_or(Value::Undef),
        None => v,
    }
}
