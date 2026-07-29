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

//! Locating the PCI host bridge and its windows.
//!
//! The bridge is identified by `device_type = "pci"` rather than by matching a
//! compatible string, because the set of host-bridge compatibles is open ended
//! while that property is required of every one of them.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;
use crate::arch::fdt::walker::Event;

use super::info::PciHost;
use super::ranges::{iter as range_iter, SPACE_IO};

pub fn find(fdt: &Fdt) -> Result<Option<PciHost>, FdtError> {
    let mut walker = fdt.walker();
    let mut depth: i32 = -1;
    let mut root_address_cells: u32 = 2;
    let mut root_size_cells: u32 = 1;
    let mut is_pci = false;
    let mut host = PciHost::default();

    while let Some(event) = walker.next()? {
        match event {
            Event::BeginNode { .. } => {
                depth += 1;
                if depth == 1 {
                    is_pci = false;
                    host = PciHost::default();
                }
            }
            Event::EndNode => {
                if depth == 1 {
                    if is_pci {
                        return Ok(Some(host));
                    }
                    is_pci = false;
                }
                depth -= 1;
            }
            Event::Property(prop) => {
                if depth == 0 {
                    if prop.name == b"#address-cells" {
                        root_address_cells = prop.u32()?;
                    } else if prop.name == b"#size-cells" {
                        root_size_cells = prop.u32()?;
                    }
                } else if depth == 1 {
                    if prop.name == b"device_type" {
                        is_pci = prop.data.starts_with(b"pci\0");
                    } else if prop.name == b"reg" {
                        if let Some((base, size)) =
                            prop.reg_iter(root_address_cells, root_size_cells).next()
                        {
                            host.ecam_base = base;
                            host.ecam_size = size;
                        }
                    } else if prop.name == b"ranges" {
                        take_io_window(&mut host, prop.data, root_address_cells);
                    }
                }
            }
        }
    }
    Ok(None)
}

/// Record the first I/O range the bridge advertises. A bridge may list several
/// memory windows but conventionally has at most one I/O window.
fn take_io_window(host: &mut PciHost, data: &[u8], root_address_cells: u32) {
    for range in range_iter(data, root_address_cells) {
        if range.space == SPACE_IO && range.size != 0 {
            host.io_cpu_base = range.parent;
            host.io_port_base = range.child;
            host.io_size = range.size;
            return;
        }
    }
}
