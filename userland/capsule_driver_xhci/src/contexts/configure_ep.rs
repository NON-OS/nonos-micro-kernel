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
const INPUT_CONTROL_INDEX: usize = 0;
const SLOT_CTX_INDEX: usize = 1;
const CONTEXT_ENTRIES_SHIFT: u32 = 27;
const SPEED_SHIFT: u32 = 20;
const ROOT_PORT_SHIFT: u32 = 16;
const ADD_SLOT_FLAG: u32 = 1;
const EP_TYPE_INTERRUPT_IN: u32 = 7;
const CERR: u32 = 3;
const DCS: u32 = 1;
/// The parameters of the interrupt-IN endpoint being configured, bundled so the
/// writer stays within the argument limit.
#[derive(Clone, Copy)]
pub struct EndpointConfig {
    pub context_size: u8,
    pub dci: u8,
    pub ring_phys: u64,
    pub max_packet: u16,
    pub interval: u8,
    pub speed: u8,
    pub root_port: u8,
}

pub fn write_configure_endpoint_input(region: &DmaRegion, cfg: EndpointConfig) {
    region.zero();
    write_dw(region, cfg.context_size, INPUT_CONTROL_INDEX, 1, ADD_SLOT_FLAG | (1 << cfg.dci));
    let slot_dw0 = ((cfg.speed as u32) << SPEED_SHIFT) | ((cfg.dci as u32) << CONTEXT_ENTRIES_SHIFT);
    write_dw(region, cfg.context_size, SLOT_CTX_INDEX, 0, slot_dw0);
    write_dw(region, cfg.context_size, SLOT_CTX_INDEX, 1, (cfg.root_port as u32) << ROOT_PORT_SHIFT);
    write_endpoint(region, cfg.context_size, cfg.dci as usize + 1, cfg.ring_phys, cfg.max_packet, cfg.interval);
}
fn write_endpoint(
    region: &DmaRegion,
    context_size: u8,
    dci: usize,
    ring_phys: u64,
    max_packet: u16,
    interval: u8,
) {
    let dw0 = (interval as u32) << 16;
    let dw1 = (CERR << 1) | (EP_TYPE_INTERRUPT_IN << 3) | ((max_packet as u32) << 16);
    write_dw(region, context_size, dci, 0, dw0);
    write_dw(region, context_size, dci, 1, dw1);
    write_dw(region, context_size, dci, 2, (ring_phys as u32) | DCS);
    write_dw(region, context_size, dci, 3, (ring_phys >> 32) as u32);
    write_dw(region, context_size, dci, 4, (max_packet as u32) | ((max_packet as u32) << 16));
}
fn write_dw(region: &DmaRegion, context_size: u8, context: usize, dword: usize, value: u32) {
    let byte = context * context_size as usize + dword * core::mem::size_of::<u32>();
    unsafe {
        core::ptr::write_volatile(region.as_mut_ptr::<u8>().add(byte) as *mut u32, value);
    }
}
