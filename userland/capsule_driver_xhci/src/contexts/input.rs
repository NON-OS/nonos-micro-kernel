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
const ADD_SLOT_AND_EP0: u32 = 0x3;
const SLOT_CTX_INDEX: usize = 1;
const EP0_CTX_INDEX: usize = 2;
const EP_TYPE_CONTROL: u32 = 4;
pub fn write_address_device_input(
    region: &DmaRegion,
    context_size: u8,
    root_port: u8,
    speed: u8,
    max_packet: u16,
    ep0_ring_phys: u64,
) {
    region.zero();
    write_dw(region, context_size, 0, 1, ADD_SLOT_AND_EP0);
    write_slot(region, context_size, root_port, speed);
    write_ep0(region, context_size, max_packet, ep0_ring_phys);
}
fn write_slot(region: &DmaRegion, context_size: u8, root_port: u8, speed: u8) {
    let dw0 = ((speed as u32) << 20) | (1 << 27);
    let dw1 = (root_port as u32) << 16;
    write_dw(region, context_size, SLOT_CTX_INDEX, 0, dw0);
    write_dw(region, context_size, SLOT_CTX_INDEX, 1, dw1);
}
fn write_ep0(region: &DmaRegion, context_size: u8, max_packet: u16, ring_phys: u64) {
    let dw1 = (3 << 1) | (EP_TYPE_CONTROL << 3) | ((max_packet as u32) << 16);
    write_dw(region, context_size, EP0_CTX_INDEX, 1, dw1);
    write_dw(region, context_size, EP0_CTX_INDEX, 2, (ring_phys as u32) | 1);
    write_dw(region, context_size, EP0_CTX_INDEX, 3, (ring_phys >> 32) as u32);
    write_dw(region, context_size, EP0_CTX_INDEX, 4, 8);
}
fn write_dw(region: &DmaRegion, context_size: u8, context: usize, dword: usize, value: u32) {
    let byte = context * context_size as usize + dword * core::mem::size_of::<u32>();
    unsafe {
        core::ptr::write_volatile(region.as_mut_ptr::<u8>().add(byte) as *mut u32, value);
    }
}