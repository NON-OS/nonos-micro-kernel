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
mod address_device;
mod configure_endpoint;
mod disable_slot;
mod enable_slot;
mod noop;
pub use address_device::address_device_command;
pub use configure_endpoint::configure_endpoint_command;
pub use disable_slot::disable_slot_command;
pub use enable_slot::enable_slot_command;
pub use noop::noop_command;
