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

use crate::browser::js::value::Value;

use super::equals::equals;

type Store = Rc<RefCell<Vec<Value>>>;

// The backing store array named by `key` ("__map__" or "__set__") on a value.
pub(super) fn store_of(recv: &Value, key: &str) -> Option<Store> {
    if let Value::Object(o) = recv {
        if let Some(Value::Array(a)) = o.borrow().get(key) {
            return Some(a.clone());
        }
    }
    None
}

// Index of the [key, value] pair whose key equals `key`.
pub(super) fn find_index(store: &Store, key: &Value) -> Option<usize> {
    store.borrow().iter().position(|e| match e {
        Value::Array(p) => p.borrow().first().map_or(false, |k| equals(k, key, true)),
        _ => false,
    })
}

// Insert or overwrite the value for `key`.
pub(super) fn set_entry(store: &Store, k: Value, v: Value) {
    if let Some(i) = find_index(store, &k) {
        if let Value::Array(p) = &store.borrow()[i] {
            if p.borrow().len() >= 2 {
                p.borrow_mut()[1] = v;
                return;
            }
        }
    }
    store.borrow_mut().push(Value::Array(Rc::new(RefCell::new(vec![k, v]))));
}

// Remove the pair for `key`, reporting whether one was present.
pub(super) fn delete_entry(store: &Store, key: &Value) -> bool {
    match find_index(store, key) {
        Some(i) => {
            store.borrow_mut().remove(i);
            true
        }
        None => false,
    }
}

// Collect column 0 (keys) or 1 (values) of every pair into a new array.
pub(super) fn column(store: &Store, col: usize) -> Value {
    let out: Vec<Value> = store
        .borrow()
        .iter()
        .filter_map(|e| match e {
            Value::Array(p) => p.borrow().get(col).cloned(),
            _ => None,
        })
        .collect();
    Value::Array(Rc::new(RefCell::new(out)))
}
