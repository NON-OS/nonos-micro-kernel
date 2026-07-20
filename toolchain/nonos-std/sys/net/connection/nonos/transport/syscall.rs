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

// Raw syscall ABI stubs for the net backend: the 4-byte tag encoder and the
// 5- and 6-argument syscall entries (the 6-arg form carries the IPC deadline).

pub(crate) const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

pub(crate) unsafe fn sys5(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> i64 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
            in("rdx") a3, in("r10") a4, in("r8") a5, out("rcx") _, out("r11") _);
    }
    r
}

pub(crate) unsafe fn sys6(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> i64 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
            in("rdx") a3, in("r10") a4, in("r8") a5, in("r9") a6, out("rcx") _, out("r11") _);
    }
    r
}
