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


//! The key that signs attestations, and where this machine's identity comes
//! from.
//!
//! Derived rather than stored. A primary key under the endorsement hierarchy
//! is a function of the TPM's seed and a fixed template, so the same part
//! reproduces the same key on every boot with nothing kept on disk. An
//! amnesic machine therefore still has an identity a counterparty can pin.

mod create;
mod load;
mod template;

pub use load::{ak_handle, load_ak};
