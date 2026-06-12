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

use super::edit::Edit;
use super::mode::next_mode;
use super::state::Settings;

pub fn apply(s: &mut Settings, e: Edit) {
    match e {
        Edit::Mode => s.default_mode = next_mode(s.default_mode),
        Edit::Timeout => s.timeout_s = if s.timeout_s >= 30 { 0 } else { s.timeout_s + 5 },
        Edit::Enforce => s.enforce_sb = !s.enforce_sb,
    }
}
