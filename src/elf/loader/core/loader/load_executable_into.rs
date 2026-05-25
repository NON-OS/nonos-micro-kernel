// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::{base_addr, build_image, load_parts, state::ElfLoader};
use crate::elf::errors::ElfError;
use crate::elf::loader::ElfImage;

impl ElfLoader {
    pub fn load_executable_into(
        &mut self,
        elf_data: &[u8],
        target_asid: u32,
    ) -> Result<ElfImage, ElfError> {
        let header = super::super::parse_header::parse_elf_header(elf_data)?;
        super::super::parse_header::validate_elf(&header)?;
        let program_headers = super::super::parse_header::parse_program_headers(elf_data, &header)?;
        let base_addr = base_addr::load_base(&header, &mut self.aslr_manager);
        let parts = load_parts::load(elf_data, &program_headers, base_addr, target_asid)?;
        Ok(build_image::from_parts(&header, base_addr, parts))
    }
}
