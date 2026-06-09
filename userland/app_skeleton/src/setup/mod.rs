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

mod announce;
mod backing;
mod binding;
mod ensure_input_subscription;
mod input_mask;
mod open;
mod register;
mod request_id;
mod resize;
mod submit_scene;
mod subscribe_input;

pub use binding::WindowBinding;
pub(crate) use ensure_input_subscription::ensure_input_subscription;
pub use open::open_window;
pub use resize::reopen_surface;
