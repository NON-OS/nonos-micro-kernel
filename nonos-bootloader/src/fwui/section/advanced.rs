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

use super::info::info;
use super::row::Row;
use crate::fwui::data::Sys;
use crate::fwui::theme;
use crate::paging::supports_1gib_pages;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn advanced(sys: &Sys) -> Vec<Row> {
    let f = |b: bool| -> (String, u32) {
        if b {
            ("ENABLED".to_string(), theme::OK)
        } else {
            ("-".to_string(), theme::MUTE)
        }
    };
    let (nx, nxc) = f(sys.feat.nxe);
    let (smep, smepc) = f(sys.feat.smep);
    let (smap, smapc) = f(sys.feat.smap);
    let (umip, umipc) = f(sys.feat.umip);
    let g1 = supports_1gib_pages();
    let (pages, pagesc) = if g1 {
        ("1 GiB + 2 MiB".to_string(), theme::OK)
    } else {
        ("2 MiB only".to_string(), theme::ACCENT)
    };
    vec![
        info(b"NX / XD", nx, nxc, b"No-Execute page protection (CPUID 80000001 EDX bit 20)."),
        info(b"SMEP", smep, smepc, b"Supervisor Mode Execution Prevention (CPUID 7 EBX bit 7)."),
        info(b"SMAP", smap, smapc, b"Supervisor Mode Access Prevention (CPUID 7 EBX bit 20)."),
        info(b"UMIP", umip, umipc, b"User-Mode Instruction Prevention (CPUID 7 ECX bit 2)."),
        info(
            b"1 GIB PAGES",
            if g1 { "SUPPORTED".to_string() } else { "ABSENT".to_string() },
            pagesc,
            b"CPUID 80000001 EDX bit 26. Loader falls back to 2 MiB when absent.",
        ),
        info(
            b"PAGING LEAVES",
            pages,
            pagesc,
            b"Hugepage sizes the loader maps the identity range and directmap with.",
        ),
    ]
}
