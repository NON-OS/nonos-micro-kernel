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

//! `receipt`: what the kernel recorded about every running capsule.
//!
//! These are the entries the attestation registry root folds, read straight
//! from the kernel rather than from any capsule's account of itself. Off the
//! machine, `nonos-receipt` refolds them and checks the result against a root
//! a TPM signed; `receipt --hex` prints them in the form that tool reads.

mod hexdump;
mod row;
mod run;
mod summary;

pub use run::run;
