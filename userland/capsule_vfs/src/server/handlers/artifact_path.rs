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

//! Shape check for the only paths the installer ops may touch. It replaces the
//! read-only gate those ops bypass, so it is deliberately a whitelist: the
//! literal `/capsules/` prefix, a bare install slug whose charset matches the
//! installer's own `valid_name`, and one of the four artifact extensions.
//! Because the slug admits neither `.` nor `/`, no traversal, nested directory
//! or alternate extension can survive it.

use alloc::string::String;
use core::str;

use super::path::normalize;
use crate::protocol::{EINVAL, MAX_PATH_BYTES};

const PREFIX: &str = "/capsules/";
const MAX_NAME: usize = 64;
const EXTS: [&str; 4] = [".elf", ".manifest.bin", ".nonos_id_cert.bin", ".zk_trailer.bin"];

pub(super) fn is_capsule_artifact(path: &str) -> bool {
    let Some(rest) = path.strip_prefix(PREFIX) else {
        return false;
    };
    let Some(ext) = EXTS.iter().find(|e| rest.len() > e.len() && rest.ends_with(**e)) else {
        return false;
    };
    let name = &rest[..rest.len() - ext.len()];
    name.len() <= MAX_NAME
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

// Decode `path_len(1) | path` from the head of a caller-stripped payload,
// normalize it and enforce the artifact shape. Returns the path plus whatever
// the op puts after it.
pub(super) fn split_artifact(rest: &[u8]) -> Result<(String, &[u8]), i32> {
    let len = *rest.first().ok_or(EINVAL)? as usize;
    if len == 0 || len > MAX_PATH_BYTES as usize || rest.len() < 1 + len {
        return Err(EINVAL);
    }
    let raw = str::from_utf8(&rest[1..1 + len]).map_err(|_| EINVAL)?;
    let path = normalize(raw);
    if !is_capsule_artifact(&path) {
        return Err(EINVAL);
    }
    Ok((path, &rest[1 + len..]))
}
