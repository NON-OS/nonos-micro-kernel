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

use nonos_abi::{syscall, N_SERVICE_LOOKUP};

pub fn lookup(name: &[u8]) -> Option<u32> {
    let mut port: u32 = 0;
    let mut pid: u32 = 0;
    let rc = syscall(
        N_SERVICE_LOOKUP,
        [
            name.as_ptr() as u64,
            name.len() as u64,
            &mut port as *mut u32 as u64,
            &mut pid as *mut u32 as u64,
            0,
            0,
        ],
    );
    if rc < 0 || pid == 0 || port == 0 {
        return None;
    }
    Some(port)
}
