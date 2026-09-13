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

//! The kernel's serial console, for included code that logs to it.
//!
//! The registry writes one line when its table fills. A host proof has no
//! console, and the line is not the behaviour under test, so it goes to the
//! test's own output where a failing run can still show it.

pub mod serial {
    pub fn println(line: &[u8]) {
        println!("{}", String::from_utf8_lossy(line));
    }
}
