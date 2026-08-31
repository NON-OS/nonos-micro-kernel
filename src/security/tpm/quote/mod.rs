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

//! `TPM2_Quote`: asking the TPM to sign its own PCR state under a challenge.
//!
//! This is what makes a measurement mean something off the machine. The PCRs
//! record what booted; a quote signs that record with a key only the TPM
//! holds, bound to a nonce the asker chose. Without the signature the values
//! are a claim, and without the nonce they are a recording.
//!
//! Marshalling only. The transport that carries these bytes to the part is
//! separate, so this layer is exercised without hardware.

mod attest;
mod command;
mod consts;
mod error;
mod pcr;
mod response;

pub use attest::check_attest;
pub use command::build_quote;
pub use consts::TPM_GENERATED_VALUE;
pub use error::QuoteError;
pub use response::{parse_quote, QuoteResult};
