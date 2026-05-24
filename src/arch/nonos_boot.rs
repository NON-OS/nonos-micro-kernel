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

use crate::arch::x86_64::acpi;
use crate::arch::x86_64::cpu;
use crate::arch::x86_64::gdt;
use crate::arch::x86_64::idt;
use crate::arch::x86_64::multiboot;
use crate::arch::x86_64::pci;
use crate::arch::x86_64::serial;

pub fn init_early() -> Result<(), &'static str> {
    cpu::init().map_err(|_| "cpu init failed")?;
    gdt::init().map_err(|_| "gdt init failed")?;
    idt::init().map_err(|_| "idt init failed")?;
    acpi::init().map_err(|_| "acpi init failed")?;
    multiboot::init().map_err(|_| "multiboot init failed")?;
    serial::init().map_err(|_| "serial init failed")?;
    pci::init().map_err(|_| "pci init failed")?;
    Ok(())
}
