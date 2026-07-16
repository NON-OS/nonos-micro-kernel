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

/// TEST UNIT READY (SPC opcode 0x00): a zero-data command that reports whether
/// the medium is present and spun up. A CHECK CONDITION here is how removable
/// media announces "no medium" or "not ready yet". Returns the 16-byte CDB and
/// its significant length.
pub fn test_unit_ready() -> ([u8; 16], u8) {
    ([0u8; 16], 6)
}
