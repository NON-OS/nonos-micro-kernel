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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    /// The transport failed: no part, a timeout, or bytes that do not parse.
    Tpm(TpmError),
    /// The part answered a command with a response code other than success.
    /// Carried whole because the code says why: `TPM_RC_POLICY_FAIL` means the
    /// PCRs are not what the key was derived under, and that is the message.
    Refused(u32),
    /// Longer than [`super::consts::LABEL_MAX`], or empty.
    BadLabel,
}

impl From<TpmError> for KeyError {
    fn from(e: TpmError) -> Self {
        KeyError::Tpm(e)
    }
}

impl KeyError {
    pub const fn as_str(self) -> &'static str {
        match self {
            KeyError::Tpm(e) => e.as_str(),
            KeyError::Refused(_) => "tpm refused the command",
            KeyError::BadLabel => "label empty or too long",
        }
    }
}
