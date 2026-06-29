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

#[allow(clippy::too_many_arguments)]
pub fn scan(
    secret: &[u8; 32],
    transcript: &mut Vec<u8>,
    msgs: &[u8],
    host: &[u8],
    now: u64,
    cert11: &mut Vec<u8>,
    validated: &mut bool,
) -> bool {
    let mut pos = 0usize;
    while pos + 4 <= msgs.len() {
        let len = ((msgs[pos + 1] as usize) << 16) | ((msgs[pos + 2] as usize) << 8) | msgs[pos + 3] as usize;
        let end = pos + 4 + len;
        if end > msgs.len() {
            return false;
        }
        match msgs[pos] {
            11 => {
                cert11.clear();
                cert11.extend_from_slice(&msgs[pos + 4..end]);
                if !super::chain_walk::verify_chain(cert11.as_slice(), host, now) {
                    return false;
                }
                transcript.extend_from_slice(&msgs[pos..end]);
            }
            15 => {
                let before_cv = transcript.clone();
                let Some(leaf) = super::cert_at::cert_at(cert11.as_slice(), 0) else {
                    return false;
                };
                if !super::cert_verify_msg::verify_cert_verify(leaf, &before_cv, &msgs[pos + 4..end]) {
                    return false;
                }
                *validated = true;
                transcript.extend_from_slice(&msgs[pos..end]);
            }
            20 => {
                if !*validated {
                    return false;
                }
                let ok = super::finished_verify::verify(secret, transcript, &msgs[pos + 4..end]);
                if ok {
                    transcript.extend_from_slice(&msgs[pos..end]);
                }
                return ok;
            }
            _ => {
                transcript.extend_from_slice(&msgs[pos..end]);
            }
        }
        pos = end;
    }
    false
}
