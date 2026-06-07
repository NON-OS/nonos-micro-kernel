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

use alloc::vec::Vec;

use crate::descriptors::{HidBinding, HidKind};
use crate::xhci::{alloc_transfer_ring, control_transfer};

use super::enumerate::HidEndpoint;

pub fn configure_binding(
    xhci_port: u32,
    slot: u8,
    binding: HidBinding,
    out: &mut Vec<HidEndpoint>,
) {
    if matches!(binding.kind, HidKind::Keyboard | HidKind::Mouse) {
        let iface = binding.interface_number as u16;
        let mut dummy = [0u8; 0];
        let _ = control_transfer(xhci_port, slot, 0x21, 0x0B, 0, iface, 0, &mut dummy);
    }
    let Ok(dci) = alloc_transfer_ring(
        xhci_port,
        slot,
        binding.endpoint_address,
        binding.max_packet_size,
        binding.interval,
    ) else {
        return;
    };
    out.push(HidEndpoint {
        port: xhci_port,
        slot,
        dci,
        kind: binding.kind,
        max_packet: binding.max_packet_size,
    });
}
