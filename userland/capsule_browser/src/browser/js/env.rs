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
use alloc::string::{String, ToString};
use core::cell::RefCell;

use super::value::Value;

pub struct Scope {
    pub vars: BTreeMap<String, Value>,
    pub parent: Option<Env>,
}

#[derive(Clone)]
pub struct Env(pub Rc<RefCell<Scope>>);

impl Env {
    pub fn root() -> Env {
        Env(Rc::new(RefCell::new(Scope { vars: BTreeMap::new(), parent: None })))
    }
    pub fn child(&self) -> Env {
        Env(Rc::new(RefCell::new(Scope { vars: BTreeMap::new(), parent: Some(self.clone()) })))
    }
    pub fn define(&self, name: &str, v: Value) {
        self.0.borrow_mut().vars.insert(name.to_string(), v);
    }
    pub fn get(&self, name: &str) -> Option<Value> {
        let s = self.0.borrow();
        if let Some(v) = s.vars.get(name) {
            return Some(v.clone());
        }
        match &s.parent {
            Some(p) => p.get(name),
            None => None,
        }
    }
    pub fn set(&self, name: &str, v: Value) {
        let mut s = self.0.borrow_mut();
        if s.vars.contains_key(name) {
            s.vars.insert(name.to_string(), v);
            return;
        }
        match &s.parent {
            Some(p) => p.set(name, v),
            None => {
                s.vars.insert(name.to_string(), v);
            }
        }
    }
}
