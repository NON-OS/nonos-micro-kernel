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

pub mod defaults;
pub mod get_bool;
pub mod get_i8;
pub mod get_str;
pub mod get_u8;
pub mod set_bool;
pub mod set_i8;
pub mod set_str;
pub mod set_u8;
pub mod state;
pub mod str_validate;
pub mod types;

pub use types::STRING_CAP;
