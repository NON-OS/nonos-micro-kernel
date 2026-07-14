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

//! Bounded byte-scan of an AML stream to locate I2C-HID device scopes.
//!
//! This is deliberately NOT a full AML interpreter. It walks the raw bytecode
//! looking for the `Device (...)` opcode pattern, reads the enclosing package
//! length to bound the device body, then within that body finds the `_HID`
//! value (to decide whether the device is a touchpad we care about) and the
//! `_CRS` value (the resource template, handed to the crs module). Every index
//! is bounds-checked and every loop is iteration-capped.

use alloc::vec::Vec;

// AML opcodes we recognise.
const EXT_OP_PREFIX: u8 = 0x5B;
const DEVICE_OP: u8 = 0x82;
const NAME_OP: u8 = 0x08;
const METHOD_OP: u8 = 0x14;
const BUFFER_OP: u8 = 0x11;
const DWORD_PREFIX: u8 = 0x0C;
const WORD_PREFIX: u8 = 0x0B;
const BYTE_PREFIX: u8 = 0x0A;
const STRING_PREFIX: u8 = 0x0D;

/// Hard cap on the number of top-level opcodes inspected, so a malformed or
/// adversarial table cannot spin the scanner.
const MAX_STEPS: usize = 1_000_000;

/// A located device scope: the raw slice of its `_HID` value encoding and the
/// raw slice of its `_CRS` value encoding, both still in AML form.
pub struct DeviceScope<'a> {
    /// Decoded seven-character HID (EISAID expanded, or string bytes), padded
    /// with zero bytes to eight.
    pub hid: [u8; 8],
    /// The AML bytes of the `_CRS` value (typically a Buffer object), if the
    /// device declared one.
    pub crs: Option<&'a [u8]>,
    /// The whole device body. A `_CRS` method that returns concatenated named
    /// templates (`Return (ConcatenateResTemplate (SBFB, SBFG))`) carries no
    /// inline descriptors; the named `ResourceTemplate` buffers live here in the
    /// body, so this is the fallback region for the descriptor byte-scan.
    pub body: &'a [u8],
}

/// Scan an AML block and return the device scopes whose `_HID` matches a known
/// I2C-HID touchpad identifier.
pub fn find_touchpad_devices(aml: &[u8]) -> Vec<DeviceScope<'_>> {
    find_devices(aml, hid_is_touchpad)
}

/// Scan an AML block for `Device (...)` objects whose decoded `_HID` satisfies
/// `matcher`, returning a scope (the HID plus the raw `_CRS` bytes) for each.
pub(super) fn find_devices(aml: &[u8], matcher: fn(&[u8; 8]) -> bool) -> Vec<DeviceScope<'_>> {
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut steps = 0usize;

    while i + 1 < aml.len() {
        steps += 1;
        if steps > MAX_STEPS {
            break;
        }

        if aml[i] == EXT_OP_PREFIX && aml[i + 1] == DEVICE_OP {
            // ExtOpPrefix DeviceOp PkgLength NameString ...body...
            let body_start = i + 2;
            if let Some((pkg_len, len_bytes)) = read_pkg_length(aml, body_start) {
                // PkgLength counts from the first byte of the length encoding
                // itself to the end of the package.
                let pkg_end = body_start.checked_add(pkg_len).unwrap_or(aml.len());
                let pkg_end = pkg_end.min(aml.len());
                let after_len = body_start + len_bytes;

                if let Some(name_end) = skip_name_string(aml, after_len) {
                    if name_end <= pkg_end {
                        if let Some(scope) = inspect_device_body(&aml[name_end..pkg_end], matcher) {
                            out.push(scope);
                        }
                        // Descend into the package rather than skipping it: a
                        // bus controller's Device wraps its children, and each
                        // child must be inspected as its own scope with its own
                        // body, not blended into the parent's.
                        i = name_end;
                        continue;
                    }
                }

                // Advance past this whole device package. Guard against a zero
                // or degenerate length that would fail to make progress.
                if pkg_end > i {
                    i = pkg_end;
                    continue;
                }
            }
        }

        i += 1;
    }

    out
}

