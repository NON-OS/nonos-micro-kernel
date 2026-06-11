// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod build;
pub mod constants;
mod frame;
mod map_directmap;
mod map_identity;
mod map_kernel_text;
mod mapper;
mod page1g;
mod phys_to_directmap;
mod seg_flags;
mod switch;
mod table;
mod verify;

pub use build::build_kernel_pml4;
pub use phys_to_directmap::phys_to_directmap_virt;
pub use switch::switch_to_kernel_pml4;
