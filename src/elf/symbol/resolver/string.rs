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

pub(super) unsafe fn read_symbol_name(ptr: *const u8, max_len: usize) -> String {
    let mut name = String::new();
    for i in 0..max_len.min(256) {
        let c = unsafe { *ptr.add(i) };
        if c == 0 {
            break;
        }
        name.push(c as char);
    }
    name
}
