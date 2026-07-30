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

use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::fdt::find::gic::{find as find_gic, GicVersion};
use crate::arch::fdt::find::pci::find as find_pci;
use crate::arch::fdt::find::rtc::find as find_rtc;
use crate::arch::fdt::find::timer::find as find_timer;
use crate::arch::fdt::find::uart::{find as find_uart, UartKind};
use crate::arch::fdt::Fdt;

pub fn populate(fdt: &Fdt, info: &mut BootInfo) {
    populate_uart(fdt, info);
    populate_gic(fdt, info);
    populate_timer(fdt, info);
    populate_pci(fdt, info);
    populate_rtc(fdt, info);
}

fn populate_uart(fdt: &Fdt, info: &mut BootInfo) {
    if let Ok(Some(u)) = find_uart(fdt) {
        if u.kind == UartKind::Pl011 {
            info.uart_base = u.base;
        }
    }
}

fn populate_gic(fdt: &Fdt, info: &mut BootInfo) {
    if let Ok(Some(g)) = find_gic(fdt) {
        info.gic_dist_base = g.dist_base;
        if g.version == GicVersion::V3 {
            info.gic_redist_base = g.redist_or_cpu_base;
        } else {
            info.gic_unsupported = true;
        }
    }
}

fn populate_timer(fdt: &Fdt, info: &mut BootInfo) {
    if let Ok(Some(t)) = find_timer(fdt) {
        info.timer_phys_intid = t.nonsecure_phys_intid;
        info.timer_virt_intid = t.virtual_intid;
    }
}

fn populate_pci(fdt: &Fdt, info: &mut BootInfo) {
    if let Ok(Some(p)) = find_pci(fdt) {
        info.pci_ecam_base = p.ecam_base;
        info.pci_ecam_size = p.ecam_size;
        info.pci_io_cpu_base = p.io_cpu_base;
        info.pci_io_port_base = p.io_port_base;
        info.pci_io_size = p.io_size;
    }
}

fn populate_rtc(fdt: &Fdt, info: &mut BootInfo) {
    if let Ok(Some(base)) = find_rtc(fdt) {
        info.rtc_base = base;
    }
}
