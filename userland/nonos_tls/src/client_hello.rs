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

use alloc::vec::Vec;

use super::constants::{
    HS_CLIENT_HELLO, LEGACY_HANDSHAKE_VERSION, SUITE_AES128_GCM_SHA256, SUITE_CHACHA20_SHA256,
};

pub fn client_hello(
    host: &[u8],
    random: &[u8; 32],
    session: &[u8; 32],
    public: &[u8; 32],
) -> Vec<u8> {
    let mut body = Vec::with_capacity(192 + host.len());
    super::push::u16(&mut body, LEGACY_HANDSHAKE_VERSION);
    body.extend_from_slice(random);
    body.push(session.len() as u8);
    body.extend_from_slice(session);
    super::push::u16(&mut body, 4);
    super::push::u16(&mut body, SUITE_CHACHA20_SHA256);
    super::push::u16(&mut body, SUITE_AES128_GCM_SHA256);
    body.push(1);
    body.push(0);
    let mut ext = Vec::with_capacity(96 + host.len());
    super::ext_sni::ext_sni(&mut ext, host);
    super::ext_versions::ext_versions(&mut ext);
    super::ext_groups::ext_groups(&mut ext);
    super::ext_sigalgs::ext_sigalgs(&mut ext);
    super::ext_keyshare::ext_keyshare(&mut ext, public);
    super::push::u16(&mut body, ext.len() as u16);
    body.extend_from_slice(&ext);
    let mut out = Vec::with_capacity(body.len() + 4);
    out.push(HS_CLIENT_HELLO);
    super::push::u24(&mut out, body.len());
    out.extend_from_slice(&body);
    out
}
