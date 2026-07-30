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

pub mod logger {
    pub use crate::log::{
        enter_panic_mode, init, log, log_critical, try_get_logger, LogManager, Severity,
    };
    pub use crate::{
        debug, info, log_dbg, log_debug, log_err, log_error, log_fatal, log_info, log_warn,
        log_warning, warn,
    };
}

pub mod nonos_logger {
    #[cfg(target_arch = "x86_64")]
    pub use crate::log::VgaBackend;
    pub use crate::log::{
        clear_log_buffer, debug, enter_panic_mode, get_log_entries, get_recent_logs, init,
        init_logger, log, log_critical, log_entry_count, try_get_logger, LogBackend, LogEntry,
        LogManager, RamBufferBackend, Severity, LOGGER, PANIC_MODE, RAM_BUF_SIZE,
    };
}

pub mod simple_logger {
    pub use super::nonos_logger::*;
}
