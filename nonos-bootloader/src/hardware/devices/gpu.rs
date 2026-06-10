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

use crate::log::logger::log_info;
use uefi::prelude::*;

pub fn detect_dual_gpu(system_table: &mut SystemTable<Boot>) -> bool {
    let bs = system_table.boot_services();
    let gop_count = bs
        .find_handles::<uefi::proto::console::gop::GraphicsOutput>()
        .map(|h| h.len())
        .unwrap_or(0);
    if gop_count > 1 {
        log_info("gpu", "dual GPU detected (multiple GOP handles)");
        return true;
    }
    false
}
