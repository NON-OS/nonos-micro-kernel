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

// Object.keys / values / entries / assign. Internal marker keys (prefixed with
// "__") are hidden so engine bookkeeping never leaks into script iteration.
pub(super) fn object_static(name: &str, argv: &[Value]) -> Value {
    let first = argv.first().cloned().unwrap_or(Value::Undef);
    match name {
        "Object.keys" => collect(&first, |k, _| Value::Str(Rc::new(k.clone()))),
        "Object.values" => collect(&first, |_, v| v.clone()),
        "Object.entries" => collect(&first, |k, v| {
            let pair = alloc::vec![Value::Str(Rc::new(k.clone())), v.clone()];
            Value::Array(Rc::new(RefCell::new(pair)))
        }),
        "Object.assign" => assign(argv),
        _ => Value::Undef,
    }
}

fn collect(v: &Value, f: impl Fn(&alloc::string::String, &Value) -> Value) -> Value {
    let mut out = Vec::new();
    if let Value::Object(m) = v {
        for (k, val) in m.borrow().iter() {
            if !k.starts_with("__") {
                out.push(f(k, val));
            }
        }
    }
    Value::Array(Rc::new(RefCell::new(out)))
}

fn assign(argv: &[Value]) -> Value {
    let target = argv.first().cloned().unwrap_or(Value::Undef);
    if let Value::Object(t) = &target {
        for src in argv.iter().skip(1) {
            if let Value::Object(s) = src {
                for (k, v) in s.borrow().iter() {
                    t.borrow_mut().insert(k.clone(), v.clone());
                }
            }
        }
    }
    target
}
