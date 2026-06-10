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

use crate::loader::types::{dyn_tag, DynamicInfo};

pub fn relocation_count(info: &DynamicInfo) -> usize {
    let mut count = 0;

    if info.rela_addr.is_some() && info.rela_ent > 0 {
        count += info.rela_size / info.rela_ent;
    }

    if info.rel_addr.is_some() && info.rel_ent > 0 {
        count += info.rel_size / info.rel_ent;
    }

    if info.jmprel_addr.is_some() {
        let ent_size =
            if info.pltrel_type == dyn_tag::DT_RELA { info.rela_ent } else { info.rel_ent };
        if ent_size > 0 {
            count += info.jmprel_size / ent_size;
        }
    }

    count
}

pub fn needs_relocations(info: &DynamicInfo) -> bool {
    info.rela_addr.is_some() || info.rel_addr.is_some() || info.jmprel_addr.is_some()
}

pub fn estimate_symbol_count(info: &DynamicInfo) -> Option<usize> {
    if info.symtab_addr.is_some() && info.syment > 0 {
        Some(info.strsz / 20)
    } else {
        None
    }
}
