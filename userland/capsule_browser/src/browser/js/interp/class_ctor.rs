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

// The constructor to run for an instance: this class's own if defined,
// otherwise the nearest ancestor's, mirroring an implicit derived constructor.
pub fn find_ctor(cls: &Map) -> Value {
    if let Some(c @ Value::Func(_)) = cls.borrow().get("__ctor__").cloned() {
        return c;
    }
    if let Some(Value::Object(sup)) = cls.borrow().get("__super__").cloned() {
        if sup.borrow().contains_key("__class__") {
            return find_ctor(&sup);
        }
    }
    Value::Undef
}
