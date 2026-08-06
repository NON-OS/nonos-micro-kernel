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

use super::map_ops::{set_entry, store_of};
use super::obj::obj;

// `new Map(iterable)`: an object with a "__map__" pair store, seeded from an
// array of [key, value] entries.
pub fn map_obj(argv: &[Value]) -> Value {
    let m = obj(&[("__map__", Value::Array(Rc::new(RefCell::new(Vec::new()))))]);
    if let (Some(store), Some(Value::Array(init))) = (store_of(&m, "__map__"), argv.first()) {
        for e in init.borrow().iter() {
            if let Value::Array(p) = e {
                let b = p.borrow();
                set_entry(
                    &store,
                    b.first().cloned().unwrap_or(Value::Undef),
                    b.get(1).cloned().unwrap_or(Value::Undef),
                );
            }
        }
    }
    m
}

// `new Set(iterable)`: an object with a "__set__" value store, seeded from an
// array while dropping duplicates.
pub fn set_obj(argv: &[Value]) -> Value {
    let s = obj(&[("__set__", Value::Array(Rc::new(RefCell::new(Vec::new()))))]);
    if let (Some(store), Some(Value::Array(init))) = (store_of(&s, "__set__"), argv.first()) {
        for v in init.borrow().iter() {
            set_add(&store, v.clone());
        }
    }
    s
}

// Append `v` to a set store if no equal element is present.
pub(super) fn set_add(store: &Rc<RefCell<Vec<Value>>>, v: Value) {
    use super::equals::equals;
    let present = store.borrow().iter().any(|e| equals(e, &v, true));
    if !present {
        store.borrow_mut().push(v);
    }
}
