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

use alloc::vec::Vec;

use super::super::config::ConfigSpace;
use super::super::constants::*;
use super::super::error::Result;
use super::super::types::{
    MsiInfo, MsixInfo, PciCapability, PcieInfo, PowerManagementInfo, VirtioPciCfg,
};
use super::enumerate::enumerate_pcie_capabilities;
use super::parse::{
    parse_msi_capability, parse_msix_capability, parse_pcie_capability,
    parse_power_management_capability,
};
use super::walker::CapabilityWalker;

pub fn get_msi_info(config: &ConfigSpace) -> Result<Option<MsiInfo>> {
    if let Some(walker) = CapabilityWalker::new(config)? {
        for cap_result in walker {
            let cap = cap_result?;
            if cap.id == CAP_ID_MSI {
                return Ok(Some(parse_msi_capability(config, &cap)?));
            }
        }
    }
    Ok(None)
}

pub fn get_msix_info(config: &ConfigSpace) -> Result<Option<MsixInfo>> {
    if let Some(walker) = CapabilityWalker::new(config)? {
        for cap_result in walker {
            let cap = cap_result?;
            if cap.id == CAP_ID_MSIX {
                return Ok(Some(parse_msix_capability(config, &cap)?));
            }
        }
    }
    Ok(None)
}

pub fn get_virtio_info(config: &ConfigSpace) -> Result<Option<VirtioPciCfg>> {
    let mut cfg = VirtioPciCfg::default();
    if let Some(walker) = CapabilityWalker::new(config)? {
        for cap_result in walker {
            let cap = cap_result?;
            if cap.id != CAP_ID_VNDR {
                continue;
            }
            let cfg_type = config.read8(cap.offset as u16 + 3)?;
            let bar = config.read8(cap.offset as u16 + 4)?;
            let off = config.read32(cap.offset as u16 + 8)?;
            match cfg_type {
                1 => {
                    cfg.common_bar = bar;
                    cfg.common_off = off;
                    cfg.present = true;
                }
                2 => {
                    cfg.notify_bar = bar;
                    cfg.notify_off = off;
                    cfg.notify_mult = config.read32(cap.offset as u16 + 16)?;
                }
                3 => {
                    cfg.isr_off = off;
                }
                4 => {
                    cfg.device_bar = bar;
                    cfg.device_off = off;
                }
                _ => {}
            }
        }
    }
    Ok(if cfg.present { Some(cfg) } else { None })
}

pub fn get_power_management_info(config: &ConfigSpace) -> Result<Option<PowerManagementInfo>> {
    if let Some(walker) = CapabilityWalker::new(config)? {
        for cap_result in walker {
            let cap = cap_result?;
            if cap.id == CAP_ID_PM {
                return Ok(Some(parse_power_management_capability(config, &cap)?));
            }
        }
    }
    Ok(None)
}

pub fn get_pcie_info(config: &ConfigSpace) -> Result<Option<PcieInfo>> {
    if let Some(walker) = CapabilityWalker::new(config)? {
        for cap_result in walker {
            let cap = cap_result?;
            if cap.id == CAP_ID_PCIE {
                return Ok(Some(parse_pcie_capability(config, &cap)?));
            }
        }
    }
    Ok(None)
}

pub fn collect_all_capabilities(config: &ConfigSpace) -> Result<Vec<PciCapability>> {
    let mut caps = Vec::new();

    if let Some(walker) = CapabilityWalker::new(config)? {
        for cap_result in walker {
            caps.push(cap_result?);
        }
    }

    Ok(caps)
}

pub fn has_aer_capability(bus: u8, device: u8, function: u8) -> bool {
    enumerate_pcie_capabilities(bus, device, function).into_iter().any(|c| c.id == PCIE_CAP_ID_AER)
}

pub fn has_acs_capability(bus: u8, device: u8, function: u8) -> bool {
    enumerate_pcie_capabilities(bus, device, function).into_iter().any(|c| c.id == PCIE_CAP_ID_ACS)
}

pub fn has_sriov_capability(bus: u8, device: u8, function: u8) -> bool {
    enumerate_pcie_capabilities(bus, device, function)
        .into_iter()
        .any(|c| c.id == PCIE_CAP_ID_SRIOV)
}

pub fn has_pasid_capability(bus: u8, device: u8, function: u8) -> bool {
    enumerate_pcie_capabilities(bus, device, function)
        .into_iter()
        .any(|c| c.id == PCIE_CAP_ID_PASID)
}

pub fn has_ats_capability(bus: u8, device: u8, function: u8) -> bool {
    enumerate_pcie_capabilities(bus, device, function).into_iter().any(|c| c.id == PCIE_CAP_ID_ATS)
}
