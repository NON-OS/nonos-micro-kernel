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

extern crate alloc;

use alloc::vec::Vec;
use uefi::prelude::*;

use crate::loader::{file_exists, load_file_from_esp};

pub fn load_zk_sidecar(st: &SystemTable<Boot>) -> Option<Vec<u8>> {
    let path = uefi::cstr16!("\\EFI\\nonos\\boot.zkp");
    if !file_exists(st, path) {
        return None;
    }
    load_file_from_esp(st, path).ok()
}
