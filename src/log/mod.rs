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

pub mod backend;
mod compat;
pub mod dbg_ring;
mod helpers;
mod macros;
pub mod manager;
pub mod types;

#[cfg(target_arch = "x86_64")]
pub use backend::VgaBackend;
pub use backend::{LogBackend, RamBufferBackend, RAM_BUF_SIZE};
pub use helpers::{debug_simple, info_simple, log_error_simple, warn_simple};
pub use manager::{
    clear_log_buffer, enter_panic_mode, get_log_entries, get_recent_logs, init, log, log_critical,
    log_entry_count, try_get_logger, LogManager, LOGGER, PANIC_MODE,
};
pub use types::{LogEntry, Severity};

pub use init as init_logger;

pub use crate::debug;
pub use crate::error;
pub use crate::info;
pub use crate::log_dbg;
pub use crate::log_debug;
pub use crate::log_err;
pub use crate::log_error;
pub use crate::log_fatal;
pub use crate::log_info;
pub use crate::log_warn;
pub use crate::log_warning;
pub use crate::security_log;
pub use crate::warn;

pub use compat::logger;
pub use compat::nonos_logger;
pub use compat::simple_logger;
