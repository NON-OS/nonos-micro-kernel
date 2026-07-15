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

//! Enumerate the platform GPIO community controllers from the ACPI namespace.
//!
//! A touchpad's GpioInt descriptor names a GPIO controller as its
//! ResourceSource; that controller is an ACPI device (never a PCI function), so
//! PCI enumeration cannot find it and the community's interrupt status
//! register stays unreachable until the device is matched here by `_HID`.

use alloc::vec::Vec;

use super::controller::parse_memory32_fixed;
use super::crs::{resource_bytes, scan_large_descriptors, walk_large_descriptors};
use super::scan::{find_devices, find_name_value};
use super::tables;
use super::types::GpioController;

const LEAD_MEMORY32_FIXED: u8 = 0x86;

/// Enumerate GPIO community controllers from the ACPI DSDT/SSDT, returning
/// those with a usable MMIO window from their `_CRS`. Empty on any failure;
/// never panics.
pub fn enumerate_gpio_controllers() -> Vec<GpioController> {
    let mut out = Vec::new();
    for aml in tables::aml_blocks() {
        for scope in find_devices(&aml, hid_is_gpio_controller) {
            let mut ctl = GpioController::new(scope.hid);
            ctl.name = scope.name;
            ctl.uid = parse_uid(scope.body).unwrap_or(0);
            if let Some(crs) = scope.crs {
                parse_gpio_crs(crs, &mut ctl);
            }
            // A `_CRS` method built from named templates keeps its descriptors
            // in the device body; scan there when the scoped parse found no
            // window, mirroring the I2C-HID enumerator.
            if ctl.mmio_base == 0 {
                parse_gpio_crs(scope.body, &mut ctl);
            }
            if ctl.is_valid() {
                out.push(ctl);
            }
        }
    }
    out
}

/// Match a decoded `_HID` against the Intel pinctrl-family GPIO community
/// controllers:
///
///   * `INT3452`  Apollo Lake / Broxton
///   * `INT3453`  Gemini Lake
///   * `INT344B`  Sunrise Point (Skylake/Kaby Lake PCH)
///   * `INT34C5`  Tiger Lake-LP
///   * `INT34C6`  Tiger Lake-H
///   * `INTC1055` Alder Lake-P (reused on Raptor Lake-P)
///   * `INTC1056` Alder Lake-S
///
/// Seven-character ids carry a trailing zero byte; eight-character ids fill
/// all eight, following `hid_is_i2c_controller`.
fn hid_is_gpio_controller(hid: &[u8; 8]) -> bool {
    const IDS: [&[u8]; 7] =
        [b"INT3452", b"INT3453", b"INT344B", b"INT34C5", b"INT34C6", b"INTC1055", b"INTC1056"];
    IDS.iter().any(|&id| id_matches(hid, id))
}

/// Compare a decoded `_HID` against one identifier, honouring the trailing
/// zero byte a seven-character id leaves in the eight-byte field.
fn id_matches(hid: &[u8; 8], id: &[u8]) -> bool {
    if id.len() == 7 {
        &hid[..7] == id && hid[7] == 0
    } else {
        &hid[..8] == id
    }
}

/// Decode a `Name (_UID, value)` in the device body as a small integer: the
/// constant opcodes Zero/One, the Byte/Word/DWord const prefixes, or a decimal
/// string (some firmware declares `_UID` as `"1"`). None when absent or in an
/// encoding we do not model; the caller then defaults to community zero.
fn parse_uid(body: &[u8]) -> Option<u32> {
    let at = find_name_value(body, b"_UID")?;
    let v = body.get(at..)?;
    match v.first().copied()? {
        0x00 => Some(0),
        0x01 => Some(1),
        0x0A => Some(u32::from(*v.get(1)?)),
        0x0B => Some(u32::from(u16::from_le_bytes([*v.get(1)?, *v.get(2)?]))),
        0x0C => Some(u32::from_le_bytes([*v.get(1)?, *v.get(2)?, *v.get(3)?, *v.get(4)?])),
        0x0D => parse_decimal_string(v.get(1..)?),
        _ => None,
    }
}

/// Parse a NUL-terminated ASCII decimal string into a u32, rejecting anything
/// that is not purely digits or that overflows.
fn parse_decimal_string(bytes: &[u8]) -> Option<u32> {
    let mut value: u32 = 0;
    let mut digits = 0usize;
    for &b in bytes {
        if b == 0 {
            break;
        }
        if !b.is_ascii_digit() {
            return None;
        }
        value = value.checked_mul(10)?.checked_add(u32::from(b - b'0'))?;
        digits += 1;
    }
    (digits > 0).then_some(value)
}

/// Fill a `GpioController` from its `_CRS` ResourceTemplate, preferring the
/// length-delimited descriptor walk and falling back to a raw byte scan when
/// the wrapper offset is unusual, exactly like `parse_controller_crs`.
fn parse_gpio_crs(crs_value: &[u8], ctl: &mut GpioController) {
    if let Some(bytes) = resource_bytes(crs_value) {
        walk_large_descriptors(bytes, |lead, data| apply_memory32(lead, data, ctl));
    }
    if ctl.mmio_base == 0 {
        scan_large_descriptors(crs_value, |lead, data| apply_memory32(lead, data, ctl));
    }
}

/// Take the community's MMIO window from the first Memory32Fixed descriptor.
/// Returns true once the window is known so a walk can stop.
fn apply_memory32(lead: u8, data: &[u8], ctl: &mut GpioController) -> bool {
    if lead == LEAD_MEMORY32_FIXED && ctl.mmio_base == 0 {
        if let Some((base, size)) = parse_memory32_fixed(data) {
            ctl.mmio_base = u64::from(base);
            ctl.mmio_size = u64::from(size);
        }
    }
    ctl.mmio_base != 0
}
