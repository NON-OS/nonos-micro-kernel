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

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use spin::Once;

pub struct ModuleDB {
    trusted_modules: BTreeMap<String, [u8; 32]>,
}

// Written once at init, read afterwards. A static mut gave a second CPU no
// ordering against that write.
static MODULE_DB: Once<ModuleDB> = Once::new();

pub fn init() -> Result<(), &'static str> {
    MODULE_DB.call_once(|| ModuleDB { trusted_modules: BTreeMap::new() });
    Ok(())
}

pub fn is_trusted_module(name: &str) -> bool {
    MODULE_DB.get().is_some_and(|db| db.trusted_modules.contains_key(name))
}

pub fn get_loaded_modules() -> Vec<String> {
    MODULE_DB.get().map(|db| db.trusted_modules.keys().cloned().collect()).unwrap_or_default()
}
