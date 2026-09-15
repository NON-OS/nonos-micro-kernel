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

//! Host proofs for the attestation key's public half and the document that
//! carries it. The kernel's parser and encoder are included by `#[path]`, so
//! the bytes under test are the bytes that ship.

extern crate alloc;

pub mod security;

#[cfg(test)]
mod document_tests;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod public_tests;
