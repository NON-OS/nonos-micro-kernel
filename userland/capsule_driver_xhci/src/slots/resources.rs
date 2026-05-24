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
use crate::contexts::{
    device_context_bytes, input_context_bytes, max_packet_for_speed, write_address_device_input,
};
use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;
use crate::rings::transfer::TransferRing;
pub struct SlotResources {
    pub slot_id: u8,
    pub port_id: u8,
    pub speed: u8,
    pub max_packet: u16,
    pub output_context: DmaRegion,
    pub input_context: DmaRegion,
    pub ep0: TransferRing,
}
impl SlotResources {
    pub fn allocate(
        pool: &DmaPool,
        context_size: u8,
        slot_id: u8,
        port_id: u8,
        speed: u8,
    ) -> XhciResult<Self> {
        let output_context = pool.alloc(device_context_bytes(context_size))?;
        let input_context = pool.alloc(input_context_bytes(context_size))?;
        let ep0 = TransferRing::new(pool)?;
        output_context.zero();
        input_context.zero();
        let max_packet = max_packet_for_speed(speed);
        write_address_device_input(
            &input_context,
            context_size,
            port_id,
            speed,
            max_packet,
            ep0.phys(),
        );
        Ok(Self { slot_id, port_id, speed, max_packet, output_context, input_context, ep0 })
    }
}