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

use crate::clock::now_ms;
use crate::tcp::siphash24;

pub fn iss_for(
    key: [u64; 2],
    local_ip: [u8; 4],
    local_port: u16,
    remote_ip: [u8; 4],
    remote_port: u16,
) -> u32 {
    let mut buf = [0u8; 12];
    buf[0..4].copy_from_slice(&local_ip);
    buf[4..6].copy_from_slice(&local_port.to_be_bytes());
    buf[6..10].copy_from_slice(&remote_ip);
    buf[10..12].copy_from_slice(&remote_port.to_be_bytes());
    let hash = siphash24(key, &buf) as u32;
    (now_ms() as u32).wrapping_add(hash)
}
