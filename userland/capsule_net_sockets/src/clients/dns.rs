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

use super::envelope::call;

const MAGIC: u32 = 0x4E44_4E53;
const RESOLVE_A: u16 = 2;

pub fn resolve_a(port: u32, host: &[u8]) -> Result<[u8; 4], u16> {
    let mut out = [0u8; 4];
    if call(port, MAGIC, RESOLVE_A, host, &mut out)? != 4 {
        return Err(4);
    }
    Ok(out)
}
