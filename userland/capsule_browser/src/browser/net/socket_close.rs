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

use super::constants::{OP_CLOSE, SOCKETS_MAGIC};

pub fn socket_close(sockets_port: u32, handle: u32) -> bool {
    if super::mixnet::is_on() {
        return super::mixnet::close();
    }
    let mut body = [0u8; 4];
    let mut rx = [0u8; 20];
    body.copy_from_slice(&handle.to_le_bytes());
    super::call::call(sockets_port, SOCKETS_MAGIC, OP_CLOSE, &body, &mut rx).is_ok()
}
