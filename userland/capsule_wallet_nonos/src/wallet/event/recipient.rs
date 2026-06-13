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

use crate::wallet::state::State;

pub fn recipient(state: &State) -> Option<[u8; 20]> {
    if state.send_to_len != 40 {
        return None;
    }
    let mut out = [0u8; 20];
    for i in 0..20 {
        let hi = super::hex_digit::hex_digit(state.send_to_hex[i * 2] as u32)?;
        let lo = super::hex_digit::hex_digit(state.send_to_hex[i * 2 + 1] as u32)?;
        out[i] = (hi << 4) | lo;
    }
    Some(out)
}
