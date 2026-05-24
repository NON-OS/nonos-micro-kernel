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
mod base;
pub mod builders;
pub mod commands;
mod completion_code;
mod get_cycle;
mod get_pointer;
mod get_type;
mod read_volatile_at;
mod set_cycle;
mod set_ioc;
mod set_pointer;
mod set_transfer_length;
mod set_type;
mod slot_id;
mod write_volatile_at;
pub use base::Trb;
pub use read_volatile_at::read_volatile_at;
pub use write_volatile_at::write_volatile_at;