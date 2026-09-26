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

//! Package installs asked for by a capsule, performed by init.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use spin::Mutex;

/// Deep enough for a person clicking faster than a download completes,
/// shallow enough that a caller in a loop cannot grow it without bound.
const DEPTH: usize = 8;

static PENDING: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Record a request. False when the queue is full, which the caller
/// reports as busy rather than silently dropping.
pub(crate) fn request(package: String) -> bool {
    let mut q = PENDING.lock();
    if q.len() >= DEPTH || q.contains(&package) {
        return false;
    }
    q.push(package);
    true
}

/// Perform every queued install.
pub(crate) fn service() {
    let taken: Vec<String> = core::mem::take(&mut *PENDING.lock());
    for package in taken {
        match crate::userspace::capsule_linux::spawn_install(&package) {
            Ok(pid) => {
                crate::sys::serial::print(b"[LINUX-INSTALL] started pid=");
                crate::sys::serial::print_hex(pid as u64);
                crate::sys::serial::print(b" ");
                crate::sys::serial::println(package.as_bytes());
            }
            Err(_) => {
                crate::sys::serial::print(b"[LINUX-INSTALL] refused ");
                crate::sys::serial::println(package.as_bytes());
            }
        }
    }
}
