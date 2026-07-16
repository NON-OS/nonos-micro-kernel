// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the AML device scan: a controller and a touchpad are each found
//! by their own `_HID` and never confused, and the scan is panic-free on
//! arbitrary bytes.

use crate::arch::x86_64::acpi::aml::controller::{
    find_i2c_controller_devices, parse_controller_crs,
};
use crate::arch::x86_64::acpi::aml::scan::find_touchpad_devices;
use crate::arch::x86_64::acpi::aml::types::LpssController;
use crate::fixtures::{crs_buffer, device, ext_interrupt, i2c_serial_bus, memory32_fixed};

use alloc::vec::Vec;

// EISAID compressions of the identifiers used below.
const INT33C3: [u8; 4] = [0x25, 0xD4, 0x33, 0xC3];
const PNP0C50: [u8; 4] = [0x41, 0xD0, 0x0C, 0x50];

fn controller_crs() -> Vec<u8> {
    let mut desc = memory32_fixed(0xFE10_2000, 0x1000);
    desc.extend_from_slice(&ext_interrupt(27));
    crs_buffer(&desc)
}

#[test]
fn a_controller_device_is_found_with_its_mmio_and_irq() {
    let dev = device(b"I2C1", INT33C3, &controller_crs());
    let found = find_i2c_controller_devices(&dev);
    assert_eq!(found.len(), 1);
    assert_eq!(&found[0].hid[..7], b"INT33C3");

    let mut ctl = LpssController::new(found[0].hid);
    parse_controller_crs(found[0].crs.unwrap(), &mut ctl);
    assert_eq!(ctl.mmio_base, 0xFE10_2000);
    assert_eq!(ctl.irq, 27);
}

#[test]
fn a_touchpad_is_not_mistaken_for_a_controller() {
    let dev = device(b"TPD0", PNP0C50, &crs_buffer(&i2c_serial_bus(0x15)));
    assert!(find_i2c_controller_devices(&dev).is_empty());
    let tp = find_touchpad_devices(&dev);
    assert_eq!(tp.len(), 1);
    assert_eq!(&tp[0].hid[..7], b"PNP0C50");
}

#[test]
fn a_controller_is_not_mistaken_for_a_touchpad() {
    let dev = device(b"I2C1", INT33C3, &controller_crs());
    assert!(find_touchpad_devices(&dev).is_empty());
}

#[test]
fn scanning_never_panics_on_hostile_bytes() {
    fn xs(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }
    for seed in 1..100_000u64 {
        let mut s = seed;
        let len = (xs(&mut s) % 96) as usize;
        let raw: Vec<u8> = (0..len).map(|_| (xs(&mut s) & 0xff) as u8).collect();
        let _ = find_i2c_controller_devices(&raw);
        let _ = find_touchpad_devices(&raw);
    }
}
