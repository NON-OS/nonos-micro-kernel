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

use super::record::FaultRecord;
use crate::sys::serial;

/// The source id is printed raw rather than split into bus, device and
/// function: it is what the DMAR tables and lspci both key on, so it is the
/// form that can be looked up directly.
pub(super) fn log_record(record: &FaultRecord) {
    serial::print(b"[VT-D] denied ");
    serial::print(if record.read { b"read " } else { b"write " });
    serial::print(b"src=");
    serial::print_hex(record.source as u64);
    serial::print(b" addr=");
    serial::print_hex(record.address);
    serial::print(b" reason=");
    serial::print_hex(record.reason as u64);
    serial::println(b"");
}
