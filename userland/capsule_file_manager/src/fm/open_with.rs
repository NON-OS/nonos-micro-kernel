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

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::call_status;

use super::file_ext::ext;

// Hand-synced with desktop_shell's protocol::{MAGIC, OP_OPEN_WITH} and
// image_viewer's poll_open: keep all three identical.
const NDSH: u32 = 0x4E44_5348;
const OP_OPEN_WITH: u16 = 0x0007;

pub fn is_codec_ext(path: &str) -> bool {
    let e = ext(path);
    e.eq_ignore_ascii_case("png")
        || e.eq_ignore_ascii_case("jpg")
        || e.eq_ignore_ascii_case("jpeg")
        || e.eq_ignore_ascii_case("bmp")
}

pub fn open_image(path: &str) -> bool {
    let Some(shell) = lookup_service(b"desktop_shell") else { return false };
    let svc = b"app.image_viewer";
    let mut body = Vec::with_capacity(2 + svc.len() + path.len());
    body.extend_from_slice(&(svc.len() as u16).to_le_bytes());
    body.extend_from_slice(svc);
    body.extend_from_slice(path.as_bytes());
    matches!(call_status(shell.port, NDSH, OP_OPEN_WITH, 1, &body), Ok(0))
}
