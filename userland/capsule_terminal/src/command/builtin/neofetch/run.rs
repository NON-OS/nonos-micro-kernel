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

use crate::command::output::Output;
use crate::term::identity::{hostname, USER};
use crate::term::state::State;
use crate::term::util::{copy_into, format_u64};

use super::compose::two_column;
use super::logo::LOGO;
use super::palette::palette;

const VERSION: &str = include_str!("../../../../../../VERSION");

const GAP: usize = 2;

fn row(label: &str, value: &[u8]) -> Vec<u8> {
    let mut line = Vec::with_capacity(48);
    line.extend_from_slice(label.as_bytes());
    while line.len() < 8 {
        line.push(b' ');
    }
    line.extend_from_slice(value);
    line
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

fn info(kernel: &[u8], up: &[u8]) -> Vec<Vec<u8>> {
    let mut head = Vec::with_capacity(32);
    head.extend_from_slice(USER);
    head.push(b'@');
    head.extend_from_slice(hostname());
    let mut rule = Vec::with_capacity(head.len());
    rule.resize(head.len(), b'-');
    alloc::vec![
        head,
        rule,
        Vec::from(&b"ZeroState Cryptographic OS"[..]),
        Vec::new(),
        row("os", b"NONOS RAM-resident"),
        row("kernel", kernel),
        row("shell", b"nox   (type 'help')"),
        row("trust", b"Ed25519 + ML-DSA-65"),
        row("arch", b"x86_64"),
        row("uptime", up),
    ]
}

pub fn run(state: &mut State) {
    let mut ubuf = [0u8; 24];
    let n = uptime(state, &mut ubuf);
    let mut kernel = Vec::with_capacity(32);
    kernel.extend_from_slice(b"microkernel ");
    kernel.extend_from_slice(VERSION.trim_end().as_bytes());

    let rows = two_column(&LOGO, &info(&kernel, &ubuf[..n]), GAP);
    let (plain, styled) = palette();

    let out = &mut Output::new(&mut state.scrollback);
    for line in &rows {
        out.writeln(line);
    }
    out.writeln(b"");
    out.writeln_styled(&plain, &styled);
}
