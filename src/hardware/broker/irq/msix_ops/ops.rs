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

use crate::drivers::pci::types::{MsixInfo, PciAddress, PciBar};

use super::super::types::IrqBindError;

pub(in crate::hardware::broker::irq) trait MsixOps: Send + Sync {
    fn program_run(
        &self,
        address: &PciAddress,
        msix: &MsixInfo,
        bars: &[PciBar; 6],
        base_vector: u8,
        count: usize,
        dest_apic_id: u8,
    ) -> Result<(), IrqBindError>;

    fn teardown_vector(
        &self,
        address: &PciAddress,
        msix: &MsixInfo,
        bars: &[PciBar; 6],
        device_vector: u16,
    );

    fn disable_for_device(&self, address: &PciAddress, msix: &MsixInfo);
}
