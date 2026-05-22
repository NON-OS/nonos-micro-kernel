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

use x86_64::structures::idt::InterruptStackFrame;

use super::cpl::cpl_from_cs;
use super::print_hex::print_hex_u64;
use crate::arch::x86_64::paging::read_cr3;

pub fn dump_trap(name: &[u8], frame: &InterruptStackFrame, err: Option<u64>, cr2: Option<u64>) {
    let cs = frame.code_segment as u64;
    let cpl = cpl_from_cs(cs);
    let rip = frame.instruction_pointer.as_u64();
    let rsp = frame.stack_pointer.as_u64();
    let ss = frame.stack_segment as u64;
    let rflags = frame.cpu_flags;
    let cr3 = read_cr3();
    let asid = crate::memory::paging::manager::active_asid().unwrap_or(0xFFFF_FFFF);
    let pid = crate::process::current_pid().unwrap_or(0);

    crate::sys::serial::print(b"[TRAP ");
    crate::sys::serial::print(name);
    crate::sys::serial::print(b"] cpl=");
    let cpl_byte = [b'0' + cpl];
    crate::sys::serial::print(&cpl_byte);
    crate::sys::serial::print(b" rip=");
    print_hex_u64(rip);
    crate::sys::serial::print(b" rsp=");
    print_hex_u64(rsp);
    crate::sys::serial::print(b" cs=");
    print_hex_u64(cs);
    crate::sys::serial::print(b" ss=");
    print_hex_u64(ss);
    crate::sys::serial::print(b" rflags=");
    print_hex_u64(rflags);
    crate::sys::serial::print(b" cr3=");
    print_hex_u64(cr3);
    crate::sys::serial::print(b" asid=");
    print_hex_u64(asid as u64);
    crate::sys::serial::print(b" pid=");
    print_hex_u64(pid as u64);
    if let Some(e) = err {
        crate::sys::serial::print(b" err=");
        print_hex_u64(e);
    }
    if let Some(c) = cr2 {
        crate::sys::serial::print(b" cr2=");
        print_hex_u64(c);
    }
    crate::sys::serial::println(b"");
}

#[cfg(feature = "nonos-trap-kstack-writer")]
pub fn dump_user_around_rsp(rsp: u64) {
    let page_lo = rsp & !0xfff;
    let page_hi = page_lo.wrapping_add(0x1000);
    let lo = rsp.saturating_sub(128) & !0x7;
    let lo = if lo < page_lo { page_lo } else { lo };
    let hi = rsp.saturating_add(128);
    let hi = if hi > page_hi { page_hi } else { hi };
    let mut va = lo;
    while va + 8 <= hi {
        let v = unsafe { core::ptr::read_volatile(va as *const u64) };
        crate::sys::serial::print(b"[USER-DUMP] ");
        print_hex_u64(va);
        crate::sys::serial::print(b" = ");
        print_hex_u64(v);
        crate::sys::serial::println(b"");
        va = va.wrapping_add(8);
    }
}
