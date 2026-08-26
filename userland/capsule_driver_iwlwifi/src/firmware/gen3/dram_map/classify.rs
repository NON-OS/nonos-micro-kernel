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

//! Partition the runtime sections into LMAC, UMAC and paged images.

use alloc::vec::Vec;

use super::super::image::sections;
use super::markers::{SEP_CPU1_CPU2, SEP_PAGING};
use super::types::FwLayout;

/// Walk the runtime sections and split them at the two separators: the run
/// before the CPU1/CPU2 separator is LMAC, the run before the paging separator
/// is UMAC, the rest is paged. The separator sections advance the phase and are
/// dropped; every other section joins the image for the current phase.
pub fn classify(blob: &[u8]) -> FwLayout<'_> {
    let mut out = FwLayout { lmac: Vec::new(), umac: Vec::new(), virt: Vec::new() };
    let mut phase = 0u8;
    for s in sections(blob) {
        match s.load_offset {
            SEP_CPU1_CPU2 if phase == 0 => phase = 1,
            SEP_PAGING if phase <= 1 => phase = 2,
            _ => match phase {
                0 => out.lmac.push(s),
                1 => out.umac.push(s),
                _ => out.virt.push(s),
            },
        }
    }
    out
}
