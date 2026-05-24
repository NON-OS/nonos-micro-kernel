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

mod enumerate;
mod helpers;
mod parse;
mod walker;

pub use enumerate::{
    enumerate_capabilities, enumerate_pcie_capabilities, find_capability, has_capability,
};
pub use helpers::{
    collect_all_capabilities, get_msi_info, get_msix_info, get_pcie_info,
    get_power_management_info, get_virtio_info, has_acs_capability, has_aer_capability,
    has_ats_capability, has_pasid_capability, has_sriov_capability,
};
pub use parse::{
    parse_msi_capability, parse_msix_capability, parse_pcie_capability,
    parse_power_management_capability,
};
pub use walker::CapabilityWalker;
