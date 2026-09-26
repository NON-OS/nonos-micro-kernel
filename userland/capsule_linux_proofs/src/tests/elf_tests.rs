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


//! The ELF reader, against a real dynamic image.

use crate::image::elf::{Elf, ET_DYN, PT_INTERP, PT_LOAD};

/// The loader never reads the dynamic section, so it names no constant
/// for it. The interpreter is what parses that, which is the point.
const PT_DYNAMIC: u32 = 2;

const IMAGE: &[u8] = include_bytes!("../../vectors/dynamic.elf");

#[test]
fn the_header_reads_as_a_shared_object_for_x86_64() {
    let elf = Elf::parse(IMAGE).expect("a 64-bit little-endian x86_64 image");
    assert_eq!(elf.kind, ET_DYN);
    assert_eq!(elf.entry, 0x1000);
    assert_eq!(elf.phnum(), 3);
    assert_eq!(elf.phentsize, 56);
}

#[test]
fn the_program_headers_are_where_the_header_says() {
    let elf = Elf::parse(IMAGE).unwrap();
    let kinds: Vec<u32> = (0..elf.phnum()).map(|i| elf.phdr(i).unwrap().kind).collect();
    assert_eq!(kinds, vec![PT_LOAD, PT_INTERP, PT_DYNAMIC]);
}

#[test]
fn the_interpreter_path_is_the_one_in_the_file() {
    let elf = Elf::parse(IMAGE).unwrap();
    let ph = (0..elf.phnum())
        .map(|i| elf.phdr(i).unwrap())
        .find(|p| p.kind == PT_INTERP)
        .expect("PT_INTERP");
    let raw = &IMAGE[ph.offset as usize..(ph.offset + ph.filesz) as usize];
    let end = raw.iter().position(|b| *b == 0).unwrap();
    assert_eq!(&raw[..end], b"/lib/ld-musl-x86_64.so.1");
}

#[test]
fn the_header_address_is_inside_a_load_segment() {
    let elf = Elf::parse(IMAGE).unwrap();
    let at = elf.phdr_addr(0).expect("headers are in a load segment");
    assert_eq!(at, 0x400000 + elf.phoff);
    assert_eq!(elf.phdr_addr(0x1000_0000), Some(0x1000_0000 + 0x400000 + elf.phoff));
}

#[test]
fn anything_that_is_not_an_image_is_refused() {
    assert!(Elf::parse(b"").is_none());
    assert!(Elf::parse(&[0u8; 64]).is_none());
    let mut wrong = IMAGE.to_vec();
    wrong[4] = 1;
    assert!(Elf::parse(&wrong).is_none(), "a 32-bit class must be refused");
}
