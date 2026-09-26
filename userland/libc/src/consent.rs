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

//! Consent to run what this machine builds and fetches.

use crate::syscall::{call_raw, N_MK_DEV_ROOT_CONFIRM, N_MK_DEV_ROOT_LOCAL};

/// Ask to enrol this machine's own build root, so what it installs can be
/// proved.
pub fn mk_dev_root_local() -> i64 {
    call_raw(N_MK_DEV_ROOT_LOCAL, [0, 0, 0, 0, 0, 0])
}

/// Complete the pending enrolment with the code the user read and typed.
pub fn mk_dev_root_confirm(code: u32) -> i64 {
    call_raw(N_MK_DEV_ROOT_CONFIRM, [code as u64, 0, 0, 0, 0, 0])
}
