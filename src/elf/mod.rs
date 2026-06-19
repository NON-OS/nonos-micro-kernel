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

pub mod aslr;
pub mod auxv;
pub mod cache;
pub mod dynlink;
pub mod embedded;
pub mod errors;
pub mod fini;
pub mod got;
pub mod hash;
pub mod init;
pub mod interpreter;
pub mod libmgr;
pub mod loader;
pub mod minimal;
pub mod process;
pub mod reloc;
pub mod stack;
pub mod symbol;
pub mod tls;
pub mod types;

pub use types::{DynamicEntry, ElfHeader, ProgramHeader, RelaEntry, SectionHeader, Symbol};

pub use types::{
    elf_class, elf_data, elf_machine, elf_osabi, elf_type, phdr_flags, phdr_type, reloc_type,
    shdr_flags, shdr_type, symbol_bind, symbol_type, ELF_MAGIC,
};

pub use errors::{ElfError, ElfResult};

pub use loader::{
    get_elf_loader, init_elf_loader, is_initialized, load_elf_entry_into, load_elf_executable,
    DynamicInfo, ElfImage, ElfLoader, LoadedSegment,
};

pub use aslr::AslrManager;
pub use auxv::{aux_type, AuxEntry, AuxvBuilder};
pub use cache::{CacheEntryState, CachedImage, ImageCache};
pub use dynlink::DynLinkInfo;
pub use embedded::{
    EmbeddedLibrary, EmbeddedLibraryLoader, EmbeddedLibraryRegistry, LibraryVersion,
};
pub use fini::{FiniArrayInfo, FiniArrayRunner, FiniFn};
pub use got::{GlobalOffsetTable, GotEntry, RelocationProcessor};
pub use hash::{gnu_hash, sysv_hash, DualHashLookup, GnuHashTable, HashTable, SysvHashTable};
pub use init::{InitArrayInfo, InitArrayRunner, InitFn, PreInitArrayInfo};
pub use interpreter::InterpreterInfo;
pub use libmgr::{LibraryManager, LibraryState, LinkMap, LoadedLibrary};
pub use process::{
    create_process, create_process_with_args, ProcessBuilder, ProcessConfig, ProcessImage,
};
pub use reloc::process_relocations;
pub use stack::{setup_user_stack, StackConfig, StackLayout};
pub use symbol::{ResolvedSymbol, SymbolLookup, SymbolResolver};
pub use tls::TlsInfo;

pub use minimal::{entry_from_bytes, validate_elf, validate_elf_detailed};
