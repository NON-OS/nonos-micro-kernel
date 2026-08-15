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

pub mod boot;
pub mod constants;
pub mod font;
pub mod fx;
pub mod gop;
pub mod log_panel;
pub mod security;

pub use boot::{
    animate_hash_reveal, draw_boot_progress, draw_status_line, draw_wordmark, init_boot_screen,
    reset_animation, show_crypto_verification, show_error_screen, show_handoff_message,
    tick_animation, update_stage, BootCryptoState, StageStatus,
};
pub use constants::*;
pub use gop::{init_gop, report_gop_mode};
pub use log_panel::{
    get_cursor_y, log_error, log_hash, log_hash_full, log_hex, log_info, log_mem, log_ok,
    log_security, log_size, log_u32, log_warn,
};
pub use security::display_enforcement_result;
