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

extern crate alloc;

use alloc::{string::String, vec::Vec};

use super::entries::Entry;

pub struct State {
    pub owner_pid: u32,
    pub prefix: String,
    pub entries: Vec<Entry>,
    pub cursor: usize,
    pub preview: Option<String>,
    pub status: &'static [u8],
}

impl State {
    pub fn new() -> Self {
        State {
            owner_pid: 0,
            prefix: String::from("/"),
            entries: Vec::new(),
            cursor: 0,
            preview: None,
            status: b"loading...",
        }
    }
}
