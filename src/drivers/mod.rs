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

// Microkernel hardware bring-up surface. The boot path needs PCI
// enumeration (from `init_core_systems`), the virtio-rng entropy probe
// (from `init_entropy`), and the security helpers used by the trusted
// memory/IPC paths.
//
// `device_info` enumerates devices through the legacy `critical`
// taxonomy and only ships diagnostics for the gated audio/gpu/xhci/ahci
// stack; no microkernel caller reads it.
pub mod pci;
pub mod security;
pub mod virtio_rng;

pub use pci::{get_pci_manager, init_pci, PciBar, PciCapability, PciDevice, PciManager, PciStats};
pub use security::{
    is_config_write_allowed, safe_mmio_read32, safe_mmio_write32, validate_dma_buffer,
    validate_lba_range, validate_mmio_region, validate_pci_access, validate_prp_list, DriverError,
    DriverOpType, RateLimiter,
};
pub use virtio_rng::init_virtio_rng;

// Legacy diagnostics. `critical` and `stats` enumerate the
// audio/gpu/xhci/ahci device set; they cannot exist without the
// drivers behind them. Off in microkernel.

// Legacy hardware stack. Storage controllers, network NICs, USB,
// audio, GPU, TPM, I2C, the legacy console, plus the orchestrators
// that drive them. Off in every microkernel profile. These are the
// drivers slated for capsule migration or deletion.
