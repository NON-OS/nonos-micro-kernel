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

use super::constants::{
    EXT_KEY_SHARE, EXT_SUPPORTED_VERSIONS, GROUP_X25519, SUITE_AES128_GCM_SHA256,
    SUITE_CHACHA20_SHA256, TLS13,
};

// Parse the server's chosen cipher suite and X25519 key share. Only the two
// suites we can key and seal are accepted.
pub fn key_share(handshake: &[u8]) -> Option<(u16, [u8; 32])> {
    if handshake.first() != Some(&2) {
        return None;
    }
    let body = super::read::slice(handshake, 4, handshake.len().checked_sub(4)?)?;
    if super::read::u16_at(body, 0)? != 0x0303 {
        return None;
    }
    let sid_len = *body.get(34)? as usize;
    let pos = 35usize.checked_add(sid_len)?;
    let suite = super::read::u16_at(body, pos)?;
    if (suite != SUITE_CHACHA20_SHA256 && suite != SUITE_AES128_GCM_SHA256)
        || *body.get(pos + 2)? != 0
    {
        return None;
    }
    let ext_len = super::read::u16_at(body, pos + 3)? as usize;
    let exts = super::read::slice(body, pos + 5, ext_len)?;
    let share = parse_exts(exts)?;
    Some((suite, share))
}

fn parse_exts(mut exts: &[u8]) -> Option<[u8; 32]> {
    let mut version_ok = false;
    let mut share = None;
    while exts.len() >= 4 {
        let kind = super::read::u16_at(exts, 0)?;
        let len = super::read::u16_at(exts, 2)? as usize;
        let body = super::read::slice(exts, 4, len)?;
        if kind == EXT_SUPPORTED_VERSIONS {
            version_ok = body == TLS13.to_be_bytes();
        } else if kind == EXT_KEY_SHARE {
            share = parse_keyshare(body);
        }
        exts = super::read::slice(exts, 4 + len, exts.len().saturating_sub(4 + len))?;
    }
    if version_ok {
        share
    } else {
        None
    }
}

fn parse_keyshare(body: &[u8]) -> Option<[u8; 32]> {
    if super::read::u16_at(body, 0)? != GROUP_X25519 || super::read::u16_at(body, 2)? != 32 {
        return None;
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(super::read::slice(body, 4, 32)?);
    Some(out)
}
