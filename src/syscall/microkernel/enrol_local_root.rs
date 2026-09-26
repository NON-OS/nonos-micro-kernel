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

//! `MkDevRootLocal`: consent to run what this machine builds and fetches.

use crate::capabilities::caps_to_bits;
use crate::security::dev_roots::request_local_build_root;
use crate::syscall::caps::current_caps_or_default;

/// Ask to enrol *this machine's own* build root, which is the only root a
/// local install can ever be proved under.
pub fn sys_dev_root_local() -> i64 {
    let caps = caps_to_bits(&current_caps_or_default().permissions);
    match request_local_build_root(caps) {
        Ok(()) => 0,
        Err(e) => {
            crate::sys::serial::print(b"[DEV-ROOT] local request refused: ");
            crate::sys::serial::println(e.as_str().as_bytes());
            e.to_errno()
        }
    }
}
