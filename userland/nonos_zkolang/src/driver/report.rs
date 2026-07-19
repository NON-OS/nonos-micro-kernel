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

//! What a proving run produced: the verdict, the trace shape a fee is priced from,
//! the public outputs, and the commitment the proof is bound to.

use alloc::vec::Vec;

/// The result of a proving run.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Report {
    /// Whether the verifier accepted the proof. For an honest run this is true; it
    /// being false would signal a prover or AIR defect, not a bad program, since a
    /// bad program fails earlier with `RunError`.
    pub verified: bool,
    /// The number of instructions the VM executed, the trace rows before padding.
    pub steps: usize,
    /// The log2 of the padded trace length.
    pub log_trace_len: u32,
    /// The padded trace length, a power of two.
    pub trace_len: usize,
    /// The trace width the AIR proves over.
    pub trace_width: usize,
    /// The public outputs the program exposed, in declaration order.
    pub outputs: Vec<u64>,
    /// The 32-byte program commitment the proof is bound to, the on-chain
    /// `programCommit` a proving job is posted against.
    pub program_commit: [u8; 32],
}
