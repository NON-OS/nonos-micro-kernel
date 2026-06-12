// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::log::logger::{log_error, log_info};
use alloc::format;

pub fn verify_kernel_version(embedded_version: u32, minimum_version: u32) -> bool {
    if embedded_version < minimum_version {
        log_error(
            "enforce",
            &format!("version {} < minimum {}", embedded_version, minimum_version),
        );
        return false;
    }
    log_info("enforce", &format!("version {} accepted", embedded_version));
    true
}
