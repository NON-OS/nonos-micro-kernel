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

use crate::browser::js::value::Value;

use super::array_util::MAX_ARRAY;

// In-place array methods: push/pop/shift/unshift/reverse/fill.
pub(super) fn array_mutate(a: &Rc<RefCell<Vec<Value>>>, method: &str, argv: &[Value]) -> Value {
    let mut b = a.borrow_mut();
    match method {
        "push" => {
            for v in argv {
                if b.len() >= MAX_ARRAY {
                    break;
                }
                b.push(v.clone());
            }
            Value::Num(b.len() as f64)
        }
        "pop" => b.pop().unwrap_or(Value::Undef),
        "shift" => {
            if b.is_empty() {
                Value::Undef
            } else {
                b.remove(0)
            }
        }
        "unshift" => {
            for (k, v) in argv.iter().enumerate() {
                if b.len() >= MAX_ARRAY {
                    break;
                }
                b.insert(k, v.clone());
            }
            Value::Num(b.len() as f64)
        }
        "reverse" => {
            b.reverse();
            drop(b);
            Value::Array(a.clone())
        }
        "fill" => {
            let val = argv.first().cloned().unwrap_or(Value::Undef);
            for x in b.iter_mut() {
                *x = val.clone();
            }
            drop(b);
            Value::Array(a.clone())
        }
        _ => Value::Undef,
    }
}
