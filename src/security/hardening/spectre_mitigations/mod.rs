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

mod barriers;
mod constants;
mod cpuid;
mod detect;
mod enable;
mod hooks;
mod ibpb;
mod ibrs;
mod init;
mod l1d;
mod mds;
mod msr;
mod report;
mod retpoline;
mod rsb;
mod ssbd;
mod state;
mod stibp;
mod types;

pub use barriers::{array_access_nospec, array_index_mask_nospec, lfence, mfence, sfence};
pub use detect::detect_vulnerabilities;
pub use enable::enable_mitigations;
pub use hooks::{context_switch_mitigations, kernel_entry_mitigations, kernel_exit_mitigations};
pub use ibpb::ibpb;
pub use ibrs::{ibrs_disable, ibrs_enable};
pub use init::{are_mitigations_enabled, get_mitigation_status, get_vulnerabilities, init};
pub use l1d::l1d_flush;
pub use mds::mds_clear;
pub use retpoline::{
    __x86_indirect_thunk_r8, __x86_indirect_thunk_rax, __x86_indirect_thunk_rbx,
    __x86_indirect_thunk_rcx, __x86_indirect_thunk_rdi, __x86_indirect_thunk_rdx,
    __x86_indirect_thunk_rsi,
};
pub use rsb::{rsb_clear, rsb_fill};
pub use ssbd::{ssbd_disable, ssbd_enable};
pub use stibp::{stibp_disable, stibp_enable};
pub use types::{CpuVulnerabilities, MitigationStatus};
