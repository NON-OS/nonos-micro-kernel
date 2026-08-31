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

//! The answer to "what is this machine running", in a form a stranger can
//! check.
//!
//! A TPM quote proves what booted. The registry knows what is running now.
//! Neither alone is the claim anyone wants; the document is the join, and the
//! join is only sound because the registry root is folded into the value the
//! TPM signs rather than carried beside it.

mod attest;
mod binding;
mod document;
mod error;
mod produce;

pub use attest::attest;
pub use binding::qualifying_data;
pub use document::{AttestationDoc, DOC_MAGIC, DOC_VERSION};
pub use error::AttestDocError;
