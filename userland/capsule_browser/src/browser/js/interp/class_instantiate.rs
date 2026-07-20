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
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::js::value::Value;

use super::call_func::call_func_this;
use super::class_ctor::find_ctor;
use super::class_methods::copy_methods;
use super::ctx::Ctx;

type Map = Rc<RefCell<BTreeMap<String, Value>>>;

// Create an instance of `cls`: bind its (and its ancestors') methods, then run
// the resolved constructor with the new object as `this`.
pub fn instantiate(ctx: &mut Ctx, cls: &Map, argv: Vec<Value>) -> Result<Value, ()> {
    let inst: Map = Rc::new(RefCell::new(BTreeMap::new()));
    copy_methods(cls, &inst);
    let this = Value::Object(inst);
    if let Value::Func(fd) = find_ctor(cls) {
        call_func_this(ctx, &fd, argv, this.clone())?;
    }
    Ok(this)
}
