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

use super::attr::AttrTest;
use super::pseudo::Pseudo;

// One compound selector: tag.class#id[attr=v]:pseudo.
pub struct Simple {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attrs: Vec<(String, AttrTest)>,
    pub pseudo: Vec<Pseudo>,
}

impl Simple {
    pub fn empty() -> Self {
        Simple { tag: None, id: None, classes: Vec::new(), attrs: Vec::new(), pseudo: Vec::new() }
    }
}

// An ancestor constraint; `direct` requires the immediate parent (the `>`
// combinator) instead of any ancestor.
pub struct Step {
    pub simple: Simple,
    pub direct: bool,
}

pub struct Selector {
    pub key: Simple,
    pub ancestors: Vec<Step>,
    // 0 plain, 1 ::before, 2 ::after: the rule styles generated content on
    // the matched element rather than the element itself.
    pub element: u8,
}
