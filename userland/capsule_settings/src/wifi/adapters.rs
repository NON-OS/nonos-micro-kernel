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

//! Live adapter discovery for the WiFi panel. The kernel broker table is
//! populated from the boot PCI scan, so `mk_device_list` returns a record for
//! every device present; this filters that real snapshot for wireless NICs by
//! PCI class. On hardware with a wireless card the panel lists it; on a machine
//! without one the list is simply empty. No device is claimed or driven here,
//! so enumerating stays a read-only capability (`DeviceEnum`).

use nonos_libc::{mk_device_list, DeviceRecord};

use super::interface::{discover, DeviceView, WifiInterface};

/// The broker table is small; this bounds the enumeration buffer.
const MAX_DEVICES: usize = 64;

/// Fill `out` with the WiFi adapters currently present, returning how many were
/// written. Reads the broker device table over `mk_device_list` and applies the
/// pure `discover` filter, so the class test stays host-testable.
pub fn scan_adapters(out: &mut [WifiInterface]) -> usize {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return 0;
    }
    let got = core::cmp::min(n as usize, MAX_DEVICES);
    let mut views = [DeviceView {
        device_id: 0,
        bus_kind: 0,
        pci_class: 0,
        pci_subclass: 0,
        vendor: 0,
        device: 0,
    }; MAX_DEVICES];
    for (view, rec) in views[..got].iter_mut().zip(buf[..got].iter()) {
        *view = DeviceView {
            device_id: rec.device_id,
            bus_kind: rec.bus_kind,
            pci_class: rec.pci_class,
            pci_subclass: rec.pci_subclass,
            vendor: rec.vendor,
            device: rec.device,
        };
    }
    discover(&views[..got], out)
}
