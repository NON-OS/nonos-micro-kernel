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

/// Compress an uncompressed SEC1 public key (0x04 || X || Y) to the 33-byte
/// form BIP32 serializes: 0x02/0x03 by Y parity, then X. None if the input
/// is not in uncompressed form.
pub fn compress_pubkey(uncompressed: &[u8; 65]) -> Option<[u8; 33]> {
    if uncompressed[0] != 0x04 {
        return None;
    }
    let mut out = [0u8; 33];
    out[0] = if uncompressed[64] & 1 == 1 { 0x03 } else { 0x02 };
    out[1..].copy_from_slice(&uncompressed[1..33]);
    Some(out)
}
