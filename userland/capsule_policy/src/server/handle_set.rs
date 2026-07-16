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

use nonos_libc::mk_service_lookup;
use nonos_policy_proto::{kind_of, Field, E_ACCES, KIND_BOOL, KIND_I8, KIND_STR, KIND_U8, OP_SET};

use super::handlers::{set_bool, set_i8, set_str, set_u8};
use super::respond;

// Only the settings app and the first-boot setup wizard may write policy. The
// settings window now runs as an on-demand instance, so its instance service
// names are trusted too; all of them are the same signed, attested settings
// capsule, just a different window slot.
const SETTERS: [&[u8]; 4] =
    [b"app.settings", b"app.setup_wizard", b"app.settings.1", b"app.settings.2"];

fn lookup_pid(name: &[u8]) -> Option<u32> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 {
        None
    } else {
        Some(pid)
    }
}

fn is_trusted_setter(sender: u32) -> bool {
    SETTERS.iter().any(|name| lookup_pid(name) == Some(sender))
}

pub fn dispatch(pid: u32, field: Field, payload: &[u8]) {
    if !is_trusted_setter(pid) {
        respond::err(pid, OP_SET, field as u32, kind_of(field), E_ACCES);
        return;
    }
    match kind_of(field) {
        KIND_BOOL => set_bool::handle(pid, field, payload),
        KIND_U8 => set_u8::handle(pid, field, payload),
        KIND_I8 => set_i8::handle(pid, field, payload),
        KIND_STR => set_str::handle(pid, field, payload),
        _ => {}
    }
}
