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

mod constants;
mod hover_motion;
mod mirror_shell_pointer;
mod refresh_display;
mod route_pointer;
mod route_to_press;
mod route_to_shell;
mod route_to_window;
mod shell_pid;
mod topmost_target;

pub use route_pointer::route_pointer;
pub(super) use shell_pid::shell_pid;
