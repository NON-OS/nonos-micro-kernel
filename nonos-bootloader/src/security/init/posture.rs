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

use uefi::cstr16;
use uefi::prelude::*;

use crate::log::logger::{log_error, log_info, log_warn};
use crate::security::types::SecurityContext;

pub fn assess_security_posture(ctx: &SecurityContext, st: &mut SystemTable<Boot>) -> u32 {
    let score = ctx.security_score();
    let _ = st.stdout().output_string(cstr16!("   [INFO] Security posture assessment:\r\n"));
    match score {
        80..=100 => {
            output_excellent(st);
            log_info("security", "Security posture: EXCELLENT");
        }
        60..=79 => {
            output_good(st);
            log_info("security", "Security posture: GOOD");
        }
        40..=59 => {
            output_moderate(st);
            log_warn("security", "Security posture: MODERATE");
        }
        _ => {
            output_low(st);
            log_error("security", "Security posture: LOW");
        }
    }
    score
}

fn output_excellent(st: &mut SystemTable<Boot>) {
    let _ = st.stdout().output_string(cstr16!("   [SUCCESS] Security Score: EXCELLENT\r\n"));
}

fn output_good(st: &mut SystemTable<Boot>) {
    let _ = st.stdout().output_string(cstr16!("   [INFO] Security Score: GOOD\r\n"));
}

fn output_moderate(st: &mut SystemTable<Boot>) {
    let _ = st.stdout().output_string(cstr16!("   [WARN] Security Score: MODERATE\r\n"));
}

fn output_low(st: &mut SystemTable<Boot>) {
    let _ = st.stdout().output_string(cstr16!("   [CRITICAL] Security Score: LOW\r\n"));
}
