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

use super::{acpi, cpu, gdt, idt, port, serial, syscall, time, vga};

pub fn init() -> Result<(), &'static str> {
    gdt::init().map_err(|_| "GDT initialization failed")?;
    idt::init().map_err(|_| "IDT initialization failed")?;
    cpu::init().map_err(|_| "CPU initialization failed")?;
    port::init().map_err(|_| "Port initialization failed")?;
    serial::init().map_err(|_| "Serial initialization failed")?;
    vga::init().map_err(|_| "VGA initialization failed")?;
    time::init()?;
    syscall::init()?;
    Ok(())
}

pub fn init_with_acpi() -> Result<(), &'static str> {
    acpi::init().map_err(|_| "ACPI initialization failed")?;
    if let Some(hpet_addr) = acpi::hpet_address() {
        time::init_with_hpet(hpet_addr)?;
    }
    Ok(())
}

pub fn is_initialized() -> bool {
    gdt::is_initialized() && idt::is_initialized() && cpu::is_initialized()
}

#[derive(Debug, Clone)]
pub struct ArchStats {
    pub gdt: gdt::GdtStats,
    pub idt: idt::IdtStats,
    pub cpu: cpu::CpuStats,
    pub vga: vga::VgaStats,
}

pub fn get_stats() -> ArchStats {
    ArchStats {
        gdt: gdt::get_stats(),
        idt: idt::get_stats(),
        cpu: cpu::get_stats(),
        vga: vga::get_stats(),
    }
}
