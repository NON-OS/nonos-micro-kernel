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

mod close;
mod focus;
mod maximize;
mod minimize;
mod move_window;
mod open;
mod raise;
mod resize;

pub use close::window_close;
pub use focus::window_focus;
pub use maximize::window_maximize;
pub use minimize::{window_minimize, window_restore};
pub use move_window::window_move;
pub use open::{window_open, WindowPlacement};
pub use raise::window_raise;
pub use resize::window_resize;
