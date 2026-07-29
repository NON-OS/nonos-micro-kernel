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

use crate::sys::serial;

#[cfg(feature = "nonos-user-entry-proof")]
pub(super) fn print_syscall_msrs() {
    use crate::arch::x86_64::syscall::msr::{
        read_msr, EFER_SCE, IA32_EFER, IA32_FMASK, IA32_LSTAR, IA32_STAR,
    };
    use crate::sys::serial::print_hex as print_hex_u64;
    let efer = read_msr(IA32_EFER);
    serial::print(b"[SYSCALL-MSR] EFER=");
    print_hex_u64(efer);
    serial::print(b" SCE=");
    print_hex_u64(efer & EFER_SCE);
    serial::println(b"");
    serial::print(b"[SYSCALL-MSR] STAR=");
    print_hex_u64(read_msr(IA32_STAR));
    serial::println(b"");
    serial::print(b"[SYSCALL-MSR] LSTAR=");
    print_hex_u64(read_msr(IA32_LSTAR));
    serial::println(b"");
    serial::print(b"[SYSCALL-MSR] SFMASK=");
    print_hex_u64(read_msr(IA32_FMASK));
    serial::println(b"");
}
