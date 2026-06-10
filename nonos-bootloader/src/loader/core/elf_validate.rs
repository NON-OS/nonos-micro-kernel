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

extern crate alloc;

use alloc::format;
use goblin::elf::{header, program_header, Elf};

use crate::loader::errors::{LoaderError, LoaderResult};
use crate::log::logger::{log_debug, log_error, log_info};

use super::constants::MAX_LOADS;
use super::segment_check::{
    compute_segment_bounds, finalize_bounds, validate_kernel_size, validate_single_segment,
};
use super::types::{ValidatedSegment, ValidationResult};

pub fn validate_elf(payload: &[u8]) -> LoaderResult<ValidationResult<'_>> {
    log_info("loader", "Parsing ELF binary...");
    log_info("loader", &format!("Payload size: {} bytes", payload.len()));

    if payload.len() < 64 {
        log_error("loader", "Payload too small for ELF header");
        return Err(LoaderError::ElfParseError("payload too small"));
    }
    if &payload[0..4] != b"\x7fELF" {
        log_error(
            "loader",
            &format!(
                "Invalid ELF magic: {:02x}{:02x}{:02x}{:02x}",
                payload[0], payload[1], payload[2], payload[3]
            ),
        );
        return Err(LoaderError::ElfParseError("invalid ELF magic"));
    }

    let elf = parse_elf_header(payload)?;

    validate_elf_arch(&elf)?;

    let (is_exec, is_dyn) = validate_elf_type(&elf)?;

    let (loads, load_count, min_addr, max_addr) = validate_segments(&elf, payload, is_exec)?;

    Ok(ValidationResult { elf, loads, load_count, min_addr, max_addr, is_exec, is_dyn })
}

fn parse_elf_header(payload: &[u8]) -> LoaderResult<Elf<'_>> {
    let elf = Elf::parse(payload).map_err(|e| {
        log_error("loader", &format!("Goblin error: {:?}", e));
        let short_err = match &e {
            goblin::error::Error::Malformed(s) => {
                log_error("loader", &format!("Malformed: {}", s));
                "malformed"
            }
            goblin::error::Error::BadMagic(_) => "bad magic",
            goblin::error::Error::Scroll(s) => {
                log_error("loader", &format!("Scroll: {}", s));
                "scroll"
            }
            _ => "parse error",
        };
        LoaderError::ElfParseError(short_err)
    })?;

    log_info("loader", "ELF parsed successfully");
    Ok(elf)
}

fn validate_elf_arch(elf: &Elf) -> LoaderResult<()> {
    if !elf.is_64 {
        log_error("loader", "ELF is not 64-bit.");
        return Err(LoaderError::UnsupportedElf("not 64-bit"));
    }
    if elf.header.e_machine != header::EM_X86_64 {
        log_error("loader", "ELF machine is not x86_64.");
        return Err(LoaderError::UnsupportedElf("non-x86_64"));
    }
    Ok(())
}

fn validate_elf_type(elf: &Elf) -> LoaderResult<(bool, bool)> {
    let is_exec = elf.header.e_type == header::ET_EXEC;
    let is_dyn = elf.header.e_type == header::ET_DYN;
    if !is_exec && !is_dyn {
        log_error("loader", "Unsupported ELF type.");
        return Err(LoaderError::UnsupportedElf("unsupported e_type"));
    }

    log_info("loader", &format!("ELF type: {}", if is_exec { "ET_EXEC" } else { "ET_DYN" }));
    Ok((is_exec, is_dyn))
}

fn validate_segments(
    elf: &Elf,
    payload: &[u8],
    is_exec: bool,
) -> LoaderResult<([ValidatedSegment; MAX_LOADS], usize, u64, u64)> {
    let mut loads: [ValidatedSegment; MAX_LOADS] =
        core::array::from_fn(|_| ValidatedSegment::default());
    let mut load_count: usize = 0;
    let mut min_addr: Option<u64> = None;
    let mut max_addr: Option<u64> = None;

    for ph in &elf.program_headers {
        if ph.p_type != program_header::PT_LOAD {
            continue;
        }
        if load_count >= MAX_LOADS {
            log_error("loader", "too many PT_LOADs for fixed table");
            return Err(LoaderError::AllocationTableFull);
        }

        let seg = validate_single_segment(ph, payload, is_exec)?;
        let (seg_start, seg_end) = compute_segment_bounds(&seg, is_exec)?;

        min_addr = Some(min_addr.map_or(seg_start, |m| m.min(seg_start)));
        max_addr = Some(max_addr.map_or(seg_end, |m| m.max(seg_end)));

        loads[load_count] = seg;
        load_count += 1;
        log_debug("loader", "Validated PT_LOAD segment");
    }

    if load_count == 0 {
        log_error("loader", "No PT_LOAD segments found in ELF payload.");
        return Err(LoaderError::NoLoadableSegments);
    }

    let (base, end) = finalize_bounds(min_addr, max_addr)?;
    validate_kernel_size(base, end)?;

    Ok((loads, load_count, base, end))
}
