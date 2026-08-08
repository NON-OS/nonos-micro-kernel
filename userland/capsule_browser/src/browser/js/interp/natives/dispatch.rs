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

use super::super::ctx::{Ctx, TimerReq};
use super::super::to_str::to_str;
use super::builtins::builtin;
use super::console::console_log;
use super::document_create::{create_element, create_fragment, create_text_node};
use super::document_get::get_by_id;
use super::document_query::query;
use super::document_query_all::query_all;
use super::js_fetch::js_fetch;
use super::json_parse::json_parse;
use super::json_stringify::json_stringify;
use super::math::math;
use super::object_static::object_static;
use super::promise_static::promise_static;
use super::timer_ms::timer_ms;

pub fn dispatch(ctx: &mut Ctx, name: &'static str, argv: Vec<Value>) -> Result<Value, ()> {
    match name {
        "console.log" => console_log(ctx, &argv),
        "document.getElementById" => Ok(get_by_id(ctx, &argv)),
        "document.querySelector" => Ok(query(ctx, &argv)),
        "document.querySelectorAll" => Ok(query_all(ctx, &argv)),
        "document.createElement" => Ok(create_element(ctx, &argv)),
        "document.createTextNode" => Ok(create_text_node(ctx, &argv)),
        "document.createDocumentFragment" => Ok(create_fragment(ctx)),
        "Math.floor" | "Math.round" | "Math.abs" | "Math.max" | "Math.min" => Ok(math(name, &argv)),
        "parseInt" | "parseFloat" | "Number" | "String" | "Boolean" | "isNaN" => {
            Ok(builtin(name, &argv))
        }
        "fetch" => Ok(js_fetch(ctx, &argv)),
        "Promise.resolve" | "Promise.reject" => Ok(promise_static(ctx, name, &argv)),
        "Object.keys" | "Object.values" | "Object.entries" | "Object.assign" => {
            Ok(object_static(name, &argv))
        }
        "JSON.parse" => Ok(argv.first().map(|v| json_parse(&to_str(v))).unwrap_or(Value::Undef)),
        "JSON.stringify" => {
            let mut out = alloc::string::String::new();
            if let Some(v) = argv.first() {
                json_stringify(v, &mut out, 0);
            }
            Ok(Value::Str(alloc::rc::Rc::new(out)))
        }
        "setTimeout" | "setInterval" => {
            if let Some(cb) = argv.first() {
                if matches!(cb, Value::Func(_)) && ctx.timers.len() < 256 {
                    let ms = argv.get(1).map(timer_ms).unwrap_or(0);
                    ctx.timers.push(TimerReq { cb: cb.clone(), ms, repeat: name == "setInterval" });
                }
            }
            Ok(Value::Undef)
        }
        _ => Ok(Value::Undef),
    }
}
