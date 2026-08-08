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

extern crate alloc;

mod api;
pub mod bar;
pub mod capabilities;
// Config access is ECAM or the PC port pair, chosen at run time by what the
// platform published, so the module itself is not architecture specific.
pub mod config;
pub mod constants;
pub mod error;
pub mod manager;
#[cfg(target_arch = "x86_64")]
pub mod msi;
pub mod security;
pub mod stats;
pub mod types;

pub use api::{
    add_device_to_blocklist, clear_device_blocklist, get_pci_stats_tuple, pci_read_config32,
    pci_read_config32_safe, pci_write_config32, pci_write_config32_safe,
    remove_device_from_blocklist, set_device_allowlist,
};

pub use bar::{decode_all_bars, decode_all_bars_unchecked, decode_bar, enumerate_bars, BarInfo};

pub use capabilities::{
    enumerate_capabilities, enumerate_pcie_capabilities, find_capability, get_msi_info,
    get_msix_info, get_pcie_info, get_power_management_info, has_capability, CapabilityWalker,
};

pub use config::{
    read32_unchecked, set_ecam_window, write32_unchecked, BridgeConfigSpace, ConfigSpace,
};

pub use error::{PciError, Result};

pub use manager::{
    count_devices, find_device_by_class, find_device_by_id, get_device_by_address,
    get_device_by_class, get_pci_manager, get_pci_stats, init_pci, is_initialized,
    scan_and_collect, scan_and_collect_safe, with_manager, PciManager,
};

#[cfg(target_arch = "x86_64")]
pub use msi::{
    configure_msi, configure_msi_multi, configure_msix, configure_msix_single,
    disable_legacy_interrupt, disable_msi, disable_msix, enable_legacy_interrupt, enable_msix,
    is_msi_enabled, is_msix_enabled, mask_all_msix, mask_msi_vector, mask_msix_vector,
    unmask_all_msix, unmask_msi_vector, unmask_msix_vector, MsiController,
};

pub use security::{
    add_to_allowlist, add_to_blocklist, approve_bus_master, audit_device, check_device_allowed,
    clear_allowlist, clear_blocklist, clear_bus_master_approvals, device_security_level,
    get_security_policy, get_security_stats, is_dma_capable, is_security_relevant,
    prepare_device_for_dma, remove_from_blocklist, reset_security_stats, revoke_bus_master,
    set_allowlist, set_security_policy, validate_config_write, validate_device_for_driver,
    DeviceAuditInfo, SecurityLevel, SecurityPolicy, SecurityStats,
};

pub use stats::PciStats;

pub use types::{
    BridgeInfo, ClassCode, DeviceId, HeaderType, MsiInfo, MsiMessage, MsixInfo, PciAddress, PciBar,
    PciCapability, PciDevice, PcieCapability, PcieDeviceType, PcieInfo, PowerManagementInfo,
};

#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::pci::PciStats as LegacyPciStats;
