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

pub mod access;
pub mod bridge;
pub mod config_space;
mod transport;

pub mod power;

pub use access::{
    get_config_stats, make_config_address, read16, read32, read32_unchecked, read8,
    reset_config_stats, write16, write32, write32_unchecked, write8,
};
pub use bridge::BridgeConfigSpace;
pub use config_space::ConfigSpace;
pub use transport::set_ecam_window;
