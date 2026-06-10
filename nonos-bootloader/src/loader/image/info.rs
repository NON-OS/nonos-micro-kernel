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

use super::kernel::KernelImage;

#[derive(Debug)]
pub struct KernelInfo {
    pub load_address: u64,
    pub virtual_address: u64,
    pub entry_point: u64,
    pub text_size: usize,
    pub data_size: usize,
    pub bss_size: usize,
    pub total_size: usize,
    pub is_pie: bool,
    pub has_relocations: bool,
    pub segment_count: usize,
}

impl KernelInfo {
    pub fn from_image(image: &KernelImage, is_pie: bool) -> Self {
        let virtual_address =
            if image.virt_base != 0 { image.virt_base } else { image.address as u64 };
        Self {
            load_address: image.address as u64,
            virtual_address,
            entry_point: image.entry_point as u64,
            text_size: 0,
            data_size: 0,
            bss_size: 0,
            total_size: image.size,
            is_pie,
            has_relocations: false,
            segment_count: 0,
        }
    }
}
