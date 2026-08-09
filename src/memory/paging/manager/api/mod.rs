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

mod address_space;
mod faults;
mod globals;
mod init;
mod mapping;
mod mapping_in_asid;
mod protection;
mod query;
mod stats;
mod tlb_ops;

pub use address_space::{
    cleanup_address_space, create_address_space, get_process_cr3, lookup_asid_for_process,
    switch_address_space, switch_to_process_address_space,
};
pub use faults::handle_page_fault;
pub use init::{init, is_initialized};
pub use mapping::{
    map_device_memory, map_huge_page, map_kernel_page, map_page, map_user_dma, map_user_mmio,
    map_user_page, unmap_page, unmap_range, unmap_user_dma, unmap_user_mmio,
};
pub use mapping_in_asid::{map_page_in_asid, translate_in_asid, unmap_page_in_asid};
pub use protection::{
    protect_pages, protect_pages_range, update_page_flags, update_page_protection,
};
pub use query::{
    active_asid, active_page_table, address_spaces_count, get_mapping_info, get_page_permissions,
    is_mapped, translate_address,
};
pub use stats::{get_memory_usage, get_paging_stats};
pub use tlb_ops::{
    disable_write_protection, enable_write_protection, flush_tlb, get_current_cr3,
    invalidate_all_pages, invalidate_page, set_cr3,
};
