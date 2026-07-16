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

//! Walk a `_CRS` ResourceTemplate and pull out the I2cSerialBus slave address
//! and the GpioInt pin number.
//!
//! The _CRS value is an AML Buffer: `BufferOp PkgLength BufferSize <bytes>`
//! where `<bytes>` is a sequence of ACPI resource descriptors terminated by an
//! EndTag (0x79). We only decode the two large descriptors we need and skip
//! everything else. Offsets follow ACPI 6.x sections 6.4.3.8 (Serial Bus /
//! I2C) and 6.4.3.8.1 (GPIO). Every field access is bounds-checked.

use crate::arch::x86_64::acpi::aml::scan::read_pkg_length;
use crate::arch::x86_64::acpi::aml::types::I2cHidDevice;

const BUFFER_OP: u8 = 0x11;

// Small resource descriptor EndTag: byte0 = 0x79 (bit7 clear, name 0x0F).
const END_TAG: u8 = 0x79;

// Large resource descriptor lead byte has bit7 set; low seven bits are type.
const LARGE_BIT: u8 = 0x80;
const LARGE_TYPE_GPIO: u8 = 0x0C;
const LARGE_TYPE_SERIAL_BUS: u8 = 0x0E;

// Full large-descriptor lead bytes (bit7 set | type), used by the byte-pattern
// fallback that scans a raw _CRS region for a specific descriptor.
const LEAD_SERIAL_BUS: u8 = LARGE_BIT | LARGE_TYPE_SERIAL_BUS; // 0x8E
const LEAD_GPIO: u8 = LARGE_BIT | LARGE_TYPE_GPIO; // 0x8C

// Serial bus type field values.
const SERIAL_BUS_TYPE_I2C: u8 = 0x01;

// GPIO connection type values.
const GPIO_CONN_TYPE_INTERRUPT: u8 = 0x00;

/// Cap on descriptors walked, defending against a malformed template.
const MAX_DESCRIPTORS: usize = 4096;

/// Parse a `_CRS` value (as located by the scanner) into the resource fields of
/// `dev`. Fills `slave_addr` from the first I2C descriptor and `gpio_pin` from
/// the first GpioInt descriptor. Missing descriptors leave the corresponding
/// field at its default. Never panics.
pub fn parse_crs(crs_value: &[u8], dev: &mut I2cHidDevice) {
    // Preferred path: unwrap the Buffer and walk the descriptor list by its
    // declared lengths. `resource_bytes` copes with both a bare Buffer and a
    // method body that returns one (it locates the first BufferOp).
    if let Some(bytes) = resource_bytes(crs_value) {
        walk_descriptors(bytes, dev);
    }

    // Robust fallback: if the structured walk did not yield what we need (for
    // example a Buffer-wrapper offset quirk, an unusual BufferSize encoding, or
    // a _CRS method whose body we could not unwrap), scan the raw _CRS region
    // for the descriptor lead bytes directly and parse from there. This is
    // resilient because a large descriptor carries its own length, so once the
    // lead byte and a type field validate we can trust the payload offsets.
    if dev.slave_addr == 0 || !dev.has_gpio {
        scan_descriptors(crs_value, dev);
    }
}

