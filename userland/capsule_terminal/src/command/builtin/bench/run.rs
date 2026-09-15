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

//! Taking the measurements and reporting them.

use nonos_bench_core::{measure, Overhead, Summary};

use super::format::{heading, row};
use super::ipc::Peer;
use super::probe_list::{IPC_NOTE, PROBES};
use super::probes::{fire, CALIBRATION, IPC_SAMPLES, SAMPLES};
use super::run_lines::{blank_line, note};
use crate::command::output::Output;

pub fn run(out: &mut Output<'_>, _argv: &[&[u8]]) {
    let overhead = Overhead::measure(CALIBRATION);
    out.writeln(b"cycles per operation, counter overhead already subtracted.");
    out.writeln(b"percentiles are nearest rank: every figure below was measured.");
    blank_line(out, overhead.cycles);
    heading(out);

    /*
     * One buffer, reused. A capsule has no business allocating sixteen kilobytes
     * per probe when the samples are consumed before the next probe starts.
     */
    let mut buf = [0u64; SAMPLES];
    for (i, p) in PROBES.iter().enumerate() {
        let s: Summary = measure(&mut buf, overhead, || fire(i));
        row(out, p.name, &s);
    }

    /*
     * The round trip, which needs a peer that answers rather than only the
     * kernel. Fewer samples: each one is a scheduler round trip through
     * another process, so 2048 of them is a visible pause.
     */
    match Peer::find() {
        Some(peer) => {
            let mut rx = [0u8; 64];
            let mut ipc_buf = [0u64; IPC_SAMPLES];
            let s = measure(&mut ipc_buf, overhead, || peer.ping(&mut rx));
            row(out, b"ipc", &s);
        }
        None => out.writeln(b"ipc        peer not registered, nothing measured"),
    }
    for p in PROBES.iter() {
        note(out, p);
    }
    note(out, &IPC_NOTE);
}
