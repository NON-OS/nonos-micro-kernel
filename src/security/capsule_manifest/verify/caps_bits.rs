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

//! The capability arithmetic of the spawn gate, holding no manifest so it can
//! be included by `mechanism_proofs` and checked against `Nonos.SpawnCaps`.
//! `check_ceiling` and `check_grant` delegate here.
//!
//! What these three decide is the capability word a capsule runs with. The
//! ceiling comes from the publisher's signed NØNOS ID certificate, and the
//! result flows unmodified to `proc_caps::install_spawn`, so nothing between
//! here and the PCB narrows it again.

/// Whether a manifest asks only for authority the publisher's certificate
/// permits. Both the required and optional sets are covered: an optional
/// capability is still one the capsule can end up holding.
pub(crate) const fn within_ceiling(required: u64, optional: u64, ceiling: u64) -> bool {
    (required | optional) & !ceiling == 0
}

/// Whether the caller's grant stays inside what the manifest declares.
pub(crate) const fn grant_within_manifest(required: u64, optional: u64, granted: u64) -> bool {
    granted & !(required | optional) == 0
}

/// The capability word the capsule installs with.
///
/// Required capabilities are not attenuated by the grant: a capsule that
/// cannot get them cannot run correctly, so the manifest declaring them is
/// what admits them. The grant only decides which optional ones come along.
/// That makes the certificate ceiling, not the grant, the upper bound worth
/// proving.
pub(crate) const fn install_caps(required: u64, optional: u64, granted: u64) -> u64 {
    required | (optional & granted)
}
