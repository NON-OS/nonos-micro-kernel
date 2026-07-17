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
use alloc::string::String;
use core::cell::RefCell;

use crate::browser::js::value::Value;

type Map = Rc<RefCell<BTreeMap<String, Value>>>;

// Copy a class's methods onto an instance, superclass first so a subclass
// method of the same name overrides the inherited one.
pub fn copy_methods(cls: &Map, inst: &Map) {
    if let Some(Value::Object(sup)) = cls.borrow().get("__super__").cloned() {
        if sup.borrow().contains_key("__class__") {
            copy_methods(&sup, inst);
        }
    }
    if let Some(Value::Object(m)) = cls.borrow().get("__methods__").cloned() {
        for (k, v) in m.borrow().iter() {
            inst.borrow_mut().insert(k.clone(), v.clone());
        }
    }
}
