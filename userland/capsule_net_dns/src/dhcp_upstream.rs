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

mod lookup;
mod status;

use crate::state::set_upstream;

pub const DHCP_MAGIC: u32 = 0x4E44_4843;
pub const OP_LEASE_STATUS: u16 = 3;
pub const SERVICE: &[u8] = b"net.dhcp.client";

pub fn apply() {
    let Some(port) = lookup::lookup() else { return };
    let Some(dns) = status::status(port) else { return };
    if dns != [0; 4] {
        set_upstream(dns);
    }
}
