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

use alloc::vec::Vec;

use super::constants::EXT_SERVER_NAME;

pub fn ext_sni(out: &mut Vec<u8>, host: &[u8]) {
    let mut body = Vec::with_capacity(host.len() + 5);
    super::push::u16(&mut body, host.len() as u16 + 3);
    body.push(0);
    super::push::u16(&mut body, host.len() as u16);
    body.extend_from_slice(host);
    super::push::ext(out, EXT_SERVER_NAME, &body);
}
