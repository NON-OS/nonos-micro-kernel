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

use super::common::parse_common;
use super::constants::{MAX_PT, TAG_LEN};
use super::types::{FrameError, OpenFrame, SealFrame};

pub(crate) fn parse_seal(payload: &[u8]) -> Result<SealFrame<'_>, FrameError> {
    let parts = parse_common(payload)?;
    if parts.body.len() > MAX_PT {
        return Err(FrameError::OversizePayload);
    }
    Ok(SealFrame { key: parts.key, nonce: parts.nonce, aad: parts.aad, plaintext: parts.body })
}

pub(crate) fn nonce_is_degenerate(nonce: &[u8]) -> bool {
    let mut acc: u8 = 0;
    for byte in nonce {
        acc |= *byte;
    }
    acc == 0
}

pub(crate) fn parse_open(payload: &[u8]) -> Result<OpenFrame<'_>, FrameError> {
    let parts = parse_common(payload)?;
    if parts.body.len() < TAG_LEN {
        return Err(FrameError::Short);
    }
    if parts.body.len() > MAX_PT + TAG_LEN {
        return Err(FrameError::OversizePayload);
    }
    Ok(OpenFrame { key: parts.key, nonce: parts.nonce, aad: parts.aad, ciphertext: parts.body })
}
