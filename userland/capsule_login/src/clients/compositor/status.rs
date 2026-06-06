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
use super::constants::HDR_LEN;

pub fn check(rc: i64, rx: &[u8]) -> Result<(), i32> {
    if rc < (HDR_LEN + 4) as i64 {
        return Err(-11);
    }
    let status = i32::from_le_bytes(rx[HDR_LEN..HDR_LEN + 4].try_into().map_err(|_| -11)?);
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}
