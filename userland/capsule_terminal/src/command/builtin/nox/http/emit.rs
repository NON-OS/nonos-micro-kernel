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

use nonos_http::Response;

use crate::term::state::State;

/// Print the body a line at a time. A non-200 status is printed first, since
/// an error page is still a body and would otherwise read as the answer.
pub fn emit(state: &mut State, response: &Response) {
    if response.status != 200 {
        let mut line = alloc::vec::Vec::new();
        line.extend_from_slice(b"http: status ");
        push_num(&mut line, response.status as u64);
        state.scrollback.push_error(&line);
    }
    for chunk in response.body.split(|&b| b == b'\n') {
        let trimmed = match chunk.split_last() {
            Some((b'\r', head)) => head,
            _ => chunk,
        };
        if !trimmed.is_empty() {
            state.scrollback.push_line(trimmed);
        }
    }
}

fn push_num(out: &mut alloc::vec::Vec<u8>, v: u64) {
    let mut buf = [0u8; 24];
    let k = crate::term::util::format_u64(v, &mut buf);
    out.extend_from_slice(&buf[..k]);
}
