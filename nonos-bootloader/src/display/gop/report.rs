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

use super::init::preferred_mode;
use super::state::{get_dimensions, is_initialized};
use crate::log::logger::{log_error, log_info};
use alloc::format;

pub fn report_gop_mode() {
    if !is_initialized() {
        log_error("gop", "[GOP] no linear BGR framebuffer was latched");
        return;
    }
    let (got_w, got_h) = get_dimensions();
    let (want_w, want_h) = match preferred_mode() {
        Some(mode) => mode,
        None => {
            log_info("gop", &format!("[GOP] want=auto got={}x{}", got_w, got_h));
            return;
        }
    };
    log_info(
        "gop",
        &format!("[GOP] want={}x{} got={}x{}", want_w, want_h, got_w, got_h),
    );
    if want_w as u32 != got_w || want_h as u32 != got_h {
        log_error(
            "gop",
            &format!(
                "[GOP] FALLBACK want={}x{} was not offered, scanning out {}x{}",
                want_w, want_h, got_w, got_h
            ),
        );
    }
}
