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

use crate::memory::addr::PhysAddr;
use alloc::vec::Vec;
use core::fmt;

use super::address::PciAddress;
use super::bar::PciBar;
use super::capability::{PciCapability, PcieCapability};
use super::class_code::ClassCode;
use super::device_id::DeviceId;
use super::header::HeaderType;
use super::msi::{MsiInfo, MsixInfo};
use super::pcie::PcieInfo;
use super::power::PowerManagementInfo;

#[derive(Clone, Debug)]
pub struct PciDevice {
    pub address: PciAddress,
    pub device_id_info: DeviceId,
    pub class_code: ClassCode,
    pub header_type: HeaderType,
    pub multifunction: bool,
    pub bars: [PciBar; 6],
    pub capabilities: Vec<PciCapability>,
    pub pcie_capabilities: Vec<PcieCapability>,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub msi: Option<MsiInfo>,
    pub msix: Option<MsixInfo>,
    pub power_management: Option<PowerManagementInfo>,
    pub pcie: Option<PcieInfo>,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub progif: u8,
}

impl PciDevice {
    pub fn new(address: PciAddress) -> Self {
        Self {
            address,
            device_id_info: DeviceId::new(0xFFFF, 0xFFFF),
            class_code: ClassCode::new(0, 0, 0),
            header_type: HeaderType::Standard,
            multifunction: false,
            bars: [PciBar::NotPresent; 6],
            capabilities: Vec::new(),
            pcie_capabilities: Vec::new(),
            interrupt_line: 0xFF,
            interrupt_pin: 0,
            msi: None,
            msix: None,
            power_management: None,
            pcie: None,
            bus: address.bus,
            device: address.device,
            function: address.function,
            vendor_id: 0xFFFF,
            device_id: 0xFFFF,
            class: 0,
            subclass: 0,
            progif: 0,
        }
    }

    pub fn sync_compat_fields(&mut self) {
        self.bus = self.address.bus;
        self.device = self.address.device;
        self.function = self.address.function;
        self.vendor_id = self.device_id_info.vendor_id;
        self.device_id = self.device_id_info.device_id;
        self.class = self.class_code.class;
        self.subclass = self.class_code.subclass;
        self.progif = self.class_code.prog_if;
    }

    pub fn bus(&self) -> u8 {
        self.address.bus
    }

    pub fn device(&self) -> u8 {
        self.address.device
    }

    pub fn function(&self) -> u8 {
        self.address.function
    }

    pub fn vendor_id(&self) -> u16 {
        self.device_id_info.vendor_id
    }

    pub fn device_id_value(&self) -> u16 {
        self.device_id_info.device_id
    }

    pub fn class(&self) -> u8 {
        self.class_code.class
    }

    pub fn subclass(&self) -> u8 {
        self.class_code.subclass
    }

    pub fn prog_if(&self) -> u8 {
        self.class_code.prog_if
    }

    pub fn revision(&self) -> u8 {
        self.device_id_info.revision
    }

    pub fn get_bar(&self, index: usize) -> Option<&PciBar> {
        self.bars.get(index).filter(|b| b.is_present())
    }

    pub fn get_memory_bar(&self, index: usize) -> Option<PhysAddr> {
        self.bars.get(index).and_then(|b| b.address())
    }

    pub fn get_io_bar(&self, index: usize) -> Option<u16> {
        self.bars.get(index).and_then(|b| b.port())
    }

    pub fn find_capability(&self, id: u8) -> Option<&PciCapability> {
        self.capabilities.iter().find(|c| c.id == id)
    }

    pub fn has_capability(&self, id: u8) -> bool {
        self.capabilities.iter().any(|c| c.id == id)
    }

    pub fn supports_msi(&self) -> bool {
        self.msi.is_some()
    }

    pub fn supports_msix(&self) -> bool {
        self.msix.is_some()
    }

    pub fn is_pcie(&self) -> bool {
        self.pcie.is_some()
    }

    pub fn is_bridge(&self) -> bool {
        matches!(self.header_type, HeaderType::PciToPciBridge | HeaderType::CardBusBridge)
    }

    pub fn is_usb_controller(&self) -> bool {
        self.class_code.is_usb()
    }

    pub fn is_nvme_controller(&self) -> bool {
        self.class_code.is_nvme()
    }

    pub fn is_ahci_controller(&self) -> bool {
        self.class_code.is_ahci()
    }

    pub fn is_network_controller(&self) -> bool {
        self.class_code.is_network()
    }

    pub fn is_display_controller(&self) -> bool {
        self.class_code.is_display()
    }

    // MSI and MSI-X delivery here is written in terms of the local APIC: the
    // message address names an APIC ID and the message data carries an
    // interrupt vector. An arm64 machine routes the same PCI messages through
    // the GIC's ITS, addressed by device ID and translated through tables the
    // interrupt controller owns, so none of this transfers. These four stay
    // with the architecture whose message format they encode.
    #[cfg(target_arch = "x86_64")]
    pub fn configure_msix(
        &mut self,
        irq_vector: u8,
        dest_apic_id: u8,
    ) -> Result<(), crate::drivers::pci::error::PciError> {
        let msix =
            self.msix.as_ref().ok_or(crate::drivers::pci::error::PciError::MsixNotSupported)?;
        let config = crate::drivers::pci::config::ConfigSpace::new(self.address);
        crate::drivers::pci::msi::configure_msix(
            &config,
            msix,
            &self.bars,
            0,
            irq_vector,
            dest_apic_id,
        )?;
        crate::drivers::pci::msi::enable_msix(&config, msix)?;
        Ok(())
    }

    #[cfg(target_arch = "x86_64")]
    pub fn disable_msix(&mut self) -> Result<(), crate::drivers::pci::error::PciError> {
        let msix =
            self.msix.as_ref().ok_or(crate::drivers::pci::error::PciError::MsixNotSupported)?;
        let config = crate::drivers::pci::config::ConfigSpace::new(self.address);
        crate::drivers::pci::msi::disable_msix(&config, msix)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn configure_msi(
        &mut self,
        irq_vector: u8,
        dest_apic_id: u8,
    ) -> Result<(), crate::drivers::pci::error::PciError> {
        let msi = self.msi.as_ref().ok_or(crate::drivers::pci::error::PciError::MsiNotSupported)?;
        let config = crate::drivers::pci::config::ConfigSpace::new(self.address);
        crate::drivers::pci::msi::configure_msi(&config, msi, irq_vector, dest_apic_id)
    }

    #[cfg(target_arch = "x86_64")]
    pub fn disable_msi(&mut self) -> Result<(), crate::drivers::pci::error::PciError> {
        let msi = self.msi.as_ref().ok_or(crate::drivers::pci::error::PciError::MsiNotSupported)?;
        let config = crate::drivers::pci::config::ConfigSpace::new(self.address);
        crate::drivers::pci::msi::disable_msi(&config, msi)
    }
}

impl fmt::Display for PciDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:04x}:{:04x} {} [{}]",
            self.address,
            self.device_id_info.vendor_id,
            self.device_id_info.device_id,
            self.class_code,
            self.class_code.name()
        )
    }
}
