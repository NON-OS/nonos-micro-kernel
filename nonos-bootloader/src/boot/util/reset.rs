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

pub fn fatal_reset(st: &mut SystemTable<Boot>, reason: &str) -> ! {
    log_error("fatal", reason);
    let _ = st.stdout().reset(false);
    let _ = st.stdout().output_string(cstr16!("\r\n[FATAL] "));
    if let Ok(s) = uefi::CString16::try_from(reason) {
        let _ = st.stdout().output_string(&s);
    }
    let _ = st.stdout().output_string(cstr16!("\r\nSystem will restart...\r\n"));
    for _ in 0..10_000_000 {
        core::hint::spin_loop();
    }
    st.runtime_services().reset(ResetType::WARM, Status::LOAD_ERROR, Some(reason.as_bytes()))
}
