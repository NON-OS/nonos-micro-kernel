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

use nonos_abi::{syscall, N_DEBUG};

const MAX_LINE: usize = 256;

pub fn log(msg: &[u8]) -> i64 {
    if msg.is_empty() {
        return -22;
    }
    let len = if msg.len() > MAX_LINE { MAX_LINE } else { msg.len() };
    syscall(N_DEBUG, [msg.as_ptr() as u64, len as u64, 0, 0, 0, 0])
}
