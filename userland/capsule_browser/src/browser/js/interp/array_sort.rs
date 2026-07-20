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
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::apply::apply;
use super::ctx::Ctx;
use super::to_num::to_num;
use super::to_str::to_str;

// Array.prototype.sort with an optional comparator, defaulting to string order.
// Stable insertion sort, bounded by the step budget.
pub(super) fn array_sort(
    ctx: &mut Ctx,
    env: &Env,
    a: &Rc<RefCell<Vec<Value>>>,
    argv: &[Value],
) -> Result<Value, ()> {
    let cmp = argv.first().filter(|v| matches!(v, Value::Func(_))).cloned();
    let mut items = a.borrow().clone();
    for i in 1..items.len() {
        let mut j = i;
        while j > 0 && order(ctx, env, &cmp, &items[j - 1], &items[j])? > 0 {
            items.swap(j - 1, j);
            j -= 1;
        }
        if ctx.steps >= ctx.budget {
            return Err(());
        }
    }
    *a.borrow_mut() = items;
    Ok(Value::Array(a.clone()))
}

fn order(ctx: &mut Ctx, env: &Env, cmp: &Option<Value>, x: &Value, y: &Value) -> Result<i32, ()> {
    match cmp {
        Some(f) => {
            let r = to_num(&apply(ctx, env, f.clone(), vec![x.clone(), y.clone()])?);
            Ok(if r > 0.0 {
                1
            } else if r < 0.0 {
                -1
            } else {
                0
            })
        }
        None => Ok(to_str(x).cmp(&to_str(y)) as i32),
    }
}
