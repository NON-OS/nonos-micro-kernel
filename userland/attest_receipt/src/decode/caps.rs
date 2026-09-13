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

//! Capability masks by name, using the kernel's own table.

use crate::kernel::capability::Capability;

/// Names every bit the mask grants.
///
/// Bits this build does not define are reported, not dropped. An unrecognised
/// grant is the one a reader most needs to see.
pub fn capability_names(mask: u64) -> Vec<String> {
    let all = Capability::all();
    let defined = all.iter().fold(0u64, |acc, c| acc | c.bit());
    let mut out: Vec<String> =
        all.iter().filter(|c| mask & c.bit() != 0).map(|c| c.as_str().to_string()).collect();
    if mask & !defined != 0 {
        out.push(format!("unrecognised bits {:#x}", mask & !defined));
    }
    out
}

/// Whether the mask lets the capsule open a socket.
pub fn reaches_network(mask: u64) -> bool {
    mask & Capability::Network.bit() != 0
}
