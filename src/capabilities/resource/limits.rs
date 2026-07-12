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

//! The quota-check arithmetic of a resource token, factored out of
//! `ResourceToken` so it holds no token and can be included by the
//! `mechanism_proofs` crate and checked against the Lean `Nonos.Quota` model.
//! `has_bytes` and `has_ops` delegate here.

/// Whether `remaining` covers a request of `amount`.
pub(crate) const fn has_at_least(remaining: u64, amount: u64) -> bool {
    remaining >= amount
}
