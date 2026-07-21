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

use super::{format_nox, q32_to_u128};

// Render an on-chain NOX amount as a bare decimal, or an em dash when the read
// is not ready or the value does not decode. The surrounding label already
// names the unit, so no suffix is added. Shared by every NOX screen so the
// dash-until-real rule lives in one place.
pub fn amount_str<'a>(ready: bool, wei: &[u8; 32], buf: &'a mut [u8]) -> &'a str {
    if !ready {
        return "\u{2014}";
    }
    let Some(v) = q32_to_u128(wei) else {
        return "\u{2014}";
    };
    let n = format_nox(v, buf);
    core::str::from_utf8(&buf[..n]).unwrap_or("\u{2014}")
}
