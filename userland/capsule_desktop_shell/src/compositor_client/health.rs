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

use super::wire::call;

const OP: u16 = 0x0001;

pub fn healthcheck(compositor_port: u32, request_id: u32) -> Result<(), &'static str> {
    let status = call(compositor_port, OP, request_id, &[])?;
    if status != 0 {
        return Err("compositor health rejected");
    }
    Ok(())
}
