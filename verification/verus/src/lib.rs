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

//! Verus-verified theorems about NONOS security algebra. Nothing here runs at
//! kernel time; it is checked by the Verus SMT verifier and is the machine-
//! checked layer of the proof strategy documented in `verification/README.md`.

pub mod capabilities;
pub mod ipc_lengths;
pub mod page_permissions;
pub mod stark_attestation;
