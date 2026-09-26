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


//! The globals a client is told about, and what binding one means.

use super::object::Object;

pub struct Global {
    pub name: u32,
    pub interface: &'static [u8],
    pub version: u32,
    pub object: Object,
}

pub const GLOBALS: &[Global] = &[
    Global { name: 1, interface: b"wl_compositor", version: 4, object: Object::Compositor },
    Global { name: 2, interface: b"wl_shm", version: 1, object: Object::Shm },
    Global { name: 3, interface: b"xdg_wm_base", version: 2, object: Object::XdgBase },
    Global { name: 4, interface: b"wl_seat", version: 5, object: Object::Seat },
];

pub fn by_name(name: u32) -> Option<&'static Global> {
    GLOBALS.iter().find(|g| g.name == name)
}
