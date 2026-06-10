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

use crate::loader::errors::LoaderError;
use crate::log::logger::{log_debug, log_error, log_info};

use super::types::{reloc_type, Dyn64, Rela64, RelocationContext};

pub fn process_relocations(ctx: &RelocationContext) -> Result<usize, LoaderError> {
    let mut reloc_count = 0;

    if let Some(rela_addr) = ctx.rela_addr {
        if ctx.rela_size > 0 && ctx.rela_ent > 0 {
            let entry_count = ctx.rela_size / ctx.rela_ent;
            log_debug("reloc", "Processing RELA relocations");

            for i in 0..entry_count {
                let rela = unsafe {
                    let ptr = (rela_addr as *const u8).add(i * ctx.rela_ent) as *const Rela64;
                    &*ptr
                };

                apply_relocation(rela, ctx)?;
                reloc_count += 1;
            }
        }
    }

    if let Some(jmprel_addr) = ctx.jmprel_addr {
        if ctx.jmprel_size > 0 && ctx.rela_ent > 0 {
            let entry_count = ctx.jmprel_size / ctx.rela_ent;
            log_debug("reloc", "Processing PLT relocations");

            for i in 0..entry_count {
                let rela = unsafe {
                    let ptr = (jmprel_addr as *const u8).add(i * ctx.rela_ent) as *const Rela64;
                    &*ptr
                };

                apply_relocation(rela, ctx)?;
                reloc_count += 1;
            }
        }
    }

    if reloc_count > 0 {
        log_info("reloc", "Relocations processed successfully");
    }

    Ok(reloc_count)
}

fn apply_relocation(rela: &Rela64, ctx: &RelocationContext) -> Result<(), LoaderError> {
    let r_type = rela.reloc_type();
    let target_addr = ctx.base_addr.wrapping_add(rela.r_offset);
    let addend = rela.r_addend;

    unsafe {
        match r_type {
            reloc_type::R_X86_64_NONE => {}
            reloc_type::R_X86_64_RELATIVE => {
                let value = (ctx.base_addr as i64).wrapping_add(addend) as u64;
                let target_ptr = target_addr as *mut u64;
                *target_ptr = value;
            }
            reloc_type::R_X86_64_64 => {
                let target_ptr = target_addr as *mut u64;
                *target_ptr = (*target_ptr).wrapping_add(ctx.load_bias as u64);
            }
            reloc_type::R_X86_64_GLOB_DAT | reloc_type::R_X86_64_JUMP_SLOT => {
                let target_ptr = target_addr as *mut u64;
                *target_ptr = (*target_ptr).wrapping_add(ctx.load_bias as u64);
            }
            reloc_type::R_X86_64_PC32 | reloc_type::R_X86_64_PLT32 => {}
            reloc_type::R_X86_64_32 => {
                let target_ptr = target_addr as *mut u32;
                let value = (*target_ptr as i64).wrapping_add(ctx.load_bias) as u32;
                *target_ptr = value;
            }
            reloc_type::R_X86_64_32S => {
                let target_ptr = target_addr as *mut i32;
                let value = (*target_ptr as i64).wrapping_add(ctx.load_bias) as i32;
                *target_ptr = value;
            }
            reloc_type::R_X86_64_IRELATIVE => {
                let value = (ctx.base_addr as i64).wrapping_add(addend) as u64;
                let target_ptr = target_addr as *mut u64;
                *target_ptr = value;
            }
            _ => {
                log_error("reloc", "Unsupported relocation type");
                return Err(LoaderError::MalformedElf("unsupported relocation type"));
            }
        }
    }

    Ok(())
}

pub fn process_elf_relocations(
    elf: &goblin::elf::Elf,
    base_addr: u64,
    load_bias: i64,
    payload: &[u8],
) -> Result<usize, LoaderError> {
    use goblin::elf::program_header::PT_DYNAMIC;

    let dynamic_ph = elf.program_headers.iter().find(|ph| ph.p_type == PT_DYNAMIC);

    let dynamic_ph = match dynamic_ph {
        Some(ph) => ph,
        None => {
            log_debug("reloc", "No PT_DYNAMIC segment, skipping relocations");
            return Ok(0);
        }
    };

    let dyn_offset = dynamic_ph.p_offset as usize;
    let dyn_size = dynamic_ph.p_filesz as usize;

    if dyn_offset + dyn_size > payload.len() {
        return Err(LoaderError::SegmentOutOfBounds);
    }

    let mut ctx = RelocationContext::new(base_addr, load_bias);

    let dyn_entry_size = core::mem::size_of::<Dyn64>();
    let dyn_count = dyn_size / dyn_entry_size;

    let dyn_ptr = unsafe { payload.as_ptr().add(dyn_offset) as *const Dyn64 };
    ctx.parse_dynamic(dyn_ptr, dyn_count);

    process_relocations(&ctx)
}
