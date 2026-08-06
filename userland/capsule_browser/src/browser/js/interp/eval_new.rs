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

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::String;
use core::cell::RefCell;

use crate::browser::js::ast::Expr;
use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::call_func::call_func_this;
use super::class_instantiate::instantiate;
use super::ctx::Ctx;
use super::error_obj::{error_obj, is_err_name};
use super::eval_args::eval_args;
use super::eval_expr::eval_expr;
use super::map_obj::{map_obj, set_obj};
use super::promise_construct::promise_construct;
use super::regex_obj::regex_obj;
use super::to_str::to_str;

// `new Target(args)`: instantiate a class object, or invoke a constructor
// function with a fresh `this`. If the constructor returns an object, that
// object becomes the result, otherwise the fresh instance is returned.
pub fn eval_new(ctx: &mut Ctx, env: &Env, callee: &Expr, arg_exprs: &[Expr]) -> Result<Value, ()> {
    let target = eval_expr(ctx, env, callee)?;
    let argv = eval_args(ctx, env, arg_exprs)?;
    match &target {
        Value::Object(cls) if cls.borrow().contains_key("__class__") => instantiate(ctx, cls, argv),
        Value::Object(o) if native_ctor(o) == Some("Promise") => promise_construct(ctx, env, &argv),
        Value::Object(o) if native_ctor(o) == Some("RegExp") => {
            let src = argv.first().map(to_str).unwrap_or_default();
            let flags = argv.get(1).map(to_str).unwrap_or_default();
            Ok(regex_obj(src, flags))
        }
        Value::Object(o) if native_ctor(o).map_or(false, is_err_name) => {
            let name = native_ctor(o).unwrap_or("Error");
            let msg = argv.first().map(to_str).unwrap_or_default();
            Ok(error_obj(name, msg))
        }
        Value::Object(o) if native_ctor(o) == Some("Map") => Ok(map_obj(&argv)),
        Value::Object(o) if native_ctor(o) == Some("Set") => Ok(set_obj(&argv)),
        Value::Func(fd) => {
            let this = Value::Object(Rc::new(RefCell::new(BTreeMap::new())));
            match call_func_this(ctx, fd, argv, this.clone())? {
                obj @ Value::Object(_) => Ok(obj),
                _ => Ok(this),
            }
        }
        _ => Ok(Value::Undef),
    }
}

// The native constructor tag on a builtin object like Promise, if present.
fn native_ctor(o: &Rc<RefCell<BTreeMap<String, Value>>>) -> Option<&'static str> {
    match o.borrow().get("__native_ctor__") {
        Some(Value::Str(s)) => match s.as_str() {
            "Promise" => Some("Promise"),
            "RegExp" => Some("RegExp"),
            "Error" => Some("Error"),
            "TypeError" => Some("TypeError"),
            "RangeError" => Some("RangeError"),
            "SyntaxError" => Some("SyntaxError"),
            "ReferenceError" => Some("ReferenceError"),
            "Map" => Some("Map"),
            "Set" => Some("Set"),
            _ => None,
        },
        _ => None,
    }
}
