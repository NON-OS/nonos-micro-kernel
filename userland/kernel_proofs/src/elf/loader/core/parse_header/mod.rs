// NONOS Operating System (AGPL-3.0-or-later)
// The real ELF header + program-header-bounds parsers (pub(crate); re-exported
// at crate scope for the proofs).
#[path = "../../../../../../../src/elf/loader/core/parse_header/header.rs"]
mod header;
#[path = "../../../../../../../src/elf/loader/core/parse_header/bounds.rs"]
mod bounds;

pub(crate) use bounds::program_header_bounds;
pub(crate) use header::parse_elf_header;
