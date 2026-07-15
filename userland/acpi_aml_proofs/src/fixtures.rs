// NONOS Operating System (AGPL-3.0-or-later)
//! Byte builders that assemble spec-shaped ACPI resource descriptors and AML
//! device scopes, so the proofs feed the real parsers exactly what firmware
//! emits.

use alloc::vec::Vec;

/// Wrap descriptor bytes in a `_CRS` Buffer: BufferOp, single-byte PkgLength,
/// a ByteConst BufferSize, the descriptors, then the EndTag and its checksum.
pub fn crs_buffer(desc: &[u8]) -> Vec<u8> {
    let mut inner = desc.to_vec();
    inner.push(0x79);
    inner.push(0x00);
    let size = inner.len();
    let mut out = alloc::vec![0x11u8, (1 + 2 + size) as u8, 0x0A, size as u8];
    out.extend_from_slice(&inner);
    out
}

fn large_descriptor(lead: u8, data: &[u8]) -> Vec<u8> {
    let mut d = alloc::vec![lead];
    d.extend_from_slice(&(data.len() as u16).to_le_bytes());
    d.extend_from_slice(data);
    d
}

/// An I2cSerialBus descriptor (0x8E) carrying `addr` at the SlaveAddress field.
pub fn i2c_serial_bus(addr: u16) -> Vec<u8> {
    let mut data = alloc::vec![0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00];
    data.extend_from_slice(&400_000u32.to_le_bytes());
    data.extend_from_slice(&addr.to_le_bytes());
    data.extend_from_slice(b"\\I2C1");
    large_descriptor(0x8E, &data)
}

/// A Memory32Fixed descriptor (0x86) with the given base and length.
pub fn memory32_fixed(base: u32, len: u32) -> Vec<u8> {
    let mut data = alloc::vec![0x01u8];
    data.extend_from_slice(&base.to_le_bytes());
    data.extend_from_slice(&len.to_le_bytes());
    large_descriptor(0x86, &data)
}

/// An Extended Interrupt descriptor (0x89) declaring a single interrupt.
pub fn ext_interrupt(irq: u32) -> Vec<u8> {
    let mut data = alloc::vec![0x01u8, 0x01];
    data.extend_from_slice(&irq.to_le_bytes());
    large_descriptor(0x89, &data)
}

/// A `Device (name) { Name(_HID, EisaId), Name(_CRS, crs) }` AML object.
pub fn device(name: &[u8; 4], eisaid: [u8; 4], crs: &[u8]) -> Vec<u8> {
    let mut body = alloc::vec![0x08u8, b'_', b'H', b'I', b'D', 0x0C];
    body.extend_from_slice(&eisaid);
    body.extend_from_slice(&[0x08, b'_', b'C', b'R', b'S']);
    body.extend_from_slice(crs);

    let mut inner = name.to_vec();
    inner.extend_from_slice(&body);
    let mut out = alloc::vec![0x5Bu8, 0x82, (inner.len() + 1) as u8];
    out.extend_from_slice(&inner);
    out
}
