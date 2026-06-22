// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod as_bytes;
mod embedded;
mod invalid;
mod load_sidecar;
mod select;
mod sidecar;
mod types;

pub use as_bytes::proof_source_bytes;
pub use invalid::proof_source_is_invalid_sidecar;
pub use select::select_zk_proof_source;
pub use sidecar::proof_source_is_sidecar;
pub use types::ProofSource;
