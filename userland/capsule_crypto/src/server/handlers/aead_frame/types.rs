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

pub(crate) struct SealFrame<'a> {
    pub(crate) key: &'a [u8],
    pub(crate) nonce: &'a [u8],
    pub(crate) aad: &'a [u8],
    pub(crate) plaintext: &'a [u8],
}

pub(crate) struct OpenFrame<'a> {
    pub(crate) key: &'a [u8],
    pub(crate) nonce: &'a [u8],
    pub(crate) aad: &'a [u8],
    pub(crate) ciphertext: &'a [u8],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum FrameError {
    Short,
    OversizeAad,
    OversizePayload,
}

pub(crate) struct CommonParts<'a> {
    pub(crate) key: &'a [u8],
    pub(crate) nonce: &'a [u8],
    pub(crate) aad: &'a [u8],
    pub(crate) body: &'a [u8],
}
