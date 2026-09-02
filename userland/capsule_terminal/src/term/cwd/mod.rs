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

mod as_bytes;
mod dir_prefix;
mod home;
mod home_var;
mod new;
mod resolve;
mod shorten;
mod set;
mod strip_home;
mod types;

pub use dir_prefix::dir_prefix;
pub use home::HOME;
pub use home_var::home_var;
pub use resolve::resolve;
pub use shorten::shorten;
pub use strip_home::strip_home;
pub use types::Cwd;
