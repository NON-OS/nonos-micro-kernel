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

//! The orders the table can be put in. Rates sort like shares: busiest first.

#[derive(Clone, Copy, PartialEq)]
pub enum Sort {
    Cpu,
    Mem,
    Ipc,
    Sysc,
    Name,
    Pid,
}

impl Sort {
    pub fn label(self) -> &'static [u8] {
        match self {
            Sort::Cpu => b"cpu",
            Sort::Mem => b"memory",
            Sort::Ipc => b"ipc rate",
            Sort::Sysc => b"syscall rate",
            Sort::Name => b"name",
            Sort::Pid => b"pid",
        }
    }
}
