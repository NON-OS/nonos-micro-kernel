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

use alloc::vec::Vec;

use crate::kernel_core::surface_registry::{
    attach_map, attach_surface, lookup_attached_va, lookup_owned, register_surface,
    release_surface, share_surface, wait_for_vsync, SurfaceDescriptor,
};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::api::translate_address;
use crate::process::current_pid;
use crate::syscall::dispatch::util::errno;
use crate::syscall::SyscallResult;
use crate::usercopy::{read_user_value, validate_user_write, write_user_value};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use super::surface_ops::{map_err, EFAULT, EINVAL, EPERM, ESRCH};

static VSYNC_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);
static FIRST_SURFACE_REGISTER: AtomicBool = AtomicBool::new(false);
static FIRST_SURFACE_PRESENT: AtomicBool = AtomicBool::new(false);
static DISPLAY_MODE_LOGGED: AtomicBool = AtomicBool::new(false);

// The first registered surface is the GPU driver's scanout, so its geometry is
// the real display mode. Log it once so every boot proves what resolution the
// machine actually came up in.
fn log_display_mode(desc: &SurfaceDescriptor) {
    if DISPLAY_MODE_LOGGED.swap(true, Ordering::Relaxed) {
        return;
    }
    let mut buf = [0u8; 24];
    let mut n = 0;
    n += fmt_u32(desc.width, &mut buf[n..]);
    buf[n] = b'x';
    n += 1;
    n += fmt_u32(desc.height, &mut buf[n..]);
    // serial::print, not trace: the mode must land in every boot log, not only
    // debug-enabled ones, so a wrong resolution is visible immediately.
    crate::sys::serial::print(b"[DISPLAY] mode ");
    crate::sys::serial::println(&buf[..n]);
}

fn fmt_u32(mut v: u32, out: &mut [u8]) -> usize {
    let mut tmp = [0u8; 10];
    let mut k = 0;
    loop {
        tmp[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 || k == 10 {
            break;
        }
    }
    for i in 0..k {
        out[i] = tmp[k - 1 - i];
    }
    k
}

fn trace_surface(op: &[u8], label: &[u8], pid: u32) {
    if !matches!(pid, 0x17 | 0x26 | 0x27) || VSYNC_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 80
    {
        return;
    }
    crate::sys::serial::trace(b"[SURFACE] ");
    crate::sys::serial::trace(op);
    crate::sys::serial::trace(b" ");
    crate::sys::serial::traceln(label);
}

pub(super) fn do_register(desc_ptr: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    trace_surface(b"register", b"enter", pid);
    let desc: SurfaceDescriptor = match read_user_value(desc_ptr) {
        Ok(v) => v,
        Err(_) => return errno(EFAULT),
    };
    // Bound the user-supplied length before deriving a page count: an
    // unbounded byte_len would force a huge Vec allocation and a near-infinite
    // translate loop in the kernel. 64 MiB covers any realistic surface
    // (4K BGRA is ~33 MiB) while capping the work at 16384 pages.
    const MAX_SURFACE_BYTES: u64 = 64 * 1024 * 1024;
    if desc.byte_len == 0 || desc.byte_len > MAX_SURFACE_BYTES || (desc.base_va & 0xFFF) != 0 {
        return errno(EINVAL);
    }
    // The surface must be the caller's own user-writable memory. Validate the
    // whole range as user-writable before translating any address. Without
    // this, base_va could point into the higher-half directmap (present in
    // every capsule's CR3) and capture arbitrary kernel physical frames, which
    // attach would then map USER_RW. The walk rejects any page that is not
    // present, user-accessible, and writable, and bounds base_va+byte_len to
    // user space with checked arithmetic.
    if validate_user_write(desc.base_va, desc.byte_len as usize).is_err() {
        return errno(EFAULT);
    }
    let pages = ((desc.byte_len as usize) + 4095) / 4096;
    let mut frames = Vec::with_capacity(pages);
    for i in 0..pages {
        let va = VirtAddr::new(desc.base_va + (i as u64) * 4096);
        let Some(pa) = translate_address(va) else {
            return errno(EFAULT);
        };
        frames.push(PhysAddr::new(pa.as_u64() & !0xFFF));
    }
    match register_surface(pid, &desc, frames) {
        Ok((sid, h)) => {
            attach_map::record(pid, h, desc.base_va, desc.byte_len);
            trace_surface(b"register", b"ok", pid);
            crate::sys::bench::mark_once(&FIRST_SURFACE_REGISTER, b"surface_register_first");
            log_display_mode(&desc);
            SyscallResult::success_audited(sid as i64)
        }
        Err(e) => errno(map_err(e)),
    }
}

pub(super) fn do_share(sid: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    trace_surface(b"share", b"enter", pid);
    let handle = match lookup_owned(pid, sid) {
        Ok(h) => h,
        Err(e) => return errno(map_err(e)),
    };
    match share_surface(pid, handle) {
        Ok(h) => {
            trace_surface(b"share", b"ok", pid);
            SyscallResult::success_audited(h as i64)
        }
        Err(e) => errno(map_err(e)),
    }
}

pub(super) fn do_attach(handle: u64, out_desc_ptr: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    trace_surface(b"attach", b"enter", pid);
    let mut desc = SurfaceDescriptor::default();
    match attach_surface(pid, handle, &mut desc) {
        Ok(va) => {
            if out_desc_ptr != 0 && write_user_value(out_desc_ptr, &desc).is_err() {
                return errno(EFAULT);
            }
            trace_surface(b"attach", b"ok", pid);
            SyscallResult::success_audited(va as i64)
        }
        Err(e) => errno(map_err(e)),
    }
}

pub(super) fn do_release(handle: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    // Only a holder (the owner or an attacher, both of which have an attach
    // record for this handle) may drop a reference. Without this, any capsule
    // could guess a handle -- epochs are small and there are only 256 slots --
    // and force-free another capsule's surface, desyncing its refcount and
    // invalidating its sid out from under it.
    if attach_map::lookup(pid, handle).is_none() {
        return errno(EPERM);
    }
    attach_map::forget(pid, handle);
    match release_surface(handle) {
        Ok(n) => SyscallResult::success_audited(n as i64),
        Err(e) => errno(map_err(e)),
    }
}

pub(super) fn do_present(handle: u64, x: u64, y: u64, w: u64, h: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    let (base_va, byte_len) = match lookup_attached_va(pid, handle) {
        Some(v) => v,
        None => return errno(EINVAL),
    };
    let result = super::graphics_present::handle(0, base_va, byte_len as usize, x, y, w, h);
    if !result.is_error() {
        crate::sys::bench::mark_once(&FIRST_SURFACE_PRESENT, b"surface_present_first");
    }
    result
}

pub(super) fn do_vsync_wait(display: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    trace_surface(b"vsync", b"enter", pid);
    match wait_for_vsync(display as u32, pid) {
        Ok(deadline) => {
            trace_surface(b"vsync", b"ok", pid);
            SyscallResult::success_audited(deadline as i64)
        }
        Err(e) => errno(map_err(e)),
    }
}
