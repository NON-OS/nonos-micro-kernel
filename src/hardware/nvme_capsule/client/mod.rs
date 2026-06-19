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
mod identify_controller;
mod identify_namespace;
mod read;
mod read_blocks;
mod seq;
mod smart_decode;
mod smart_health;
mod status_map;
mod transport;
mod write_blocks;

pub(super) use transport::REPLY_INBOX;

pub use capacity::capacity;
pub use controller_info::{controller_info, NvmeControllerInfo};
pub use flush::flush;
pub use healthcheck::healthcheck;
pub use identify_controller::{identify_controller, NvmeControllerIdentity};
pub use identify_namespace::{identify_namespace, NvmeNamespaceIdentity};
pub use read_blocks::read_blocks;
pub use smart_health::{smart_health, NvmeSmartHealth};
pub use write_blocks::write_blocks;
