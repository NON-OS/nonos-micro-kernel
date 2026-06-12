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

mod bootinfo;
mod config;
mod exit;
mod jump;
pub mod prepare;
mod timing;
pub mod types;

pub use bootinfo::{build_bootinfo, BootInfoParams, BootModeFlags, ZeroStateBootInfo};
pub use exit::exit_and_jump;
pub use jump::{copy_memory_map, finalize_mmap, jump_to_kernel, settle_delay, MemoryMapEntry};
pub use prepare::{
    allocate_handoff_resources, build_handoff_flags, detect_cpu_security_features,
    estimate_tsc_frequency, HandoffAllocations, MAX_MMAP_ENTRIES, MMAP_PAGES,
};
pub use timing::get_uefi_time_epoch;
pub use types::{BootHandoffV1, CryptoHandoff, ZkAttestation, HANDOFF_MAGIC, HANDOFF_VERSION};