/// Within a device body, find `_HID` and `_CRS` and, if the HID satisfies
/// `matcher`, build a `DeviceScope`. Returns `None` when there is no matching
/// `_HID`.
fn inspect_device_body(body: &[u8], matcher: fn(&[u8; 8]) -> bool) -> Option<DeviceScope<'_>> {
    // Only the device's own objects count: everything up to its first nested
    // Device. Searching the whole package would let a controller Device with
    // no _HID of its own match on its first HID child, and would then pair
    // that child's identity with _CRS descriptors from any sibling.
    let own = &body[..first_nested_device(body)];
    let hid = find_named_hid(own)?;
    if !matcher(&hid) {
        return None;
    }
    let crs = find_named_crs(own);
    Some(DeviceScope { hid, crs, body: own })
}

/// Offset of the first nested `Device (...)` opcode in a device body, or the
/// body length when there is none (a leaf device).
fn first_nested_device(body: &[u8]) -> usize {
    let mut i = 0usize;
    while i + 1 < body.len() {
        if body[i] == EXT_OP_PREFIX && body[i + 1] == DEVICE_OP {
            return i;
        }
        i += 1;
    }
    body.len()
}

/// Locate a `Name (_HID, value)` in `body` and decode the value into a padded
/// seven-character identifier.
fn find_named_hid(body: &[u8]) -> Option<[u8; 8]> {
    let value_at = find_name_value(body, b"_HID")?;
    decode_hid_value(&body[value_at..])
}

/// Locate the device's `_CRS` and return a byte slice covering its resource
/// template. `_CRS` may be declared two ways in practice:
///
///   * `Name (_CRS, Buffer(){...})` — a static ResourceTemplate. The value is a
///     BufferOp and we bound the slice to its declared package length.
///   * `Method (_CRS, 0) { Return (ResourceTemplate(){...}) }` — a method that
///     immediately returns a static buffer. We cannot evaluate AML, but the
///     overwhelmingly common shape is a single `Return(Buffer)`, so we hand the
///     method body to the caller and let the crs walker locate the buffer (or,
///     failing that, byte-scan for the descriptors directly).
///
/// The ELAN2513 and most modern touchpads use the Method form, which the old
/// Name-only lookup missed entirely (hence a zero slave address). Returning the
/// method body preserves the byte-pattern fallback in the crs parser.
fn find_named_crs(body: &[u8]) -> Option<&[u8]> {
    // Prefer a static Name (_CRS, Buffer).
    if let Some(value_at) = find_name_value(body, b"_CRS") {
        if let Some(rest) = body.get(value_at..) {
            if rest.first().copied() == Some(BUFFER_OP) {
                if let Some((pkg_len, _len_bytes)) = read_pkg_length(rest, 1) {
                    // Length is measured from the first PkgLength byte to the
                    // end of the buffer object.
                    let end = 1usize.checked_add(pkg_len).unwrap_or(rest.len()).min(rest.len());
                    if let Some(slice) = rest.get(..end) {
                        return Some(slice);
                    }
                }
            }
            // Non-buffer value (unexpected): hand it over as-is for scanning.
            return Some(rest);
        }
    }

    // Fall back to Method (_CRS, ...) { ... Return(Buffer) ... }. Return the
    // whole method body; the crs parser will find the first BufferOp inside it,
    // or byte-scan for the I2cSerialBus / GpioInt lead bytes.
    find_method_body(body, b"_CRS")
}

