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

// reduce / reduceRight. Without a seed the first element seeds the accumulator;
// the callback receives (acc, item, originalIndex).
pub(super) fn array_reduce(
    ctx: &mut Ctx,
    env: &Env,
    a: &Rc<RefCell<Vec<Value>>>,
    method: &str,
    argv: &[Value],
) -> Result<Value, ()> {
    let cb = match argv.first() {
        Some(f @ Value::Func(_)) => f.clone(),
        _ => return Ok(Value::Undef),
    };
    let len = a.borrow().len();
    let right = method == "reduceRight";
    let order: Vec<usize> = if right { (0..len).rev().collect() } else { (0..len).collect() };
    let mut seq = order.into_iter();
    let mut acc = match argv.get(1) {
        Some(v) => v.clone(),
        None => match seq.next() {
            Some(i) => a.borrow()[i].clone(),
            None => return Ok(Value::Undef),
        },
    };
    for i in seq {
        let item = a.borrow()[i].clone();
        acc = apply(ctx, env, cb.clone(), vec![acc, item, Value::Num(i as f64)])?;
        if ctx.steps >= ctx.budget {
            return Err(());
        }
    }
    Ok(acc)
}
