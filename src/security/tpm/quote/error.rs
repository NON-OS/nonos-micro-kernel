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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteError {
    /// The response ended inside a field. Never treated as an empty value:
    /// a short read and an absent signature must not look alike.
    Truncated,
    /// The TPM rejected the command and returned this response code.
    Tpm(u32),
    /// The signed structure did not begin with `TPM_GENERATED_VALUE`, so it
    /// was not produced inside a TPM.
    NotTpmGenerated,
    /// The signed structure was not an attestation of type quote.
    NotAQuote,
    /// The nonce in the signed structure did not match the one requested, so
    /// the quote answers a different question than the one asked.
    NonceMismatch,
}

impl QuoteError {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Truncated => "tpm quote response truncated",
            Self::Tpm(_) => "tpm rejected the quote command",
            Self::NotTpmGenerated => "attest structure not tpm generated",
            Self::NotAQuote => "attest structure is not a quote",
            Self::NonceMismatch => "quote nonce does not match the challenge",
        }
    }
}
