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

use alloc::rc::Rc;

use crate::browser::js::ast::Stmt;
use crate::browser::js::env::Env;
use crate::browser::js::value::{FuncData, Value};

pub fn hoist(env: &Env, stmts: &[Stmt]) {
    for s in stmts {
        if let Stmt::Func(name, params, body, is_async) = s {
            let fd = FuncData {
                params: params.clone(),
                body: body.clone(),
                env: env.clone(),
                is_async: *is_async,
            };
            env.define(name, Value::Func(Rc::new(fd)));
        }
    }
}
