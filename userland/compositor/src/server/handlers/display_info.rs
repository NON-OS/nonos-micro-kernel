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

use nonos_libc::SURFACE_FORMAT_ARGB8888;

use crate::protocol::{Request, DISPLAY_INFO_RESP_LEN};
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &Context,
    sender_pid: u32,
    req: &Request,
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let mut body = [0u8; DISPLAY_INFO_RESP_LEN - 4];
    body[0..4].copy_from_slice(&ctx.width.to_le_bytes());
    body[4..8].copy_from_slice(&ctx.height.to_le_bytes());
    body[8..12].copy_from_slice(&ctx.stride.to_le_bytes());
    body[12..16].copy_from_slice(&SURFACE_FORMAT_ARGB8888.to_le_bytes());
    respond::status_payload(sender_pid, req, 0, &body, tx)
}