/// Walk a descriptor byte list, honouring each descriptor's declared length,
/// and invoke `cb(lead, data)` for every large descriptor, where `lead` is the
/// full lead byte (bit7 set) and `data` is the payload after the three-byte
/// header. `cb` returns true to stop the walk early. Small descriptors are
/// skipped by their embedded length; an EndTag or any truncation ends the walk.
pub(super) fn walk_large_descriptors(bytes: &[u8], mut cb: impl FnMut(u8, &[u8]) -> bool) {
    let mut i = 0usize;
    let mut steps = 0usize;

    while i < bytes.len() {
        steps += 1;
        if steps > MAX_DESCRIPTORS {
            return;
        }

        let lead = bytes[i];
        if lead == END_TAG {
            return;
        }

        if lead & LARGE_BIT != 0 {
            // Large descriptor: lead, len_lo, len_hi, then `len` data bytes.
            let len_lo = match bytes.get(i + 1) {
                Some(v) => *v as usize,
                None => return,
            };
            let len_hi = match bytes.get(i + 2) {
                Some(v) => *v as usize,
                None => return,
            };
            let data_len = len_lo | (len_hi << 8);
            let data_start = i + 3;
            let data_end = match data_start.checked_add(data_len) {
                Some(e) if e <= bytes.len() => e,
                _ => return,
            };
            if cb(lead, &bytes[data_start..data_end]) {
                return;
            }
            i = data_end;
        } else {
            // Small descriptor: lead byte encodes a 3-bit length in bits 0..2.
            let small_len = (lead & 0x07) as usize;
            i = match i.checked_add(1 + small_len) {
                Some(next) if next > i => next,
                _ => return,
            };
        }
    }
}

/// Byte-pattern fallback: scan every offset for a large descriptor lead byte
/// and invoke `cb(lead, data)` when its declared length fits the region. `cb`
/// returns true to stop. This is resilient against a Buffer-wrapper offset
/// quirk because each large descriptor carries its own length and a
/// distinguishing type byte, so a validated lead can be trusted.
pub(super) fn scan_large_descriptors(region: &[u8], mut cb: impl FnMut(u8, &[u8]) -> bool) {
    let mut i = 0usize;
    let mut steps = 0usize;

    while i + 3 <= region.len() {
        steps += 1;
        if steps > MAX_DESCRIPTORS {
            return;
        }

        let lead = region[i];
        if lead & LARGE_BIT != 0 {
            let len_lo = region[i + 1] as usize;
            let len_hi = region[i + 2] as usize;
            let data_len = len_lo | (len_hi << 8);
            let data_start = i + 3;
            if let Some(data_end) = data_start.checked_add(data_len) {
                if data_end <= region.len() && cb(lead, &region[data_start..data_end]) {
                    return;
                }
            }
        }
        i += 1;
    }
}

/// Apply one large descriptor to the touchpad's fields: the I2C slave address
/// from a SerialBus descriptor and the GPIO pin from a GpioInt descriptor.
fn fill_touchpad_descriptor(lead: u8, data: &[u8], dev: &mut I2cHidDevice) {
    match lead {
        LEAD_SERIAL_BUS => {
            if dev.slave_addr == 0 {
                if let Some(addr) = parse_i2c_serial_bus(data) {
                    dev.slave_addr = addr;
                    // The controller name must come from the same fragment
                    // that supplied the address: a body scan can walk several
                    // ResourceTemplate fragments (multi-SKU firmware keeps
                    // alternatives side by side), and pairing the address from
                    // one with the ResourceSource of another names the wrong
                    // bus.
                    if let Some(ctrl) = parse_i2c_controller(data) {
                        dev.controller = ctrl;
                    }
                }
            }
        }
        LEAD_GPIO => {
            if !dev.has_gpio {
                if let Some(pin) = parse_gpio_int(data) {
                    dev.gpio_pin = pin;
                    dev.has_gpio = true;
                    // Same-fragment rule as the I2C controller name above: the
                    // community name must come from the descriptor that
                    // supplied the pin, or a multi-SKU body scan can pair the
                    // pin with another template's community.
                    if let Some(ctrl) = parse_gpio_controller(data) {
                        dev.gpio_controller = ctrl;
                    }
                }
            }
        }
        _ => {}
    }
}

/// Fill the touchpad's I2C address and GPIO pin from a length-delimited
/// descriptor list.
fn walk_descriptors(bytes: &[u8], dev: &mut I2cHidDevice) {
    walk_large_descriptors(bytes, |lead, data| {
        fill_touchpad_descriptor(lead, data, dev);
        false
    });
}

