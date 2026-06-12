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

use crate::loader::*;

#[test]
fn test_elf_magic() {
    assert_eq!(types::ELF_MAGIC, [0x7f, b'E', b'L', b'F']);
}

#[test]
fn test_page_constants() {
    assert_eq!(types::memory::PAGE_SIZE, 0x1000);
    assert_eq!(types::memory::PAGE_SHIFT, 12);
}

#[test]
fn test_page_align_functions() {
    assert_eq!(types::memory::page_align_down(0x1234), 0x1000);
    assert_eq!(types::memory::page_align_up(0x1234), 0x2000);
    assert_eq!(types::memory::page_align_up(0x1000), 0x1000);
    assert_eq!(types::memory::pages_needed(0x1000), 1);
    assert_eq!(types::memory::pages_needed(0x1001), 2);
}

#[test]
fn test_elf64_header_validation() {
    let mut header = types::Elf64Header {
        e_ident: [0; 16],
        e_type: types::elf_type::ET_EXEC,
        e_machine: types::elf_machine::EM_X86_64,
        e_version: 1,
        e_entry: 0x100000,
        e_phoff: 64,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 64,
        e_phentsize: 56,
        e_phnum: 2,
        e_shentsize: 0,
        e_shnum: 0,
        e_shstrndx: 0,
    };

    header.e_ident[0..4].copy_from_slice(&types::ELF_MAGIC);
    header.e_ident[4] = types::elf_class::ELFCLASS64;
    header.e_ident[5] = types::elf_data::ELFDATA2LSB;
    header.e_ident[6] = 1;

    assert!(header.is_valid());
    assert!(header.is_executable());
    assert!(header.is_x86_64());
}

#[test]
fn test_program_header_flags() {
    let phdr = types::Elf64Phdr {
        p_type: types::ph_type::PT_LOAD,
        p_flags: types::ph_flags::PF_R | types::ph_flags::PF_X,
        p_offset: 0,
        p_vaddr: 0x100000,
        p_paddr: 0x100000,
        p_filesz: 0x1000,
        p_memsz: 0x1000,
        p_align: 0x1000,
    };

    assert!(phdr.is_load());
    assert!(phdr.is_readable());
    assert!(phdr.is_executable());
    assert!(!phdr.is_writable());
}

#[test]
fn test_loaded_segment() {
    let phdr = types::Elf64Phdr {
        p_type: types::ph_type::PT_LOAD,
        p_flags: types::ph_flags::PF_R | types::ph_flags::PF_W,
        p_offset: 0x1000,
        p_vaddr: 0x200000,
        p_paddr: 0,
        p_filesz: 0x2000,
        p_memsz: 0x3000,
        p_align: 0x1000,
    };

    let segment = types::LoadedSegment::from_phdr(&phdr);

    assert_eq!(segment.file_offset, 0x1000);
    assert_eq!(segment.file_size, 0x2000);
    assert_eq!(segment.mem_size, 0x3000);
    assert_eq!(segment.target_addr, 0x200000);
    assert_eq!(segment.bss_size(), 0x1000);
    assert!(!segment.has_wx());
}

#[test]
fn test_wx_detection() {
    let wx_phdr = types::Elf64Phdr {
        p_type: types::ph_type::PT_LOAD,
        p_flags: types::ph_flags::PF_R | types::ph_flags::PF_W | types::ph_flags::PF_X,
        p_offset: 0,
        p_vaddr: 0x100000,
        p_paddr: 0,
        p_filesz: 0x1000,
        p_memsz: 0x1000,
        p_align: 0x1000,
    };

    let segment = types::LoadedSegment::from_phdr(&wx_phdr);
    assert!(segment.has_wx());
}

#[test]
fn test_error_categories() {
    assert_eq!(errors::LoaderError::InvalidMagic.category(), "parse");
    assert_eq!(errors::LoaderError::NoLoadableSegments.category(), "segment");
    assert_eq!(errors::LoaderError::OutOfMemory.category(), "memory");
    assert_eq!(errors::LoaderError::WxViolation.category(), "security");
    assert_eq!(errors::LoaderError::CapsuleInvalid.category(), "capsule");
}

#[test]
fn test_security_error_detection() {
    assert!(errors::LoaderError::WxViolation.is_security_error());
    assert!(errors::LoaderError::SignatureInvalid.is_security_error());
    assert!(!errors::LoaderError::FileNotFound.is_security_error());
}

#[test]
fn test_fatal_error_detection() {
    assert!(errors::LoaderError::SignatureInvalid.is_fatal());
    assert!(errors::LoaderError::HashMismatch.is_fatal());
    assert!(!errors::LoaderError::FileNotFound.is_fatal());
}
