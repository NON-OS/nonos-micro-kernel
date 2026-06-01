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

use nonos_abi::InputEvent;

pub(super) fn parse_event(p: &[u8]) -> Option<InputEvent> {
    if p.len() < 32 {
        return None;
    }
    let kind = u16::from_le_bytes([p[0], p[1]]);
    if kind > 7 {
        return None;
    }
    Some(InputEvent {
        kind,
        flags: u16::from_le_bytes([p[2], p[3]]),
        code: u32::from_le_bytes([p[4], p[5], p[6], p[7]]),
        x: i32::from_le_bytes([p[8], p[9], p[10], p[11]]),
        y: i32::from_le_bytes([p[12], p[13], p[14], p[15]]),
        delta_x: i32::from_le_bytes([p[16], p[17], p[18], p[19]]),
        delta_y: i32::from_le_bytes([p[20], p[21], p[22], p[23]]),
        timestamp_ns: u64::from_le_bytes([
            p[24], p[25], p[26], p[27], p[28], p[29], p[30], p[31],
        ]),
    })
}
