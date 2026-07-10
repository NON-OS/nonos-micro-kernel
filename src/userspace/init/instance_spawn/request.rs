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

use super::queue::{self, PendingApp};

/// Queue one window-instance spawn for init to perform. Runs in the
/// caller's syscall context, so it only touches the request queue; the
/// real spawn happens later in init. Returns true when the request was
/// accepted.
pub fn request(app: PendingApp) -> bool {
    queue::push(app)
}
