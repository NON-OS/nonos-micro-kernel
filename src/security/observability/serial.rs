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

//! Policy-gated console output.
//!
//! This used to drive COM1 directly, one byte per port write with no check
//! that the transmitter had room, which both hard-coded a PC and dropped
//! characters under load. The console already exists behind `sys::serial`,
//! knows how to wait for the UART, and works on any board. What belongs here
//! is only the decision of whether to emit and what to redact.

use super::policy::{is_production_mode, should_emit_serial};
use super::redact::redact_panic_message;
use crate::sys::serial;

pub fn serial_log(msg: &str) {
    if !should_emit_serial() {
        return;
    }
    serial::print_str(msg);
    serial::println(b"");
}

pub fn serial_log_redacted(msg: &str) {
    if !should_emit_serial() {
        return;
    }
    if is_production_mode() {
        serial::print_str(&redact_panic_message(msg));
    } else {
        serial::print_str(msg);
    }
    serial::println(b"");
}
