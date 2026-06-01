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

extern crate alloc;

use super::build_context::context_for;
use super::dispatch::apply;
use super::state::RelocationValues;
use crate::elf::errors::ElfError;
use crate::elf::loader::ElfImage;
use crate::elf::types::RelaEntry;
use crate::memory::addr::VirtAddr;
use alloc::collections::BTreeMap;
use alloc::string::String;

use super::super::context::RelocationContext;

pub fn process_relocations_with_context(
    image: &ElfImage,
    rela_entries: &[RelaEntry],
    context: &RelocationContext,
) -> Result<(), ElfError> {
    for rela in rela_entries {
        let values = RelocationValues::resolve(image, rela, context)?;
        unsafe { apply(values)? };
    }
    Ok(())
}

pub fn process_relocations(image: &ElfImage, rela_entries: &[RelaEntry]) -> Result<(), ElfError> {
    let empty_cache: BTreeMap<String, VirtAddr> = BTreeMap::new();
    let context = context_for(image, &empty_cache);
    process_relocations_with_context(image, rela_entries, &context)
}
