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

use crate::descriptors::{hid_bindings, HidKind};
use crate::xhci::{
    address_device, control_transfer, enable_slot, get_config_descriptor,
    port_status, PortSnapshot,
};

use super::binding::configure_binding;

const MAX_PORTS: usize = 255;
const DESC_LEN: u16 = 64;
const PORTSC_CONNECTED: u32 = 1;

pub struct HidEndpoint {
    pub port: u32,
    pub slot: u8,
    pub dci: u8,
    pub kind: HidKind,
    pub max_packet: u16,
}

pub fn enumerate(xhci_port: u32) -> Vec<HidEndpoint> {
    let mut snapshots = [PortSnapshot { port_id: 0, portsc_raw: 0 }; MAX_PORTS];
    let Ok(count) = port_status(xhci_port, &mut snapshots) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for i in 0..count {
        let snap = snapshots[i];
        if snap.portsc_raw & PORTSC_CONNECTED == 0 {
            continue;
        }
        configure_port(xhci_port, snap, &mut out);
    }
    out
}

fn configure_port(xhci_port: u32, snap: PortSnapshot, out: &mut Vec<HidEndpoint>) {
    let Ok(slot) = enable_slot(xhci_port) else { return };
    let Ok(_dev) = address_device(xhci_port, slot, snap.port_id) else { return };
    let mut desc = [0u8; DESC_LEN as usize];
    let Ok(len) = get_config_descriptor(xhci_port, slot, DESC_LEN, &mut desc) else {
        return;
    };
    let Ok(bindings) = hid_bindings(&desc[..len]) else { return };
    if bindings.is_empty() {
        return;
    }
    let mut dummy = [0u8; 0];
    if control_transfer(xhci_port, slot, 0x00, 0x09, 1, 0, 0, &mut dummy).is_err() {
        return;
    }
    for binding in bindings {
        configure_binding(xhci_port, slot, binding, out);
    }
}
