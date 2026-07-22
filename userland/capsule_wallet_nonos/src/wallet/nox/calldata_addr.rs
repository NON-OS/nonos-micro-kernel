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

// ABI calldata for `f(address)`: the 4-byte selector followed by the address
// right-aligned in a 32-byte word (12 zero bytes, then the 20 address bytes).
pub fn calldata_addr(selector: &[u8; 4], addr: &[u8; 20]) -> [u8; 36] {
    let mut out = [0u8; 36];
    out[0..4].copy_from_slice(selector);
    out[16..36].copy_from_slice(addr);
    out
}
