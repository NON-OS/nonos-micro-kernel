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

use crate::log::logger::log_error;
use uefi::prelude::*;
use uefi::table::runtime::ResetType;

/// Log allocation failure and cold reset. Called when critical handoff allocation fails.
pub fn fatal_alloc_error(st: &SystemTable<Boot>, resource: &str) -> ! {
    log_error("handoff", resource);
    for _ in 0..1_000_000 {
        core::hint::spin_loop();
    }
    st.runtime_services().reset(ResetType::COLD, Status::OUT_OF_RESOURCES, None);
}