/// Strip the Buffer wrapper (`BufferOp PkgLength BufferSize`) off a `_CRS`
/// region and return the raw resource-descriptor byte slice.
///
/// The region may be a bare Buffer value (from `Name (_CRS, Buffer)`) or a
/// method body such as `Return (Buffer)` (from `Method (_CRS) { ... }`). We
/// therefore do not assume the BufferOp is the very first byte: we locate the
/// first BufferOp in the region and unwrap from there. If none is present we
/// treat the whole region as descriptors so the caller can still byte-scan it.
pub(super) fn resource_bytes(crs_value: &[u8]) -> Option<&[u8]> {
    let buf_at = find_buffer_op(crs_value)?;
    let buf = crs_value.get(buf_at..)?;
    // BufferOp PkgLength BufferSize <descriptors...>
    let (_pkg_len, len_bytes) = read_pkg_length(buf, 1)?;
    let after_pkg = 1 + len_bytes;
    // BufferSize is itself a TermArg integer: a small-integer constant here.
    let size_bytes = integer_encoding_len(buf.get(after_pkg..)?)?;
    let data_start = after_pkg + size_bytes;
    buf.get(data_start..)
}

/// Return the index of the first BufferOp (0x11) in `region`, capped so a large
/// slice cannot spin. A method body that returns a template starts with
/// `Return (0xA4) Buffer (0x11) ...`, so the BufferOp is only a byte or two in.
fn find_buffer_op(region: &[u8]) -> Option<usize> {
    if region.first().copied()? == BUFFER_OP {
        return Some(0);
    }
    let limit = region.len().min(8);
    for (idx, b) in region.iter().take(limit).enumerate() {
        if *b == BUFFER_OP {
            return Some(idx);
        }
    }
    None
}

/// Fill still-unset touchpad fields from a raw `_CRS` region by byte-scanning
/// for the I2cSerialBus (0x8E) and GpioInt (0x8C) descriptors directly.
fn scan_descriptors(region: &[u8], dev: &mut I2cHidDevice) {
    scan_large_descriptors(region, |lead, data| {
        fill_touchpad_descriptor(lead, data, dev);
        dev.slave_addr != 0 && dev.has_gpio
    });
}

/// Length in bytes of a leading AML integer constant encoding (ZeroOp, OneOp,
/// or ByteConst/WordConst/DWordConst). Used to skip the Buffer's size operand.
fn integer_encoding_len(data: &[u8]) -> Option<usize> {
    match data.first().copied()? {
        0x00 | 0x01 | 0xFF => Some(1), // ZeroOp, OneOp, OnesOp
        0x0A => Some(2),               // BytePrefix + 1 byte
        0x0B => Some(3),               // WordPrefix + 2 bytes
        0x0C => Some(5),               // DWordPrefix + 4 bytes
        0x0E => Some(9),               // QWordPrefix + 8 bytes
        _ => None,
    }
}

/// Parse an I2C Serial Bus large descriptor's data payload (the bytes after the
/// three-byte large header) and return the 7-bit slave address.
///
/// Data layout (ACPI 6.x, I2C variant), offsets relative to `data[0]`:
///   0  RevisionId
///   1  ResourceSourceIndex
///   2  SerialBusType (1 = I2C)
///   3  GeneralFlags
///   4  TypeSpecificFlags (2 bytes)
///   6  TypeSpecificRevisionId
///   7  TypeSpecificDataLength (2 bytes)
///   9  ConnectionSpeed (4 bytes)
///   13 SlaveAddress (2 bytes)
///   15 ResourceSource (string, controller path) ...
fn parse_i2c_serial_bus(data: &[u8]) -> Option<u16> {
    let bus_type = *data.get(2)?;
    if bus_type != SERIAL_BUS_TYPE_I2C {
        return None;
    }
    let lo = *data.get(13)? as u16;
    let hi = *data.get(14)? as u16;
    let raw = lo | (hi << 8);
    // Mask to the 7-bit address; 10-bit addressing is not modeled here.
    Some(raw & 0x7F)
}

