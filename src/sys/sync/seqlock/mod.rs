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

//! A sequence lock for read-mostly data. Reads are wait-free and always
//! consistent; a reader retries while a writer is mid-update. The sequence
//! discipline lives in `pure` and is checked against the Lean `Nonos.Seqlock`
//! model by the `sync_proofs` crate.

mod new;
mod pure;
mod read;
mod state;
mod write;

pub use state::SeqLock;
