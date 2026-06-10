// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod allocate;
mod free;
mod record;
mod table;
mod util;

pub use allocate::{allocate_anywhere, allocate_at_address, allocate_below_4gb};
pub use free::{free_all, to_array};
pub use record::{AllocationRecord, MemoryRegion};
pub use table::AllocationTable;
pub use util::{
    copy_memory, is_page_aligned, page_align_down, page_align_up, pages_for_size, zero_memory,
};
