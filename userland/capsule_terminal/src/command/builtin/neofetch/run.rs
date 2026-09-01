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

//! `neofetch` — the fresh-tab splash rendered as scrollback text.

use alloc::vec::Vec;
use nonos_libc::mk_time_millis;

use crate::paint::fetch_banner::BANNER;
use crate::command::output::Output;
use crate::term::state::State;
use crate::term::util::{copy_into, format_u64};

const VERSION: &str = include_str!("../../../../../../VERSION");

fn row(out: &mut Output<'_>, label: &str, value: &str) {
    let mut line = Vec::with_capacity(48);
    line.extend_from_slice(label.as_bytes());
    while line.len() < 8 {
        line.push(b' ');
    }
    line.extend_from_slice(value.as_bytes());
    out.writeln(&line);
}

fn uptime(state: &State, buf: &mut [u8]) -> usize {
    let now = mk_time_millis();
    let elapsed =
        if now > 0 && now as u64 >= state.start_ms { now as u64 - state.start_ms } else { 0 };
    let total = elapsed / 1000;
    let mut k = 0;
    k += format_u64(total / 60, &mut buf[k..]);
    k += copy_into(&mut buf[k..], b"m ");
    k += format_u64(total % 60, &mut buf[k..]);
    k += copy_into(&mut buf[k..], b"s");
    k
}

pub fn run(state: &mut State) {
    let mut ubuf = [0u8; 24];
    let n = uptime(state, &mut ubuf);
    let up = core::str::from_utf8(&ubuf[..n]).unwrap_or("");
    let mut kernel = Vec::with_capacity(32);
    kernel.extend_from_slice(b"microkernel ");
    kernel.extend_from_slice(VERSION.trim_end().as_bytes());
    let kernel = core::str::from_utf8(&kernel).unwrap_or("microkernel");

    let out = &mut Output::new(&mut state.scrollback);
    for line in BANNER {
        out.writeln(line.as_bytes());
    }
    out.writeln(b"ZeroState Cryptographic OS");
    out.writeln(b"");
    out.writeln(b"nonos@capsule");
    out.writeln(b"-------------");
    row(out, "os", "NONOS RAM-resident");
    row(out, "kernel", kernel);
    row(out, "shell", "nox   (type 'help')");
    row(out, "trust", "Ed25519 + ML-DSA-65");
    row(out, "arch", "x86_64");
    row(out, "uptime", up);
}
