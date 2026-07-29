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

//! Lets the `x86_64` crate's page-table builders draw from our allocator.
//!
//! The allocator itself deals in `PhysAddr` and knows nothing about any one
//! architecture's frame type. This shim is the only place that translation
//! happens, which is why it is the only part of the module gated to x86_64.

use super::allocator::FrameAllocator;
use x86_64::structures::paging::{FrameAllocator as X86FrameAllocator, PhysFrame, Size4KiB};

// SAFETY: `FrameAllocator::alloc` hands back a frame drawn from the physical
// bitmap that owns all of RAM, so each address is unused and stays reserved
// until it is freed. That is exactly the uniqueness the trait requires.
unsafe impl X86FrameAllocator<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.alloc().map(|addr| PhysFrame::containing_address(x86_64::PhysAddr::new(addr.as_u64())))
    }
}
