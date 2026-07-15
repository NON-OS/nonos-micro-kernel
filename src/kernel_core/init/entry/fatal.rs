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

use crate::sys::boot_log;

pub(super) fn fatal(stage: &str, detail: &str) -> ! {
    boot_log::error(stage);
    crate::sys::serial::print(b"[FATAL] ");
    crate::sys::serial::print_str(stage);
    crate::sys::serial::print(b": ");
    crate::sys::serial::println(detail.as_bytes());
    crate::arch::halt_loop()
}
