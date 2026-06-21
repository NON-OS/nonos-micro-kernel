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

mod capacity;
mod controller_info;
mod flush;
mod healthcheck;
mod port_list;
mod read_blocks;
mod seq;
mod status_map;
mod transport;
mod write_blocks;

pub(super) use transport::REPLY_INBOX;

pub use capacity::capacity;
pub use controller_info::{controller_info, AhciControllerInfo};
pub use flush::flush;
pub use healthcheck::healthcheck;
pub use port_list::{port_list, AhciPortInfo};
pub use read_blocks::read_blocks;
pub use write_blocks::write_blocks;
