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

pub fn call_func(ctx: &mut Ctx, fd: &FuncData, argv: Vec<Value>) -> Result<Value, ()> {
    ctx.depth += 1;
    if ctx.depth > 400 {
        ctx.depth -= 1;
        return Err(());
    }
    let scope = fd.env.child();
    for (i, p) in fd.params.iter().enumerate() {
        scope.define(p, argv.get(i).cloned().unwrap_or(Value::Undef));
    }
    let result = exec(ctx, &scope, &fd.body);
    ctx.depth -= 1;
    match result? {
        Flow::Return(v) => Ok(v),
        _ => Ok(Value::Undef),
    }
}
