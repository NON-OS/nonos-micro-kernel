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

use nonos_libc::{mk_ipc_send_to_pid, mk_service_lookup};

use crate::state::apps::LauncherApp;

const CONTROL_LEN: usize = 8;
const CONTROL_MAGIC: u32 = u32::from_le_bytes(*b"NCTL");
const CONTROL_VERSION: u16 = 1;
const OP_FOCUS_SELF: u16 = 1;

pub fn request(app: &LauncherApp) -> bool {
    let Some(pid) = lookup_pid(app.service) else { return false };
    let frame = focus_frame();
    mk_ipc_send_to_pid(pid, frame.as_ptr(), frame.len()) >= 0
}

fn lookup_pid(service: &[u8]) -> Option<u32> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(service.as_ptr(), service.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 {
        return None;
    }
    Some(pid)
}

fn focus_frame() -> [u8; CONTROL_LEN] {
    let mut frame = [0u8; CONTROL_LEN];
    frame[0..4].copy_from_slice(&CONTROL_MAGIC.to_le_bytes());
    frame[4..6].copy_from_slice(&CONTROL_VERSION.to_le_bytes());
    frame[6..8].copy_from_slice(&OP_FOCUS_SELF.to_le_bytes());
    frame
}
