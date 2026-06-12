// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::guid::VENDOR;
use super::state::Settings;
use uefi::cstr16;
use uefi::prelude::*;

pub fn load(st: &SystemTable<Boot>) -> Settings {
    let mut buf = [0u8; 3];
    match st.runtime_services().get_variable(cstr16!("NonosSetup"), &VENDOR, &mut buf) {
        Ok(_) => Settings {
            default_mode: if (1..=5).contains(&buf[0]) { buf[0] } else { 1 },
            timeout_s: if buf[1] <= 30 { buf[1] } else { 5 },
            enforce_sb: buf[2] != 0,
        },
        Err(_) => Settings::fallback(),
    }
}
