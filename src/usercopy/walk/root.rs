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

//! Resolve the page-table root that the page walk should use. A
//! syscall handler runs on the caller's CR3, so the active CR3 is the
//! fallback when the per-CPU execution-context mirror has not been
//! populated. There is no inline asm at this layer.

use crate::arch::active_page_table_root;
use crate::context::{get_current_context, ExecutionContext};
use crate::memory::paging::constants::PTE_ADDR_MASK;
use crate::usercopy::error::UsercopyError;

pub(super) fn page_table_root() -> Result<u64, UsercopyError> {
    match get_current_context() {
        ExecutionContext::Process(ctx) => Ok(ctx.page_table_root),
        ExecutionContext::Kernel(_) | ExecutionContext::None => Ok(active_cr3_root()),
    }
}

fn active_cr3_root() -> u64 {
    active_page_table_root() & PTE_ADDR_MASK
}
