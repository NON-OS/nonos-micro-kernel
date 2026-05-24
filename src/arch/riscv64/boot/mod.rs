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

pub mod dtb_adapter;
pub mod entry;
pub mod hart_id;
pub mod info;
mod init;
pub mod multicore;
pub mod stack;

pub use entry::kernel_entry;
pub use info::{BootInfo, MemoryRegion};
pub use init::init;
pub use multicore::start_secondary_harts;
pub use stack::setup_stack;
