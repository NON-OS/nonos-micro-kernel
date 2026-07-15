// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the `_CRS` decoders: the touchpad I2C slave address and the
//! controller MMIO window and interrupt, plus a fuzz sweep for panic freedom.

use crate::arch::x86_64::acpi::aml::controller::parse_controller_crs;
use crate::arch::x86_64::acpi::aml::crs::parse_crs;
use crate::arch::x86_64::acpi::aml::types::{I2cHidDevice, LpssController};
use crate::fixtures::{crs_buffer, ext_interrupt, i2c_serial_bus, memory32_fixed};

use alloc::vec::Vec;

#[test]
fn touchpad_crs_yields_the_i2c_slave_address() {
    let crs = crs_buffer(&i2c_serial_bus(0x15));
    let mut dev = I2cHidDevice::new([0; 8]);
    parse_crs(&crs, &mut dev);
    assert_eq!(dev.slave_addr, 0x15);
}

#[test]
fn controller_crs_yields_mmio_window_and_irq() {
    let mut desc = memory32_fixed(0xFE10_2000, 0x1000);
    desc.extend_from_slice(&ext_interrupt(27));
    let mut ctl = LpssController::new([0; 8]);
    parse_controller_crs(&crs_buffer(&desc), &mut ctl);
    assert_eq!(ctl.mmio_base, 0xFE10_2000);
    assert_eq!(ctl.mmio_size, 0x1000);
    assert!(ctl.has_irq);
    assert_eq!(ctl.irq, 27);
    assert!(ctl.is_valid());
}

#[test]
fn controller_without_a_memory_window_is_not_usable() {
    let mut ctl = LpssController::new([0; 8]);
    parse_controller_crs(&crs_buffer(&ext_interrupt(9)), &mut ctl);
    assert!(ctl.has_irq);
    assert!(!ctl.is_valid());
}

#[test]
fn descriptor_order_does_not_matter() {
    let mut desc = ext_interrupt(11);
    desc.extend_from_slice(&memory32_fixed(0xC000_0000, 0x2000));
    let mut ctl = LpssController::new([0; 8]);
    parse_controller_crs(&crs_buffer(&desc), &mut ctl);
    assert_eq!(ctl.mmio_base, 0xC000_0000);
    assert_eq!(ctl.irq, 11);
}

#[test]
fn parsing_never_panics_on_hostile_bytes() {
    fn xs(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }
    for seed in 1..100_000u64 {
        let mut s = seed;
        let len = (xs(&mut s) % 64) as usize;
        let raw: Vec<u8> = (0..len).map(|_| (xs(&mut s) & 0xff) as u8).collect();
        let mut ctl = LpssController::new([0; 8]);
        parse_controller_crs(&raw, &mut ctl);
        let mut dev = I2cHidDevice::new([0; 8]);
        parse_crs(&raw, &mut dev);
    }
}
