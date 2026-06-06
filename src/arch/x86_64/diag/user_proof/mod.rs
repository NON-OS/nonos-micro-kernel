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

//! Pre-iretq audit. Walks the target CR3 through the directmap and
//! proves the architectural preconditions for entering CPL=3:
//! kernel-half PML4 entries (256 directmap, 511 kernel text) are
//! present, the user RIP leaf is present + user-accessible +
//! executable, the user RSP leaf is present + user-accessible +
//! writable + NX, and the trap substrate (TSS.RSP0 plus the IST
//! slots used by #DF, #PF, #GP) is non-zero.
//!
//! On any failure prints `[USER-PROOF] FAIL <reason>` and refuses
//! to iretq. Production builds compile this path out; smoketest
//! profiles enable `nonos-user-entry-proof`.

use crate::arch::x86_64::gdt::{
    constants::{IST_DOUBLE_FAULT, IST_GP, IST_PAGE_FAULT},
    get_ist, get_kernel_stack,
};

mod entry_bytes;
mod gs_state;
mod page_walk;
mod report;

const PML4_DIRECTMAP: usize = 256;
const PML4_KERNEL_TEXT: usize = 511;

pub fn assert_user_entry(cr3: u64, user_rip: u64, user_rsp: u64, cpu_id: u32) -> bool {
    let ok_directmap = page_walk::pml4_present(cr3, PML4_DIRECTMAP);
    if !ok_directmap {
        report::fail(b"PML4[256] missing", cr3);
        return false;
    }
    if !page_walk::pml4_present(cr3, PML4_KERNEL_TEXT) {
        report::fail(b"PML4[511] missing", cr3);
        return false;
    }

    let rip_req = page_walk::LeafReq { user: true, writable: false, executable: true };
    if !page_walk::leaf_satisfies(cr3, user_rip, rip_req) {
        report::fail(b"user RIP leaf bad", user_rip);
        return false;
    }

    let rsp_req = page_walk::LeafReq { user: true, writable: true, executable: false };
    if !page_walk::leaf_satisfies(cr3, user_rsp - 8, rsp_req) {
        report::fail(b"user RSP leaf bad", user_rsp);
        return false;
    }

    let rsp0 = get_kernel_stack(cpu_id).unwrap_or(0);
    if rsp0 == 0 {
        report::fail(b"TSS.RSP0 zero", 0);
        return false;
    }
    let gs = gs_state::read();
    if gs.rsp0 != rsp0 {
        report::fail(b"GS RSP0 mirror mismatch", gs.rsp0);
        return false;
    }
    if gs.base == 0 || gs.kernel_base != 0 {
        report::fail(b"GS swap state bad", gs.kernel_base);
        return false;
    }

    let ist_df = get_ist(cpu_id, IST_DOUBLE_FAULT).unwrap_or(0);
    let ist_pf = get_ist(cpu_id, IST_PAGE_FAULT).unwrap_or(0);
    let ist_gp = get_ist(cpu_id, IST_GP).unwrap_or(0);
    if ist_df == 0 || ist_pf == 0 || ist_gp == 0 {
        report::fail(b"IST slot zero", 0);
        return false;
    }

    if let Some(leaf) = page_walk::leaf_for(cr3, user_rip) {
        entry_bytes::print(user_rip, leaf);
    }
    report::ok(cr3, user_rip, user_rsp, rsp0, gs, ist_df, ist_pf, ist_gp);
    true
}
