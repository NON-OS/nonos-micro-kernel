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

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_ACPI, BUS_KIND_PCI};

use crate::constants::{device_info, INTEL_VENDOR_ID};

const MAX_DEVICES: usize = 128;
const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;
const ACPI_LPSS_FAMILY: &str = "acpi-lpss";
const CLASS_I2C_HID: u32 = 0x0041;

/// Upper bound on host controllers probed in one bring-up. Gemini Lake exposes
/// eight LPSS I2C functions; other platforms fewer.
pub const MAX_CONTROLLERS: usize = 8;

#[derive(Clone, Copy, Default)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub bar0_size: u64,
    pub pci_device: u16,
    pub clock_hz: u32,
    pub family: &'static str,
    pub is_acpi: bool,
}

/// Collect every LPSS I2C host controller with a usable MMIO window into `out`,
/// returning how many were written. The touchpad may hang off any one of them,
/// so the caller probes each rather than committing to the first.
pub fn find_controllers(out: &mut [Found]) -> usize {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return 0;
    }
    let mut count = 0;
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if count >= out.len() {
            break;
        }
        let Some((family, clock_hz, is_acpi)) = classify(r) else { continue };
        let bar0 = r.bars[0];
        if r.bar_count != 0 && bar0.kind == BAR_KIND_MMIO && bar0.size != 0 {
            out[count] = Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                bar0_size: bar0.size,
                pci_device: r.device,
                clock_hz,
                family,
                is_acpi,
            };
            count += 1;
        }
    }
    count
}

/// An ACPI-declared HID device: its 7-bit I2C address, used to probe which
/// controller actually answers it, the controller index the firmware named
/// in its `_CRS` ResourceSource (carried as index+1 in `pci_progif`, zero
/// when unnamed), and its interrupt: the GPIO pin (`irq_source`) plus the
/// GPIO community's `_UID`+1 (`irq_pin`, zero when unknown).
#[derive(Clone, Copy, Default)]
pub struct AcpiTouchpad {
    pub addr: u8,
    pub controller_idx: Option<u8>,
    pub gpio_pin: u16,
    pub gpio_community: Option<u8>,
}

/// Upper bound on ACPI HID candidates considered. Multi-SKU firmware declares
/// one device per possible pad; only the fitted one answers.
pub const MAX_TARGETS: usize = 4;

/// Collect every ACPI-declared i2c-HID device into `out`, returning how many
/// were written. Firmware for a chassis with several possible pads (HP ships
/// ELAN2513 and ELAN0712 variants of the same laptop) declares them all and
/// gates the real one behind `_STA`, which this kernel does not evaluate — so
/// the caller must treat each entry as a candidate and let the bus probe
/// decide, never just the first record.
pub fn find_touchpad_addrs(out: &mut [AcpiTouchpad]) -> usize {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return 0;
    }
    let mut count = 0;
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if count >= out.len() {
            break;
        }
        if r.bus_kind == BUS_KIND_ACPI && r.class == CLASS_I2C_HID && r.vendor != 0 {
            out[count] = AcpiTouchpad {
                addr: (r.vendor & 0x7F) as u8,
                controller_idx: r.pci_progif.checked_sub(1),
                gpio_pin: r.irq_source as u16,
                gpio_community: r.irq_pin.checked_sub(1),
            };
            count += 1;
        }
    }
    count
}

/// Locate the broker record of the GPIO community with the given `_UID`,
/// registered by the kernel from ACPI. Returns it as a `Found` so the claim
/// and MMIO-map helpers apply unchanged; `clock_hz`/`family` are meaningless
/// for a GPIO window and left at their defaults.
pub fn find_gpio_community(uid: u8) -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if r.bus_kind == BUS_KIND_ACPI
            && r.class == crate::constants::CLASS_GPIO_CTRL
            && r.device == u16::from(uid)
            && r.bar_count != 0
            && r.bars[0].kind == BAR_KIND_MMIO
            && r.bars[0].size != 0
        {
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                bar0_size: r.bars[0].size,
                pci_device: r.device,
                clock_hz: 0,
                family: "gpio-community",
                is_acpi: true,
            });
        }
    }
    None
}

// Recognise an LPSS I2C host controller on either bus. A PCI function matches by
// Intel vendor and a known device id; an ACPI device carries its serial-bus
// class and its source clock (in the bar's aux field) from the kernel's AML
// enumeration. The transfer engine polls, so a missing legacy IRQ line never
// disqualifies a controller.
//
// The PCI class is deliberately not gated: Intel LPSS reports the serial-bus
// class (0x0c) on Sunrise Point and later but the signal-processing class (0x11)
// on Apollo/Gemini Lake. The per-device table is the exact filter, so trust it.
fn classify(r: &DeviceRecord) -> Option<(&'static str, u32, bool)> {
    if r.bus_kind == BUS_KIND_PCI && r.vendor == INTEL_VENDOR_ID {
        if let Some((family, clock_hz)) = device_info(r.device) {
            return Some((family, clock_hz, false));
        }
    }
    if r.bus_kind == BUS_KIND_ACPI && r.pci_class == PCI_CLASS_SERIAL_BUS {
        return Some((ACPI_LPSS_FAMILY, r.bars[0].aux, true));
    }
    None
}
