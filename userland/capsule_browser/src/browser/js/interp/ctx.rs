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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::dom::Dom;
use crate::browser::js::value::Value;

pub struct Ctx<'a> {
    pub dom: &'a mut Dom,
    pub out: String,
    pub steps: u64,
    pub budget: u64,
    pub depth: u32,
    pub dirty: bool,
    pub timers: Vec<Value>,
}

impl<'a> Ctx<'a> {
    pub fn new(dom: &'a mut Dom, budget: u64) -> Self {
        Ctx { dom, out: String::new(), steps: 0, budget, depth: 0, dirty: false, timers: Vec::new() }
    }
    pub fn tick(&mut self) -> bool {
        self.steps += 1;
        self.steps <= self.budget
    }
}
