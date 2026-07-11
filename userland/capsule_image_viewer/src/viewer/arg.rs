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

extern crate alloc;
use alloc::string::String;

use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::{call_payload, HDR_LEN};

use crate::viewer::load;
use crate::viewer::state::ViewerState;

// Hand-synced with desktop_shell's protocol::{MAGIC, OP_TAKE_OPEN_ARG} and
// file_manager's open-with sender: keep all three identical.
const NDSH: u32 = 0x4E44_5348;
const OP_TAKE_OPEN_ARG: u16 = 0x0008;

pub fn poll_open(st: &mut ViewerState) -> bool {
    let Some(shell) = lookup_service(b"desktop_shell") else { return false };
    let mut rx = [0u8; 512];
    let total = match call_payload(shell.port, NDSH, OP_TAKE_OPEN_ARG, 1, &[], &mut rx) {
        Ok(n) => n,
        Err(_) => return false,
    };
    if total <= HDR_LEN + 4 {
        return false;
    }
    let path = match core::str::from_utf8(&rx[HDR_LEN + 4..total]) {
        Ok(p) if !p.is_empty() => p,
        _ => return false,
    };
    let owned = String::from(path);
    load::open_path(st, &owned);
    true
}
