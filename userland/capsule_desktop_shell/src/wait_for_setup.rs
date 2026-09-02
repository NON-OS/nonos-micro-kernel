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

use nonos_libc::{mk_debug, mk_yield};

pub fn wait_for_setup() -> crate::state::Context {
    let mut last: &'static str = "";
    let mut rounds: u32 = 0;
    loop {
        match crate::setup::run() {
            Ok(ctx) => return ctx,
            Err(step) => {
                // A silent retry here reads as an empty desktop from outside.
                // Say what failed, once per change and then every 64 rounds.
                if step != last || rounds % 64 == 0 {
                    let mut line = [0u8; 96];
                    let tag = b"[SHELL] setup stuck: ";
                    let n = tag.len().min(line.len());
                    line[..n].copy_from_slice(&tag[..n]);
                    let m = step.len().min(line.len() - n);
                    line[n..n + m].copy_from_slice(&step.as_bytes()[..m]);
                    let _ = mk_debug(line.as_ptr(), n + m);
                    last = step;
                }
                rounds = rounds.wrapping_add(1);
                for _ in 0..64 {
                    mk_yield();
                }
            }
        }
    }
}
