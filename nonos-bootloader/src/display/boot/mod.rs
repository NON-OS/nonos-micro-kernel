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

mod chips;
mod crypto;
mod error;
mod handoff;
mod init;
pub mod layout;
mod panel;
mod progress;
mod stage;
mod status_line;
mod vignette;
mod wordmark;

pub use crypto::{animate_hash_reveal, show_crypto_verification, BootCryptoState};
pub use error::show_error_screen;
pub use handoff::show_handoff_message;
pub use init::{init_boot_screen, reset_animation, tick_animation};
pub use panel::{draw_panel, panel_line};
pub use progress::draw_boot_progress;
pub use stage::{get_boot_progress_percent, update_stage, StageStatus};
pub use status_line::draw_status_line;
pub use vignette::draw_vignette;
pub use wordmark::draw_wordmark;
