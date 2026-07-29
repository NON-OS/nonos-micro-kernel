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

//! Where the PCI bridge put its I/O window, and how a port number reaches it.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::sys::serial;

static CPU_BASE: AtomicU64 = AtomicU64::new(0);
static PORT_BASE: AtomicU64 = AtomicU64::new(0);
static SIZE: AtomicU64 = AtomicU64::new(0);
static WARNED: AtomicBool = AtomicBool::new(false);

/// Record the window the device tree described.
///
/// `cpu_base` is where it lands in physical memory, `port_base` is the port
/// number that address corresponds to, and `size` is its extent. A `size` of
/// zero means the board has no I/O space, which is left as the default.
pub fn set_io_window(cpu_base: u64, port_base: u64, size: u64) {
    CPU_BASE.store(cpu_base, Ordering::Relaxed);
    PORT_BASE.store(port_base, Ordering::Relaxed);
    SIZE.store(size, Ordering::Release);
}

/// The address `port` maps to, or `None` when this board cannot reach it.
pub(super) fn address_of(port: u16) -> Option<u64> {
    let size = SIZE.load(Ordering::Acquire);
    let port_base = PORT_BASE.load(Ordering::Relaxed);
    let offset = (port as u64).checked_sub(port_base)?;
    if size == 0 || offset >= size {
        report_unreachable();
        return None;
    }
    Some(CPU_BASE.load(Ordering::Relaxed) + offset)
}

/// Say it once. A driver that pokes an absent port usually does so in a loop,
/// and a log line per iteration would bury the boot.
fn report_unreachable() {
    if !WARNED.swap(true, Ordering::Relaxed) {
        serial::println(b"[IO] port access with no PCI I/O window; reads answer all-ones");
    }
}
