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

use crate::security::tpm::error::TpmError;
use crate::security::tpm::quote::QuoteError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttestDocError {
    /// A running capsule went unrecorded at some point, so the machine can no
    /// longer describe itself completely and refuses to try.
    RegistryIncomplete,
    Tpm(TpmError),
    Quote(QuoteError),
}

impl AttestDocError {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryIncomplete => "attestation registry incomplete",
            Self::Tpm(e) => e.as_str(),
            Self::Quote(e) => e.as_str(),
        }
    }
}
