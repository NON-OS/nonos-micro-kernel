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

use crate::crypto::Nonce;
use crate::crypto::TAG_BYTES;
use crate::protocol::{NYM_HEADER_BYTES, NYM_PAYLOAD_BYTES};

pub const HEADER_LEN: usize = NYM_HEADER_BYTES;
pub const REPLAY_TAG_LEN: usize = 32;
pub const AEAD_PLAIN_BYTES: usize = NYM_PAYLOAD_BYTES - TAG_BYTES;
pub const FLAG_COVER: u8 = 0x01;
pub const FLAG_REPLY: u8 = 0x02;

pub struct Decoded<'a> {
    pub session_id: u32,
    pub flags: u8,
    pub nonce: Nonce,
    pub replay_tag: [u8; REPLAY_TAG_LEN],
    pub ciphertext: &'a [u8],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PacketError {
    Short,
    BadMagic,
    BadVersion,
    BadLength,
    BadTag,
    Crypto,
    NoRoute,
}
