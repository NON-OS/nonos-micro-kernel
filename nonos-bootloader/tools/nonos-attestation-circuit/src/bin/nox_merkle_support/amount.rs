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

pub fn parse_amount(decimal: &str) -> Result<[u8; 32], String> {
    if decimal.is_empty() || !decimal.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("amount {decimal} is not a decimal integer"));
    }
    let mut out = [0u8; 32];
    for digit in decimal.bytes() {
        let mut carry = (digit - b'0') as u16;
        for byte in out.iter_mut().rev() {
            let value = (*byte as u16) * 10 + carry;
            *byte = (value & 0xFF) as u8;
            carry = value >> 8;
        }
        if carry != 0 {
            return Err(format!("amount {decimal} exceeds 256 bits"));
        }
    }
    Ok(out)
}
