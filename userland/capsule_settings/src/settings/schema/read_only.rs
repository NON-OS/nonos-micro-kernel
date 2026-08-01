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

//! Fields the panel shows but will not change.
//!
//! The policy protocol has no read-only notion, so the store accepts a write
//! to any field. Some of them report what the system did rather than what the
//! user wants, and letting someone assert one from a settings row would be
//! stating something untrue about the machine.

use nonos_policy_proto::Field;

/// Whether `field` is a status the panel displays without editing.
pub fn read_only(field: Field) -> bool {
    matches!(field, Field::SystemKeysGenerated)
}
