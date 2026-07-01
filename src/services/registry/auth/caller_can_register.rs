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

use super::caller_has_register_right::caller_has_register_right;
use super::caller_has_required::caller_has_required;

pub(in crate::services::registry) fn caller_can_register(required: u64) -> bool {
    match crate::process::current_pid() {
        None => true,
        Some(pid) if pid <= 64 => caller_has_required(required),
        Some(_) => caller_has_required(required) && caller_has_register_right(),
    }
}
