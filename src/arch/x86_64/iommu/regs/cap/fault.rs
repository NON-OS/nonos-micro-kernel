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

/// Byte offset of the fault-recording registers, stored as a 16-byte count.
pub const fn fault_recording_offset(cap: u64) -> usize {
    (((cap >> 24) & 0x3FF) as usize) * 16
}

/// Number of fault-recording registers, stored as count - 1.
pub const fn fault_recording_count(cap: u64) -> u16 {
    (((cap >> 40) & 0xFF) as u16) + 1
}
