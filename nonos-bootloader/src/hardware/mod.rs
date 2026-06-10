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

mod acpi;
mod cpu;
mod devices;
mod discover;
mod display;
mod memory;
pub mod tpm;
mod types;

pub use acpi::{discover_acpi_rsdp, get_cpu_count_from_acpi};
pub use cpu::detect_cpu_features;
pub use devices::{
    detect_dual_gpu, enumerate_graphics, enumerate_network, enumerate_pci, enumerate_storage,
    needs_safe_exit,
};
pub use discover::discover_system_hardware;
pub use display::display_hardware_summary;
pub use memory::discover_memory_size;
pub use types::{CpuFeatureFlags, HardwareInfo, RsdpDescriptor};
