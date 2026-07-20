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

// The close IPCs a released backing dispatches to. Best-effort by design: the
// descriptor is already gone from the table when these run, and the service
// reclaims the handle on process exit regardless, so a lost reply only leaks
// until then. The wire framing mirrors the net and fs transports; it is
// duplicated here because those live behind private platform modules and this
// close path must stay reachable from `OwnedFd::drop`.

use super::table::Backing;

const SK_NAME: &[u8] = b"net.sockets";
const SK_MAGIC: u32 = 0x4E53_4B54;
const SK_OP_CLOSE: u16 = 9;
const SK_TIMEOUT_MS: u64 = 2000;

const VFS_MAGIC: u32 = 0x4E4F_5646;
const VFS_OP_CLOSE: u16 = 2;

// Both protocols use a 20-byte request header: magic, version, op, 8 reserved
// bytes, body length.
const HDR: usize = 20;

pub(super) fn close_backing(backing: Backing) {
    match backing {
        Backing::Socket { handle } => close_socket(handle),
        Backing::File { port, pid, handle } => close_file(port, pid, handle),
    }
}

fn header(magic: u32, op: u16, body_len: u32, frame: &mut [u8]) {
    frame[0..4].copy_from_slice(&magic.to_le_bytes());
    frame[4..6].copy_from_slice(&1u16.to_le_bytes());
    frame[6..8].copy_from_slice(&op.to_le_bytes());
    frame[16..20].copy_from_slice(&body_len.to_le_bytes());
}

// net.sockets OP_CLOSE on `handle`, under a deadline so a wedged service
// cannot hang a drop path.
fn close_socket(handle: u32) {
    let mut port = 0u32;
    let mut owner = 0u32;
    let rc = unsafe {
        sys5(
            tag4(b"MSVL"),
            SK_NAME.as_ptr() as u64,
            SK_NAME.len() as u64,
            &mut port as *mut u32 as u64,
            &mut owner as *mut u32 as u64,
            0,
        )
    };
    if rc != 0 {
        return;
    }
    let mut tx = [0u8; HDR + 4];
    header(SK_MAGIC, SK_OP_CLOSE, 4, &mut tx);
    tx[HDR..HDR + 4].copy_from_slice(&handle.to_le_bytes());
    let mut rx = [0u8; HDR];
    let _ = unsafe {
        sys6(
            tag4(b"MICL"),
            port as u64,
            tx.as_ptr() as u64,
            tx.len() as u64,
            rx.as_mut_ptr() as u64,
            rx.len() as u64,
            SK_TIMEOUT_MS,
        )
    };
}

// vfs OP_CLOSE on `handle`, addressed to the port captured at open so the
// close reaches the same pool worker that owns the handle.
fn close_file(port: u32, pid: u32, handle: u32) {
    let mut tx = [0u8; HDR + 8];
    header(VFS_MAGIC, VFS_OP_CLOSE, 8, &mut tx);
    tx[HDR..HDR + 4].copy_from_slice(&pid.to_le_bytes());
    tx[HDR + 4..HDR + 8].copy_from_slice(&handle.to_le_bytes());
    let mut rx = [0u8; HDR + 4];
    let _ = unsafe {
        sys5(
            tag4(b"MICL"),
            port as u64,
            tx.as_ptr() as u64,
            tx.len() as u64,
            rx.as_mut_ptr() as u64,
            rx.len() as u64,
        )
    };
}

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

unsafe fn sys5(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> i64 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
            in("rdx") a3, in("r10") a4, in("r8") a5, out("rcx") _, out("r11") _);
    }
    r
}

unsafe fn sys6(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> i64 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
            in("rdx") a3, in("r10") a4, in("r8") a5, in("r9") a6, out("rcx") _, out("r11") _);
    }
    r
}
