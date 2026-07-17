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
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::js::env::Env;
use crate::browser::js::value::Value;

use super::array_iter::array_iter;
use super::array_mutate::array_mutate;
use super::array_query::array_query;
use super::array_reduce::array_reduce;
use super::array_sort::array_sort;
use super::ctx::Ctx;

// Dispatch an array method to the group that implements it.
pub(super) fn array_method(
    ctx: &mut Ctx,
    env: &Env,
    a: &Rc<RefCell<Vec<Value>>>,
    method: &str,
    argv: &[Value],
) -> Result<Value, ()> {
    match method {
        "push" | "pop" | "shift" | "unshift" | "reverse" | "fill" => {
            Ok(array_mutate(a, method, argv))
        }
        "join" | "indexOf" | "lastIndexOf" | "includes" | "slice" | "concat" | "flat" => {
            Ok(array_query(a, method, argv))
        }
        "forEach" | "map" | "filter" | "find" | "findIndex" | "some" | "every" => {
            array_iter(ctx, env, a, method, argv)
        }
        "reduce" | "reduceRight" => array_reduce(ctx, env, a, method, argv),
        "sort" => array_sort(ctx, env, a, argv),
        _ => Ok(Value::Undef),
    }
}
