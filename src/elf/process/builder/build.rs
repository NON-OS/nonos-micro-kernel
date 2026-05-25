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

use crate::elf::auxv::AuxvBuilder;
use crate::elf::errors::ElfResult;
use crate::elf::stack::{setup_user_stack, StackConfig};

use super::state::ProcessBuilder;
use super::super::image::ProcessImage;

impl<'a> ProcessBuilder<'a> {
    pub fn build(mut self, elf_data: &[u8]) -> ElfResult<ProcessImage> {
        let executable = self.loader.load_executable(elf_data)?;
        let interpreter = if let Some(path) = executable.interpreter.clone() {
            Some(self.load_interpreter(&path)?)
        } else {
            None
        };
        let phdr_addr = self.find_phdr_addr(&executable, elf_data)?;
        let phnum = self.get_phnum(elf_data)?;
        let mut auxv_builder = AuxvBuilder::from_elf_image(&executable, phdr_addr, phnum);
        auxv_builder.set_uid(self.config.uid as u64);
        auxv_builder.set_euid(self.config.uid as u64);
        auxv_builder.set_gid(self.config.gid as u64);
        auxv_builder.set_egid(self.config.gid as u64);
        let auxv = auxv_builder.build();
        let stack_config = StackConfig::new()
            .with_args(self.config.args)
            .with_env(self.config.env)
            .with_auxv(auxv)
            .with_stack_size(self.config.stack_size);
        let stack = setup_user_stack(self.stack_top, self.config.stack_size, &stack_config)?;
        let mut process = ProcessImage::new(executable, interpreter, stack);
        if let Some(tls_info) = process.executable.tls_info {
            process.set_tls(tls_info);
        }
        process.set_ready();
        Ok(process)
    }
}
