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

use core::sync::atomic::{AtomicBool, Ordering};

use nonos_abi::mmap;

use super::allocator::ALLOCATOR;
use super::error::AllocError;

const INITIAL_HEAP_SIZE: usize = 16 * 1024 * 1024;
const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const MAP_PRIVATE: i32 = 0x02;
const MAP_ANONYMOUS: i32 = 0x20;
const USERSPACE_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn init() -> Result<(), AllocError> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Err(AllocError::AlreadyInitialized);
    }
    let base = mmap(
        core::ptr::null_mut(),
        INITIAL_HEAP_SIZE,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    );
    if base.is_null() || (base as i64) < 0 || base as u64 > USERSPACE_MAX {
        INITIALIZED.store(false, Ordering::SeqCst);
        return Err(AllocError::MmapFailed);
    }
    unsafe {
        ALLOCATOR.init(base, INITIAL_HEAP_SIZE);
    }
    Ok(())
}
