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

use alloc::string::String;

#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    pub name: String,
    pub port: u32,
    pub pid: u32,
    pub caps_required: u64,
}

impl ServiceEndpoint {
    pub fn new(name: &str, port: u32, pid: u32, caps_required: u64) -> Self {
        Self { name: String::from(name), port, pid, caps_required }
    }
}
