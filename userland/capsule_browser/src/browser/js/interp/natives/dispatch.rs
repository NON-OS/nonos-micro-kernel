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

use crate::browser::js::value::Value;

use super::super::ctx::Ctx;
use super::builtins::builtin;
use super::console::console_log;
use super::document_get::get_by_id;
use super::document_query::query;
use super::math::math;

pub fn dispatch(ctx: &mut Ctx, name: &'static str, argv: Vec<Value>) -> Result<Value, ()> {
    match name {
        "console.log" => console_log(ctx, &argv),
        "document.getElementById" => Ok(get_by_id(ctx, &argv)),
        "document.querySelector" => Ok(query(ctx, &argv)),
        "Math.floor" | "Math.round" | "Math.abs" | "Math.max" | "Math.min" => Ok(math(name, &argv)),
        "parseInt" | "parseFloat" | "Number" | "String" | "Boolean" | "isNaN" => Ok(builtin(name, &argv)),
        "setTimeout" => {
            if let Some(cb) = argv.first() {
                if matches!(cb, Value::Func(_)) && ctx.timers.len() < 1024 {
                    ctx.timers.push(cb.clone());
                }
            }
            Ok(Value::Undef)
        }
        _ => Ok(Value::Undef),
    }
}