/// Locate a `Method (_CRS, ...)` and return the slice of its body (everything
/// after the flags byte, bounded by the method's package length). The body of a
/// resource method is typically `Return (ResourceTemplate(){...})`, but we do
/// not require that here; the caller scans the returned bytes.
fn find_method_body<'a>(body: &'a [u8], target: &[u8; 4]) -> Option<&'a [u8]> {
    let mut i = 0usize;
    let mut steps = 0usize;
    while i < body.len() {
        steps += 1;
        if steps > MAX_STEPS {
            return None;
        }
        if body[i] == METHOD_OP {
            // MethodOp PkgLength NameString MethodFlags TermList
            if let Some((pkg_len, len_bytes)) = read_pkg_length(body, i + 1) {
                let pkg_start = i + 1;
                let pkg_end = pkg_start.checked_add(pkg_len).unwrap_or(body.len()).min(body.len());
                let after_len = pkg_start + len_bytes;
                if let Some(name_end) = skip_name_string(body, after_len) {
                    if name_end >= 4
                        && name_end <= pkg_end
                        && &body[name_end - 4..name_end] == target
                    {
                        // Skip the one-byte MethodFlags to reach the TermList.
                        let term_start = name_end + 1;
                        if term_start <= pkg_end {
                            return body.get(term_start..pkg_end);
                        }
                    }
                }
            }
        }
        i += 1;
    }
    None
}

/// Scan `body` for a `NameOp NameString(==target) value` pattern and return the
/// index of the value's first byte. The four-character `target` must be the
/// trailing NameSeg of the NameString.
fn find_name_value(body: &[u8], target: &[u8; 4]) -> Option<usize> {
    let mut i = 0usize;
    let mut steps = 0usize;
    while i < body.len() {
        steps += 1;
        if steps > MAX_STEPS {
            return None;
        }
        if body[i] == NAME_OP {
            let name_start = i + 1;
            if let Some(name_end) = skip_name_string(body, name_start) {
                // The final NameSeg is the last four bytes consumed by the
                // NameString. Compare it against the target.
                if name_end >= 4 && &body[name_end - 4..name_end] == target {
                    return Some(name_end);
                }
            }
        }
        i += 1;
    }
    None
}

/// Decode an AML `_HID` value into a padded seven-character identifier.
///
/// Integers are treated as EISAID-compressed and expanded. Strings are copied
/// verbatim (first eight bytes). Returns `None` for encodings we do not model.
fn decode_hid_value(value: &[u8]) -> Option<[u8; 8]> {
    let op = value.first().copied()?;
    match op {
        DWORD_PREFIX => {
            let raw = value.get(1..5)?;
            let bytes = [raw[0], raw[1], raw[2], raw[3]];
            Some(expand_eisaid(bytes))
        }
        WORD_PREFIX => {
            // A word EISAID is unusual but decode defensively as a 4-byte id
            // with the high half zeroed.
            let raw = value.get(1..3)?;
            let bytes = [raw[0], raw[1], 0, 0];
            Some(expand_eisaid(bytes))
        }
        BYTE_PREFIX => {
            let raw = value.get(1..2)?;
            Some(expand_eisaid([raw[0], 0, 0, 0]))
        }
        STRING_PREFIX => {
            // StringPrefix ASCII... 0x00
            let mut out = [0u8; 8];
            let mut n = 0usize;
            let mut j = 1usize;
            while j < value.len() && n < out.len() {
                let c = value[j];
                if c == 0 {
                    break;
                }
                out[n] = c;
                n += 1;
                j += 1;
            }
            if n == 0 {
                None
            } else {
                Some(out)
            }
        }
        _ => None,
    }
}

/// Expand a 4-byte EISAID (as stored little-endian-agnostic big-endian pair per
/// the ACPI spec) into a padded seven-character ASCII identifier such as
/// "PNP0C50". Bytes 0..2 encode three manufacturer letters; bytes 2..4 encode
/// four hex product digits.
fn expand_eisaid(b: [u8; 4]) -> [u8; 8] {
    let mut out = [0u8; 8];
    // Manufacturer: three 5-bit letters, '@'-based (so 1 => 'A').
    out[0] = ((b[0] >> 2) & 0x1F) + b'@';
    out[1] = (((b[0] & 0x03) << 3) | (b[1] >> 5)) + b'@';
    out[2] = (b[1] & 0x1F) + b'@';
    // Product: four hex digits from the high/low nibbles of bytes 2 and 3.
    out[3] = hex_digit(b[2] >> 4);
    out[4] = hex_digit(b[2] & 0x0F);
    out[5] = hex_digit(b[3] >> 4);
    out[6] = hex_digit(b[3] & 0x0F);
    out[7] = 0;
    out
}

