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

use super::constants::dyn_tag;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Dyn {
    pub d_tag: i64,
    pub d_val: u64,
}

impl Elf64Dyn {
    pub fn is_null(&self) -> bool {
        self.d_tag == dyn_tag::DT_NULL
    }

    pub fn is_needed(&self) -> bool {
        self.d_tag == dyn_tag::DT_NEEDED
    }

    pub fn is_rela(&self) -> bool {
        self.d_tag == dyn_tag::DT_RELA
    }

    pub fn is_rel(&self) -> bool {
        self.d_tag == dyn_tag::DT_REL
    }

    pub fn is_jmprel(&self) -> bool {
        self.d_tag == dyn_tag::DT_JMPREL
    }

    pub fn is_symtab(&self) -> bool {
        self.d_tag == dyn_tag::DT_SYMTAB
    }

    pub fn is_strtab(&self) -> bool {
        self.d_tag == dyn_tag::DT_STRTAB
    }
}

#[derive(Debug, Clone, Default)]
pub struct DynamicInfo {
    pub rela_addr: Option<u64>,
    pub rela_size: usize,
    pub rela_ent: usize,

    pub rel_addr: Option<u64>,
    pub rel_size: usize,
    pub rel_ent: usize,

    pub jmprel_addr: Option<u64>,
    pub jmprel_size: usize,
    pub pltrel_type: i64,

    pub symtab_addr: Option<u64>,
    pub syment: usize,

    pub strtab_addr: Option<u64>,
    pub strsz: usize,

    pub hash_addr: Option<u64>,
    pub gnu_hash_addr: Option<u64>,

    pub init_addr: Option<u64>,
    pub fini_addr: Option<u64>,

    pub init_array_addr: Option<u64>,
    pub init_array_size: usize,

    pub fini_array_addr: Option<u64>,
    pub fini_array_size: usize,

    pub pltgot_addr: Option<u64>,
    pub flags_1: u64,
}

impl DynamicInfo {
    pub fn has_relocations(&self) -> bool {
        self.rela_addr.is_some() || self.rel_addr.is_some() || self.jmprel_addr.is_some()
    }

    pub fn has_symbols(&self) -> bool {
        self.symtab_addr.is_some() && self.strtab_addr.is_some()
    }

    pub fn has_init(&self) -> bool {
        self.init_addr.is_some() || self.init_array_addr.is_some()
    }

    pub fn has_fini(&self) -> bool {
        self.fini_addr.is_some() || self.fini_array_addr.is_some()
    }

    pub fn rela_count(&self) -> usize {
        if self.rela_ent > 0 {
            self.rela_size / self.rela_ent
        } else {
            0
        }
    }

    pub fn rel_count(&self) -> usize {
        if self.rel_ent > 0 {
            self.rel_size / self.rel_ent
        } else {
            0
        }
    }

    pub fn jmprel_count(&self) -> usize {
        let ent_size =
            if self.pltrel_type == dyn_tag::DT_RELA { self.rela_ent } else { self.rel_ent };
        if ent_size > 0 {
            self.jmprel_size / ent_size
        } else {
            0
        }
    }

    pub fn total_relocation_count(&self) -> usize {
        self.rela_count() + self.rel_count() + self.jmprel_count()
    }

    pub fn init_array_count(&self) -> usize {
        self.init_array_size / 8
    }

    pub fn fini_array_count(&self) -> usize {
        self.fini_array_size / 8
    }
}
