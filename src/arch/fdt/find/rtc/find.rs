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

//! Locating a PL031 real-time clock.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;
use crate::arch::fdt::walker::Event;

const PL031: &[u8] = b"arm,pl031";

/// The MMIO base of the first PL031 in the tree, or `None` for a board with
/// no real-time clock.
pub fn find(fdt: &Fdt) -> Result<Option<u64>, FdtError> {
    let mut walker = fdt.walker();
    let mut depth: i32 = -1;
    let mut address_cells: u32 = 2;
    let mut size_cells: u32 = 1;
    let mut matched = false;
    let mut base: Option<u64> = None;

    while let Some(event) = walker.next()? {
        match event {
            Event::BeginNode { .. } => {
                depth += 1;
                if depth == 1 {
                    matched = false;
                    base = None;
                }
            }
            Event::EndNode => {
                if depth == 1 && matched {
                    if let Some(found) = base {
                        return Ok(Some(found));
                    }
                }
                depth -= 1;
            }
            Event::Property(prop) => {
                if depth == 0 {
                    if prop.name == b"#address-cells" {
                        address_cells = prop.u32()?;
                    } else if prop.name == b"#size-cells" {
                        size_cells = prop.u32()?;
                    }
                } else if depth == 1 {
                    if prop.name == b"compatible" {
                        matched = prop.compatible_matches(PL031);
                    } else if prop.name == b"reg" {
                        base = prop.reg_iter(address_cells, size_cells).next().map(|(b, _)| b);
                    }
                }
            }
        }
    }
    Ok(None)
}