fn hex_digit(nibble: u8) -> u8 {
    let n = nibble & 0x0F;
    if n < 10 {
        b'0' + n
    } else {
        b'A' + (n - 10)
    }
}

/// True when the decoded seven-character HID is a known I2C-HID touchpad
/// identifier: the generic HID-over-I2C IDs, or a common touchpad vendor
/// prefix.
fn hid_is_touchpad(hid: &[u8; 8]) -> bool {
    // The generic HID-over-I2C identifier is the seven-character EISAID
    // "PNP0C50". Some firmware declares the eight-character string "ACPI0C50".
    if &hid[..7] == b"PNP0C50" || hid == b"ACPI0C50" {
        return true;
    }
    // Common touchpad vendor prefixes on the manufacturer field.
    let prefixes: [&[u8]; 4] = [b"ELAN", b"SYNA", b"FTE", b"CYAP"];
    for p in prefixes.iter() {
        if &hid[..p.len()] == *p {
            return true;
        }
    }
    false
}

/// Decode an AML PkgLength beginning at `at`. Returns the total package length
/// (including the length-encoding bytes) and the number of bytes the encoding
/// itself occupied. Returns `None` on truncation.
///
/// PkgLength: the top two bits of the lead byte give the number of following
/// bytes (0..3). When zero following bytes, the low six bits are the length.
/// Otherwise the low four bits of the lead byte are the low nibble and each
/// following byte contributes the next eight bits, little-endian.
pub fn read_pkg_length(data: &[u8], at: usize) -> Option<(usize, usize)> {
    let lead = *data.get(at)?;
    let follow = (lead >> 6) as usize;
    if follow == 0 {
        return Some(((lead & 0x3F) as usize, 1));
    }
    let mut len = (lead & 0x0F) as usize;
    for k in 0..follow {
        let byte = *data.get(at + 1 + k)? as usize;
        len |= byte << (4 + 8 * k);
    }
    Some((len, 1 + follow))
}

/// Skip an AML NameString starting at `at`, returning the index just past it.
///
/// A NameString is: an optional RootChar '\\' (0x5C) or one-or-more
/// ParentPrefix '^' (0x5E), then a name path which is one of a NullName (0x00),
/// a DualNamePrefix (0x2E) followed by two NameSegs, a MultiNamePrefix (0x2F)
/// followed by a segment count and that many NameSegs, or a single NameSeg.
/// Each NameSeg is exactly four bytes.
pub fn skip_name_string(data: &[u8], at: usize) -> Option<usize> {
    let mut i = at;
    // Root and parent prefixes.
    if data.get(i).copied()? == 0x5C {
        i += 1;
    } else {
        while data.get(i).copied() == Some(0x5E) {
            i += 1;
        }
    }

    match data.get(i).copied()? {
        0x00 => {
            // NullName.
            Some(i + 1)
        }
        0x2E => {
            // DualNamePrefix: two NameSegs follow.
            let end = i + 1 + 8;
            if end <= data.len() {
                Some(end)
            } else {
                None
            }
        }
        0x2F => {
            // MultiNamePrefix: next byte is the seg count, then count*4 bytes.
            let count = data.get(i + 1).copied()? as usize;
            let end = i + 2 + count.checked_mul(4)?;
            if end <= data.len() {
                Some(end)
            } else {
                None
            }
        }
        _ => {
            // Single NameSeg: four bytes.
            let end = i + 4;
            if end <= data.len() {
                Some(end)
            } else {
                None
            }
        }
    }
}
