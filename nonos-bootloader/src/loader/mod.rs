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

pub mod core;
pub mod dynamic;
pub mod errors;
pub mod file;
pub mod image;
pub mod memory;
pub mod reloc;
pub mod security;
pub mod segment;
pub mod types;
pub mod validate;

#[cfg(test)]
mod tests;

pub use core::load_kernel;

pub use image::{KernelImage, KernelImageBuilder, KernelInfo};

pub use errors::{LoaderError, LoaderResult};

pub use file::{file_exists, load_file_from_esp, load_kernel_binary, FileLoadError};

pub use types::{
    dyn_tag, elf_class, elf_data, elf_machine, elf_type, memory as elf_memory, ph_flags, ph_type,
    sh_type, DynamicInfo, Elf64Dyn, Elf64Header, Elf64Phdr, Elf64Shdr, Elf64Sym, LoadedSegment,
    ELF_MAGIC,
};

pub use validate::{
    validate_elf, validate_elf_strict, validate_entry_point, validate_header, validate_ident,
    validate_magic, validate_program_header, validate_program_headers, ValidationContext,
};

pub use memory::{
    allocate_anywhere, allocate_at_address, allocate_below_4gb, copy_memory, is_page_aligned,
    page_align_down, page_align_up, pages_for_size, zero_memory, AllocationRecord, AllocationTable,
    MemoryRegion,
};

pub use segment::{
    calculate_memory_bounds, check_segment_overlaps, count_wx_violations, load_all_segments,
    load_segment, total_file_size, total_memory_size, validate_segment_addresses, SegmentLoadInfo,
    SegmentPermissions,
};

pub use security::{
    check_address_bounds, check_critical_memory, check_pie_policy, check_size_policy,
    check_wx_policy, compute_kernel_hash, validate_security, verify_kernel_hash, SecurityAudit,
    SecurityCheckResult, SecurityPolicy,
};

pub use dynamic::{
    estimate_symbol_count, needs_relocations, parse_dynamic_section, relocation_count,
};
pub use reloc::{
    dyn_tag as reloc_dyn_tag, process_elf_relocations, process_relocations, reloc_type, Dyn64,
    Rela64, RelocationContext,
};
