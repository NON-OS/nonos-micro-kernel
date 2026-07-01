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

mod create;
mod emit_lease_marker;
mod emit_status_selfcheck;
mod fill_marker;
mod handle_configured;
mod handle_deconfigured;
mod install_dns_socket;
mod poll_event;
mod types;
mod write_decimal_u8;
mod write_octet_quad;

pub use create::create;
pub use poll_event::poll_event;
