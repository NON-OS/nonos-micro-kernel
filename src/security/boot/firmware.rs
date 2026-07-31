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

use alloc::collections::BTreeMap;
use alloc::string::String;
use spin::Once;

pub struct FirmwareDB {
    pub trusted_hashes: BTreeMap<String, [u8; 32]>,
}

// Same as the module database: written once at init, read afterwards.
static FIRMWARE_DB: Once<FirmwareDB> = Once::new();

pub fn init() -> Result<(), &'static str> {
    FIRMWARE_DB.call_once(|| FirmwareDB { trusted_hashes: BTreeMap::new() });
    Ok(())
}
