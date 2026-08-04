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

pub fn spki_point(spki: &[u8]) -> Option<&[u8]> {
    let (tag, val, end) = super::der_tlv::der_tlv(spki, 0)?;
    if tag != 0x30 || end != spki.len() {
        return None;
    }
    let (_, _, alg_end) = super::der_tlv::der_tlv(spki, val)?;
    let (tag, bit_val, bit_end) = super::der_tlv::der_tlv(spki, alg_end)?;
    if tag == 0x03 && bit_val < bit_end && spki[bit_val] == 0 {
        Some(&spki[bit_val + 1..bit_end])
    } else {
        None
    }
}
