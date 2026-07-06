// NONOS Operating System (AGPL-3.0-or-later)
// The real RX ring primitives (pub(super) in the driver; wrapped for the
// proofs).
#[path = "../../../capsule_driver_rtl8139/src/rx/copy_ring.rs"]
mod copy_ring;
#[path = "../../../capsule_driver_rtl8139/src/rx/ring_u16.rs"]
mod ring_u16;
#[path = "../../../capsule_driver_rtl8139/src/rx/ring_u8.rs"]
mod ring_u8;

pub fn u8_at(base: u64, off: usize) -> u8 {
    ring_u8::ring_u8(base, off)
}

pub fn u16_at(base: u64, off: usize) -> u16 {
    ring_u16::ring_u16(base, off)
}

pub fn copy(base: u64, start: usize, out: &mut [u8], len: usize) {
    copy_ring::copy_ring(base, start, out, len)
}
