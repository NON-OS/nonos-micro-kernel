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

use core::arch::asm;

use super::method::PsciMethod;
use super::state::method;

pub fn psci_call(func: u32, arg0: u64, arg1: u64, arg2: u64) -> i64 {
    match method() {
        PsciMethod::Smc => call_smc(func, arg0, arg1, arg2),
        PsciMethod::Hvc => call_hvc(func, arg0, arg1, arg2),
    }
}

fn call_smc(func: u32, arg0: u64, arg1: u64, arg2: u64) -> i64 {
    let ret: i64;
    unsafe {
        asm!("smc #0", inout("x0") func as u64 => ret, in("x1") arg0, in("x2") arg1, in("x3") arg2, options(nomem, nostack));
    }
    ret
}

fn call_hvc(func: u32, arg0: u64, arg1: u64, arg2: u64) -> i64 {
    let ret: i64;
    unsafe {
        asm!("hvc #0", inout("x0") func as u64 => ret, in("x1") arg0, in("x2") arg1, in("x3") arg2, options(nomem, nostack));
    }
    ret
}

pub fn psci_call0(func: u32) -> i64 {
    psci_call(func, 0, 0, 0)
}

pub fn psci_call1(func: u32, arg0: u64) -> i64 {
    psci_call(func, arg0, 0, 0)
}

pub fn psci_call2(func: u32, arg0: u64, arg1: u64) -> i64 {
    psci_call(func, arg0, arg1, 0)
}
