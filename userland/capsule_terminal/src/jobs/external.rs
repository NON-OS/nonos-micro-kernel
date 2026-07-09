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

use nonos_libc::{mk_pid_alive, mk_proc_input, mk_proc_output, mk_wait};

use crate::command::output::Output;
use crate::jobs::JobProgress;

pub fn step_external(
    pid: u32,
    in_buf: &[u8],
    in_cursor: &mut usize,
    out: &mut Output<'_>,
) -> JobProgress {
    feed_stdin(pid, in_buf, in_cursor);
    let mut buf = [0u8; 256];
    let n = mk_proc_output(pid, buf.as_mut_ptr(), buf.len());
    if n > 0 {
        out.feed_raw(&buf[..(n as usize).min(buf.len())]);
    }
    if !mk_pid_alive(pid) {
        drain_remaining(pid, out, &mut buf);
        return JobProgress::Done(mk_wait(pid as u64, 0) as i32);
    }
    JobProgress::Running
}

fn feed_stdin(pid: u32, in_buf: &[u8], in_cursor: &mut usize) {
    if *in_cursor >= in_buf.len() {
        return;
    }
    let pending = &in_buf[*in_cursor..];
    let sent = mk_proc_input(pid as u64, pending.as_ptr(), pending.len() as u64);
    if sent > 0 {
        *in_cursor += (sent as usize).min(pending.len());
    }
}

fn drain_remaining(pid: u32, out: &mut Output<'_>, buf: &mut [u8; 256]) {
    loop {
        let m = mk_proc_output(pid, buf.as_mut_ptr(), buf.len());
        if m <= 0 {
            break;
        }
        out.feed_raw(&buf[..(m as usize).min(buf.len())]);
    }
}
