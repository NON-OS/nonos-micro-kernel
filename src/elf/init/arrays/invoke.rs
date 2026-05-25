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

use core::ptr;

use crate::memory::addr::VirtAddr;

use super::{constants::{InitFn, INIT_FN_SIZE}, info::{InitArrayInfo, PreInitArrayInfo}};

pub(super) unsafe fn invoke_addr(addr: VirtAddr) {
    let init_fn: InitFn = unsafe { core::mem::transmute(addr.as_u64()) };
    init_fn();
}

pub(super) unsafe fn invoke_array_info(addr: VirtAddr, count: usize) {
    for index in 0..count {
        let fn_ptr_addr = addr.as_u64() + (index * INIT_FN_SIZE) as u64;
        let fn_ptr = unsafe { ptr::read(fn_ptr_addr as *const u64) };
        if fn_ptr != 0 && fn_ptr != u64::MAX {
            let init_fn: InitFn = unsafe { core::mem::transmute(fn_ptr) };
            init_fn();
        }
    }
}

pub(super) unsafe fn invoke_init_array(info: &InitArrayInfo) { unsafe { invoke_array_info(info.addr, info.count()) } }
pub(super) unsafe fn invoke_preinit_array(info: &PreInitArrayInfo) { unsafe { invoke_array_info(info.addr, info.count()) } }
