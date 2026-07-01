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

use super::constants::{DNS_MAGIC, OP_RESOLVE_A};

pub fn resolve(dns_port: u32, host: &[u8]) -> Result<[u8; 4], ()> {
    let mut rx = [0u8; 32];
    let n = super::call::call(dns_port, DNS_MAGIC, OP_RESOLVE_A, host, &mut rx)?;
    if n < 24 {
        return Err(());
    }
    let ip = [rx[20], rx[21], rx[22], rx[23]];
    if ip == [0, 0, 0, 0] {
        return Err(());
    }
    Ok(ip)
}
