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
use alloc::string::ToString;
use core::cell::RefCell;

use crate::browser::js::ast::{ClassMethod, Expr};
use crate::browser::js::env::Env;
use crate::browser::js::value::{FuncData, Value};

use super::ctx::Ctx;
use super::eval_expr::eval_expr;

// Build the runtime object for a class declaration. Methods close over the
// declaring scope; the constructor is stored apart under "__ctor__" and the
// superclass object (if any) under "__super__" so instances inherit its methods.
pub fn build_class(
    ctx: &mut Ctx,
    env: &Env,
    sup: &Option<Expr>,
    methods: &[ClassMethod],
) -> Result<Value, ()> {
    let mut ctor = Value::Undef;
    let mmap = Rc::new(RefCell::new(BTreeMap::new()));
    for m in methods {
        let f = Value::Func(Rc::new(FuncData {
            params: m.params.clone(),
            body: m.body.clone(),
            env: env.clone(),
            is_async: m.is_async,
        }));
        if m.name == "constructor" {
            ctor = f;
        } else {
            mmap.borrow_mut().insert(m.name.clone(), f);
        }
    }
    let super_val = match sup {
        Some(e) => eval_expr(ctx, env, e)?,
        None => Value::Undef,
    };
    let mut fields = BTreeMap::new();
    fields.insert("__class__".to_string(), Value::Bool(true));
    fields.insert("__ctor__".to_string(), ctor);
    fields.insert("__methods__".to_string(), Value::Object(mmap));
    fields.insert("__super__".to_string(), super_val);
    Ok(Value::Object(Rc::new(RefCell::new(fields))))
}
