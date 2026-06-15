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

pub fn self_check() -> bool {
    let host = b"ethereum-rpc.publicnode.com";
    let random = [0x33u8; 32];
    let session = [0x44u8; 32];
    let public = [0x55u8; 32];
    let server_public = [0x66u8; 32];
    let shared = [0x88u8; 32];
    let keys = match super::schedule::handshake_keys(&shared, b"nonos tls transcript") {
        Some(keys) => keys,
        None => return false,
    };
    let sealed = match super::record_seal::seal(&keys.client_key, &keys.client_iv, 0, 22, b"abc") {
        Some(record) => record,
        None => return false,
    };
    let opened = match super::record_open::open(&keys.client_key, &keys.client_iv, 0, &sealed) {
        Some(plain) => plain,
        None => return false,
    };
    let hello = super::client_hello::client_hello(host, &random, &session, &public);
    let record = super::record::handshake_record(&hello);
    record.starts_with(&[22, 3, 1])
        && hello.first() == Some(&1)
        && hello.windows(host.len()).any(|w| w == host)
        && hello.windows(2).any(|w| w == [0x13, 0x03])
        && hello.windows(2).any(|w| w == [0x00, 0x1d])
        && hello.windows(public.len()).any(|w| w == public)
        && super::server_hello::key_share(&server_hello(&server_public)) == Some(server_public)
        && keys.client_key != keys.server_key
        && keys.client_iv != keys.server_iv
        && opened == b"abc\x16"
        && super::finished_self_check::finished_self_check()
}

fn server_hello(public: &[u8; 32]) -> Vec<u8> {
    let mut body = Vec::with_capacity(96);
    super::push::u16(&mut body, 0x0303);
    body.extend_from_slice(&[0x77u8; 32]);
    body.push(0);
    super::push::u16(&mut body, 0x1303);
    body.push(0);
    let mut ext = Vec::with_capacity(48);
    super::push::ext(&mut ext, 43, &[0x03, 0x04]);
    let mut ks = Vec::with_capacity(36);
    super::push::u16(&mut ks, 0x001d);
    super::push::u16(&mut ks, 32);
    ks.extend_from_slice(public);
    super::push::ext(&mut ext, 51, &ks);
    super::push::u16(&mut body, ext.len() as u16);
    body.extend_from_slice(&ext);
    let mut out = Vec::with_capacity(body.len() + 4);
    out.push(2);
    super::push::u24(&mut out, body.len());
    out.extend_from_slice(&body);
    out
}
