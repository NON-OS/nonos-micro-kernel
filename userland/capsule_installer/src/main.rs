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

#![no_std]
#![no_main]

extern crate alloc;

mod protocol;
mod server;

use nonos_libc::{heap_init_sized, mk_exit};

// A load holds the 8 MiB request buffer, the artifact itself, and the transient
// copy the read's grow-and-copy leaves live at the same time, so peak use is
// roughly 8 + 2.5x the artifact. The 16 MiB libc default cannot even hold a
// 4 MiB capsule, and the failure is a silent OOM abort with no reply; size the
// heap against MAX_ARTIFACT (16 MiB) instead.
const INSTALLER_HEAP: usize = 64 * 1024 * 1024;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init_sized(INSTALLER_HEAP).is_err() {
        mk_exit(1);
    }
    server::run();
}
