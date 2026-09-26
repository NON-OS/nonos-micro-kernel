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

//! `install <name>`: fetch a package and everything under it.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_debug;

use super::download::download;
use super::index_load::load_index;
use super::place::unpack;
use super::provenance::Provenance;

pub(super) const HOST: &str = "dl-cdn.alpinelinux.org";
pub(super) const PORT: u16 = 80;
pub(super) const RELEASE: &str = "v3.20";
pub(super) const ARCH: &str = "x86_64";
pub(super) const BRANCHES: [&str; 2] = ["main", "community"];

/// Rounds of resolution. A closure that has not settled by now is a
/// dependency cycle the index cannot satisfy, and looping would hide it.
const ROUNDS: usize = 12;

pub fn install(name: &str) -> bool {
    let Some(index) = load_index() else {
        say(b"[LINUX] no package index\n");
        return false;
    };
    let mut wanted: Vec<String> = vec![String::from(name)];
    let mut done: Vec<String> = Vec::new();
    for _ in 0..ROUNDS {
        let Some(next) = wanted.pop() else {
            return true;
        };
        if done.contains(&next) {
            continue;
        }
        let Some(pkg) = index.by_name(&next).or_else(|| index.by_lib(&next)) else {
            say(b"[LINUX] nothing provides it\n");
            return false;
        };
        let apk = download(&pkg.name, &pkg.version);
        if apk.is_empty() {
            say(b"[LINUX] package would not download\n");
            return false;
        }
        /*
         * Nothing says these are the bytes the distribution
         * published: see `provenance`.
         */
        unpack(&apk, Provenance::Unauthenticated);
        done.push(next);
    }
    true
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
