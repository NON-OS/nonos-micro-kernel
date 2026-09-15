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

//! HKDF-SHA256, the two steps.

use crate::hmac::hmac_sha256;
use crate::subkey::{KeyError, INFO_PREFIX, MAX_RECORD, ROOT_LABEL};
use crate::wipe::wipe;

pub fn subkey(root: &[u8; 32], record: &[u8]) -> Result<[u8; 32], KeyError> {
    if record.is_empty() || record.len() > MAX_RECORD {
        return Err(KeyError::BadRecord);
    }

    /*
     * Extract. The salt is the label rather than zeros: two roots derived
     * under different labels then cannot expand to the same subkey even if a
     * future caller reuses a record name.
     */
    let prk = hmac_sha256(ROOT_LABEL, root).ok_or(KeyError::BadRecord)?;

    /*
     * Expand, one round. The info is the prefix, the record name, then the
     * counter byte RFC 5869 requires.
     */
    let mut info = [0u8; INFO_PREFIX.len() + MAX_RECORD + 1];
    let n = INFO_PREFIX.len();
    info[..n].copy_from_slice(INFO_PREFIX);
    info[n..n + record.len()].copy_from_slice(record);
    info[n + record.len()] = 0x01;
    let out = hmac_sha256(&prk, &info[..n + record.len() + 1]).ok_or(KeyError::BadRecord)?;

    let mut prk = prk;
    wipe(&mut prk);
    wipe(&mut info);
    Ok(out)
}
