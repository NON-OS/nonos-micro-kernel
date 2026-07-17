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

//! The money-grade DEEP STARK proof: the trace stays in the base field, but the
//! out-of-domain frame, the composition openings, and the DEEP polynomial are all
//! in the extension, because the out-of-domain point is sampled at `z in Fp2`.

use super::super::field::{Fp, Fp2};
use super::super::fri_ext::FriProofExt;
use alloc::vec::Vec;

/// One consistency query at position `p`: the extension DEEP value, the whole
/// base-field trace row at `p`, and the extension composition at `p`. The trace row
/// is authenticated by a single wide-leaf path; the DEEP and composition values by
/// their own extension-leaf paths.
pub struct StarkQueryExt {
    pub deep: Fp2,
    pub deep_path: Vec<[u8; 32]>,
    pub trace: Vec<Fp>,
    pub trace_path: Vec<[u8; 32]>,
    pub comp: Fp2,
    pub comp_path: Vec<[u8; 32]>,
}

/// A complete money-grade STARK proof. The trace is committed row-wise as one
/// wide-leaf tree, so there is a single trace root and one trace path per query.
pub struct StarkProofExt {
    pub trace_root: [u8; 32],
    pub comp_root: [u8; 32],
    /// The trace columns evaluated at `g^k * z` for each window row `k`, row-major
    /// as a transition window: `ood_frame[k * width + col]`, in `Fp2`.
    pub ood_frame: Vec<Fp2>,
    pub fri: FriProofExt,
    pub queries: Vec<StarkQueryExt>,
}
