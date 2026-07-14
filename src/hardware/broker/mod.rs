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

mod acpi_i2c;
mod census;
mod claim;
mod class;
mod device;
pub mod dma;
mod grant;
// IRQ broker is cross-arch: x86 uses IO-APIC + MSI-X; aarch64 uses
// GICv3 SPIs; riscv64 uses PLIC external sources. Backend selection
// happens inside `irq::mod.rs` via `target_arch` cfg.
pub mod irq;
pub mod mmio;
pub mod pci;
pub(crate) mod pci_index;
// PIO is an x86-only instruction class. Non-x86 builds skip the
// broker submodule entirely; the syscall layer fail-closes with
// ENOSYS via `syscall/microkernel/pio/unsupported.rs`.
#[cfg(target_arch = "x86_64")]
pub mod pio;
mod platform;
mod power;
mod table;

pub use acpi_i2c::register_acpi_i2c;
pub use census::render_and_hold as device_census;
pub use claim::{
    claim as claim_device, lookup as claim_lookup, release as release_device,
    release_all_for_pid as release_claims_for_pid, Claim, ClaimError,
};
pub use class::{classify_pci, Class};
pub use device::{Bar, BarKind, BusKind, DeviceRecord, DEVICE_FLAG_CLAIMED, DEVICE_FLAG_DISABLED};
pub use dma::{
    map_for_caller as dma_map_for_caller, release_all_for_pid as dma_release_all_for_pid,
    release_for_device as dma_release_for_device, unmap_grant as dma_unmap_grant, DmaError,
    DmaGrant, DmaMapError, DmaMapRequest, DmaMapResult,
};
pub use grant::{GrantError, MmioGrant};
pub use irq::{
    ack_grant as irq_ack_grant, bind as irq_bind, poll as irq_poll,
    release_all_for_pid as irq_release_all_for_pid, release_for_device as irq_release_for_device,
    unmap_grant as irq_unmap_grant, wait_arm as irq_wait_arm, wait_disarm as irq_wait_disarm,
    IrqBindError, IrqBindRequest, IrqBindResult, IrqError, IrqPollResult,
};
pub use mmio::{
    map_for_caller, release_all_for_pid, release_for_device, unmap_grant, MmioMapError,
    MmioMapRequest, MmioMapResult,
};
pub use pci::{
    read as pci_config_read, write as pci_config_write, PciReadError, PciReadRequest,
    PciWriteError, PciWriteRequest,
};
#[cfg(target_arch = "x86_64")]
pub use pio::{
    grant_for_caller as pio_grant_for_caller, read as pio_read,
    release_all_for_pid as pio_release_all_for_pid, release_for_device as pio_release_for_device,
    release_grant as pio_release_grant, write as pio_write, PioError, PioGrantRequest,
    PioGrantResult, PioWidth,
};
pub use platform::register_legacy as register_legacy_platform_devices;
pub use table::{contains, init_from_pci, list, list_by_class};
