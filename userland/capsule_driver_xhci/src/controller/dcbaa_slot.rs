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
use crate::dma::DmaRegion;
use crate::error::{XhciError, XhciResult};
pub fn set_dcbaa_slot(
    dcbaa: &DmaRegion,
    slot_id: u8,
    max_slots: u8,
    device_context_phys: u64,
) -> XhciResult<()> {
    write_dcbaa_slot(dcbaa, slot_id, max_slots, device_context_phys)
}
pub fn clear_dcbaa_slot(dcbaa: &DmaRegion, slot_id: u8, max_slots: u8) -> XhciResult<()> {
    write_dcbaa_slot(dcbaa, slot_id, max_slots, 0)
}
fn write_dcbaa_slot(dcbaa: &DmaRegion, slot_id: u8, max_slots: u8, value: u64) -> XhciResult<()> {
    if slot_id == 0 || slot_id > max_slots {
        return Err(XhciError::ControllerUnsupported);
    }
    unsafe {
        core::ptr::write_volatile(dcbaa.as_mut_ptr::<u64>().add(slot_id as usize), value);
    }
    Ok(())
}
