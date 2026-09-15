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

//! A bounds-checked walk over a TPM response.
//!
//! Every field is fetched through here, so a short or malformed response is
//! refused at the field that ran out rather than read past its end.

use crate::security::tpm::error::TpmError;

pub(super) struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn at(buf: &'a [u8], pos: usize) -> Self {
        Self { buf, pos }
    }
    pub(super) fn position(&self) -> usize {
        self.pos
    }
    pub(super) fn take(&mut self, n: usize) -> Result<&'a [u8], TpmError> {
        let end = self.pos.checked_add(n).ok_or(TpmError::InvalidResponse)?;
        let out = self.buf.get(self.pos..end).ok_or(TpmError::InvalidResponse)?;
        self.pos = end;
        Ok(out)
    }
    pub(super) fn skip(&mut self, n: usize) -> Result<(), TpmError> {
        self.take(n).map(|_| ())
    }
    pub(super) fn u16(&mut self) -> Result<u16, TpmError> {
        let b = self.take(2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]))
    }
    pub(super) fn u32(&mut self) -> Result<u32, TpmError> {
        let b = self.take(4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
}
