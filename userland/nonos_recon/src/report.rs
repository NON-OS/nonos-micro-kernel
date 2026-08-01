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

//! The scan outcome and its text form.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// What a single probe found.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PortState {
    /// The connect completed: something is listening.
    Open,
    /// The connect was refused: the host answered but nothing listens.
    Closed,
    /// No answer within the timeout, or the transport failed. Whether a
    /// firewall dropped the packet or the host is down is not distinguishable
    /// from a connect scan, so both read as filtered.
    Filtered,
}

impl PortState {
    fn label(self) -> &'static str {
        match self {
            PortState::Open => "open",
            PortState::Closed => "closed",
            PortState::Filtered => "filtered",
        }
    }
}

/// One port's result.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ScanRow {
    pub port: u16,
    pub state: PortState,
}

/// Render the report the way a scanner prints it: the target, then one line per
/// open or closed port, then a count of the filtered ports rather than a line
/// each, since a wide scan of a filtered host is otherwise all noise.
pub fn format_report(octets: &[u8; 4], rows: &[ScanRow]) -> String {
    let mut out = String::new();
    out.push_str("recon ");
    append_ipv4(&mut out, octets);
    out.push('\n');

    let mut open = 0usize;
    let mut closed = 0usize;
    let mut filtered = 0usize;
    for row in rows {
        match row.state {
            PortState::Open => open += 1,
            PortState::Closed => closed += 1,
            PortState::Filtered => {
                filtered += 1;
                continue;
            }
        }
        out.push_str("  ");
        append_u16(&mut out, row.port);
        out.push('\t');
        out.push_str(row.state.label());
        out.push('\n');
    }

    out.push_str("scanned ");
    append_usize(&mut out, rows.len());
    out.push_str(": ");
    append_usize(&mut out, open);
    out.push_str(" open, ");
    append_usize(&mut out, closed);
    out.push_str(" closed, ");
    append_usize(&mut out, filtered);
    out.push_str(" filtered\n");
    out
}

fn append_ipv4(out: &mut String, octets: &[u8; 4]) {
    for (i, o) in octets.iter().enumerate() {
        if i > 0 {
            out.push('.');
        }
        append_u16(out, *o as u16);
    }
}

fn append_u16(out: &mut String, v: u16) {
    append_usize(out, v as usize);
}

fn append_usize(out: &mut String, mut v: usize) {
    if v == 0 {
        out.push('0');
        return;
    }
    let mut digits: Vec<u8> = Vec::new();
    while v > 0 {
        digits.push(b'0' + (v % 10) as u8);
        v /= 10;
    }
    while let Some(d) = digits.pop() {
        out.push(d as char);
    }
}
