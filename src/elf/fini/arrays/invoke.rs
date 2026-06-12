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

use super::{
    constants::{FiniFn, FINI_FN_SIZE},
    info::FiniArrayInfo,
};

pub(super) unsafe fn invoke_addr(addr: VirtAddr) {
    let fini_fn: FiniFn = unsafe { core::mem::transmute(addr.as_u64()) };
    fini_fn();
}

pub(super) unsafe fn invoke_array(info: &FiniArrayInfo) {
    for index in (0..info.count()).rev() {
        let fn_ptr_addr = info.addr.as_u64() + (index * FINI_FN_SIZE) as u64;
        let fn_ptr = unsafe { ptr::read(fn_ptr_addr as *const u64) };
        if fn_ptr != 0 && fn_ptr != u64::MAX {
            let fini_fn: FiniFn = unsafe { core::mem::transmute(fn_ptr) };
            fini_fn();
        }
    }
}
