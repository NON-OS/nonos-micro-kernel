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

use crate::driver::Driver;
use crate::protocol::{Request, E_FW_INVALID, E_OK};
use crate::server::respond;

pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, out: &mut [u8]) {
    match driver.load_firmware() {
        Ok(sections) => {
            let mut body = [0u8; 4];
            body.copy_from_slice(&sections.to_le_bytes());
            let _ = respond::send(sender_pid, req, E_OK, &body, out);
        }
        Err(_) => {
            let _ = respond::send(sender_pid, req, E_FW_INVALID, &[], out);
        }
    }
}
