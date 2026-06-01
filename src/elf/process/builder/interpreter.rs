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

use crate::elf::embedded::load_embedded_library;
use crate::elf::errors::{ElfError, ElfResult};
use crate::elf::interpreter::{InterpreterInfo, NONOS_INTERP};
use crate::elf::loader::ElfImage;

use super::state::ProcessBuilder;

impl<'a> ProcessBuilder<'a> {
    pub(super) fn load_interpreter(&mut self, path: &str) -> ElfResult<ElfImage> {
        let info = InterpreterInfo::from(path);
        if !info.is_nonos_ld() {
            return Err(ElfError::InterpreterNotFound);
        }
        let registry = self.embedded_registry.ok_or(ElfError::InterpreterNotFound)?;
        for candidate in [info.as_str(), info.filename(), NONOS_INTERP] {
            if let Ok(image) = load_embedded_library(registry, self.loader, candidate) {
                return Ok(image);
            }
        }
        Err(ElfError::InterpreterNotFound)
    }
}
