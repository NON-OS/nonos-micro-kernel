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

use alloc::{string::String, vec::Vec};

use nonos_app_skeleton::clients::vfs::list_paths;
use nonos_libc::mk_getpid;

use super::load_by_name::valid_name;
use crate::protocol::{encode_response, Request, EINVAL};

const STORE_PREFIX: &str = "/capsules/";
const MAX_NAMES: usize = 64;

// The four blobs `load_by_name` feeds to the load syscall; an entry missing any
// of them can never spawn, so reporting it would yield a broken launcher entry.
const ARTIFACT_EXTS: [&str; 4] = [".elf", ".nonos_id_cert.bin", ".manifest.bin", ".zk_trailer.bin"];

// Request payload: empty.
// Reply body: count(4, LE) | count * (name_len(1) | name). Names are sorted so
// that a store holding more than MAX_NAMES truncates deterministically.
pub fn list_installed(req: Request<'_>) -> Vec<u8> {
    let Ok(entries) = list_paths(mk_getpid(), STORE_PREFIX.as_bytes()) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let mut names = installed_names(&entries);
    names.sort_unstable();
    names.truncate(MAX_NAMES);
    encode_response(req.seq, 0, &encode_names(&names))
}

fn installed_names(entries: &[String]) -> Vec<&str> {
    let mut names = Vec::new();
    for entry in entries {
        let Some(base) = store_stem(entry, ".elf") else {
            continue;
        };
        if valid_name(base.as_bytes()) && has_every_artifact(entries, base) {
            names.push(base);
        }
    }
    names
}

fn has_every_artifact(entries: &[String], base: &str) -> bool {
    ARTIFACT_EXTS.iter().all(|ext| entries.iter().any(|e| store_stem(e, ext) == Some(base)))
}

fn store_stem<'a>(entry: &'a str, ext: &str) -> Option<&'a str> {
    entry.strip_prefix(STORE_PREFIX)?.strip_suffix(ext)
}

fn encode_names(names: &[&str]) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + names.len() * 8);
    body.extend_from_slice(&(names.len() as u32).to_le_bytes());
    for name in names {
        body.push(name.len() as u8);
        body.extend_from_slice(name.as_bytes());
    }
    body
}
