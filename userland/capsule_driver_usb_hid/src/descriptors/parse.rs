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

use crate::protocol::MAX_HID_BINDINGS;

use super::binding::HidBinding;
use super::types::{Endpoint, Interface, DT_CONFIGURATION, DT_ENDPOINT, DT_INTERFACE};

pub fn hid_bindings(raw: &[u8]) -> Result<Vec<HidBinding>, ()> {
    validate_config(raw)?;
    let total = u16::from_le_bytes([raw[2], raw[3]]) as usize;
    let mut out = Vec::new();
    let mut iface = None;
    let mut i = 9usize;
    while i + 2 <= total {
        let len = raw[i] as usize;
        if len < 2 || i + len > total {
            return Err(());
        }
        match raw[i + 1] {
            DT_INTERFACE if len >= 9 => iface = parse_interface(&raw[i..i + len]),
            DT_ENDPOINT if len >= 7 => maybe_push(&mut out, iface, &raw[i..i + len]),
            _ => {}
        }
        if out.len() == MAX_HID_BINDINGS {
            break;
        }
        i += len;
    }
    Ok(out)
}

fn validate_config(raw: &[u8]) -> Result<(), ()> {
    if raw.len() < 9 || raw[0] < 9 || raw[1] != DT_CONFIGURATION {
        return Err(());
    }
    let total = u16::from_le_bytes([raw[2], raw[3]]) as usize;
    if total < 9 || total > raw.len() {
        return Err(());
    }
    Ok(())
}

fn parse_interface(buf: &[u8]) -> Option<Interface> {
    Some(Interface { number: buf[2], class: buf[5], subclass: buf[6], protocol: buf[7] })
}

fn maybe_push(out: &mut Vec<HidBinding>, iface: Option<Interface>, buf: &[u8]) {
    let Some(iface) = iface else { return };
    let ep = Endpoint {
        address: buf[2],
        attributes: buf[3],
        max_packet_size: u16::from_le_bytes([buf[4], buf[5]]) & 0x07ff,
        interval: buf[6],
    };
    if let Some(binding) = HidBinding::from_pair(iface, ep) {
        out.push(binding);
    }
}