/// Parse the trailing NameSeg of an I2cSerialBus descriptor's ResourceSource:
/// the ACPI name of the host controller the device sits on (for example
/// `I2C4`). The ResourceSource follows the type-specific data, whose length is
/// declared at offset 7, so its start is computed rather than assumed.
fn parse_i2c_controller(data: &[u8]) -> Option<[u8; 4]> {
    if *data.get(2)? != SERIAL_BUS_TYPE_I2C {
        return None;
    }
    let type_len = *data.get(7)? as usize | ((*data.get(8)? as usize) << 8);
    let src = data.get(9usize.checked_add(type_len)?..)?;
    source_name_seg(src)
}

/// Parse the trailing NameSeg of a GpioInt descriptor's ResourceSource: the
/// ACPI name of the GPIO community controller the pin belongs to (for example
/// `GPO0`). The layout continues the table above `parse_gpio_int`: the
/// ResourceSourceNameOffset is the u16 at `data[14..16]` (descriptor bytes
/// 17..19, after PinTableOffset at 14..16 and ResourceSourceIndex at 16) and,
/// like PinTableOffset, it is measured from the descriptor's lead byte, so the
/// three-byte large header is subtracted before indexing `data`.
fn parse_gpio_controller(data: &[u8]) -> Option<[u8; 4]> {
    if *data.get(1)? != GPIO_CONN_TYPE_INTERRUPT {
        return None;
    }
    let off_lo = *data.get(14)? as usize;
    let off_hi = *data.get(15)? as usize;
    let name_offset = off_lo | (off_hi << 8);
    let src = data.get(name_offset.checked_sub(3)?..)?;
    source_name_seg(src)
}

/// Extract the trailing NameSeg of a ResourceSource string region: the bytes up
/// to the first NUL terminator (or the whole region when none), split on the
/// last `.` of the ACPI path. None for an empty path.
fn source_name_seg(src: &[u8]) -> Option<[u8; 4]> {
    let end = src.iter().position(|&b| b == 0).unwrap_or(src.len());
    let path = &src[..end];
    if path.is_empty() {
        return None;
    }
    let seg_start = path.iter().rposition(|&b| b == b'.').map_or(0, |i| i + 1);
    let seg = &path[seg_start..];
    let mut out = [0u8; 4];
    let n = seg.len().min(4);
    out[..n].copy_from_slice(&seg[..n]);
    Some(out)
}

/// Parse a GPIO Connection large descriptor's data payload and return the first
/// interrupt pin number.
///
/// Data layout (ACPI 6.x GpioInt/GpioIo), offsets relative to `data[0]`:
///   0  RevisionId
///   1  GpioConnectionType (0 = Interrupt, 1 = IO)
///   2  GeneralFlags (2 bytes)
///   4  InterruptAndIoFlags (2 bytes)
///   6  PinConfiguration
///   7  OutputDriveStrength (2 bytes)
///   9  DebounceTimeout (2 bytes)
///   11 PinTableOffset (2 bytes)  -- relative to descriptor start (lead byte)
///   13 ResourceSourceIndex
///   14 ResourceSourceNameOffset (2 bytes)
///   16 VendorDataOffset (2 bytes)
///   18 VendorDataLength (2 bytes)
///
/// The pin table is an array of 16-bit pin numbers. PinTableOffset is measured
/// from the descriptor's lead byte, so we subtract the three-byte large header
/// to index into `data`.
fn parse_gpio_int(data: &[u8]) -> Option<u16> {
    let conn_type = *data.get(1)?;
    if conn_type != GPIO_CONN_TYPE_INTERRUPT {
        return None;
    }
    let off_lo = *data.get(11)? as usize;
    let off_hi = *data.get(12)? as usize;
    let pin_table_offset = off_lo | (off_hi << 8);
    // Offset counts from the lead byte; `data` starts three bytes past it.
    let idx = pin_table_offset.checked_sub(3)?;
    let lo = *data.get(idx)? as u16;
    let hi = *data.get(idx + 1)? as u16;
    Some(lo | (hi << 8))
}
