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

pub mod constants;
pub mod dma;
pub mod error;
pub mod examples;
pub mod lba;
mod limiters;
pub mod mmio;
pub mod pci;
pub mod rate_limiter;

pub use constants::{
    ASSUMED_CPU_FREQ_MHZ, DEFAULT_ADMIN_OPS_PER_SEC, DEFAULT_DMA_OPS_PER_SEC,
    DEFAULT_IO_OPS_PER_SEC, HIGH_MMIO_START, KERNEL_PHYS_END, LOW_MMIO_END, LOW_MMIO_START,
    MAX_DMA_SIZE, MAX_PHYS_ADDR_BITS, MAX_PRP_ENTRIES, PAGE_SIZE, PCI_CONFIG_SPACE_SIZE,
    PCI_EXTENDED_CONFIG_SIZE, PCI_MAX_BUS, PCI_MAX_DEVICE, PCI_MAX_FUNCTION, PLATFORM_MMIO_END,
    PLATFORM_MMIO_START, PROTECTED_CONFIG_OFFSETS, RATE_LIMIT_WINDOW_MS,
};

pub use dma::{validate_dma_buffer, validate_prp_list, validate_sg_list};

pub use error::DriverError;

pub use lba::{
    is_lba_in_partition, validate_lba_in_partition, validate_lba_range,
    validate_lba_range_with_size,
};

pub use limiters::{
    admin_rate_limiter, default_multi_rate_limiter, dma_rate_limiter, io_rate_limiter,
};

pub use mmio::{
    safe_mmio_read32, safe_mmio_read64, safe_mmio_write32, safe_mmio_write64, validate_mmio_region,
};

pub use pci::{
    build_config_address, is_config_write_allowed, is_sensitive_config_read, validate_pci_access,
    validate_pci_extended_access,
};

pub use rate_limiter::{DriverOpType, MultiRateLimiter, RateLimiter};
