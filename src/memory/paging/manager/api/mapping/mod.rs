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

mod map_device_memory;
mod map_huge_page;
mod map_kernel_page;
mod map_page;
mod map_user_dma;
mod map_user_mmio;
mod map_user_page;
mod unmap_page;
mod unmap_range;
mod unmap_user_dma;
mod unmap_user_mmio;

pub use map_device_memory::map_device_memory;
pub use map_huge_page::map_huge_page;
pub use map_kernel_page::map_kernel_page;
pub use map_page::map_page;
pub use map_user_dma::map_user_dma;
pub use map_user_mmio::map_user_mmio;
pub use map_user_page::map_user_page;
pub use unmap_page::unmap_page;
pub use unmap_range::unmap_range;
pub use unmap_user_dma::unmap_user_dma;
pub use unmap_user_mmio::unmap_user_mmio;
