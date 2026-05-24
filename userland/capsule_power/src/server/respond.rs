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

use crate::protocol::{response_header, write_status, Request, HDR_LEN, STATUS_LEN};

pub fn status(out: &mut [u8], req: &Request, status_code: i32) -> usize {
    response_header(out, req, STATUS_LEN as u32);
    write_status(out, status_code);
    HDR_LEN + STATUS_LEN
}
