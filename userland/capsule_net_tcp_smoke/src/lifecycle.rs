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

mod activeflow;
mod closeflow;
mod ops;
mod serverflow;

use crate::client;
use crate::wait::poll_until;
use ops::{connect, mark};

const SRV: [u8; 4] = [10, 0, 2, 200];

pub fn run() {
    let mut port = 0u32;
    let found = poll_until(40_000, || match client::lookup() {
        Some(p) => {
            port = p;
            true
        }
        None => false,
    });
    if !found {
        return;
    }
    #[cfg(feature = "tcp-selftest")]
    ops::selftest(port);
    serverflow::passive_server(port);
    serverflow::active_server(port);
    let mut handle = None;
    poll_until(40_000, || {
        handle = connect(port, SRV, 7);
        handle.is_some()
    });
    let handle = match handle {
        Some(h) => h,
        None => return,
    };
    mark(b"[TCP] CONNECT OK\n");
    activeflow::echo(port, handle);
    activeflow::close_active(port, handle);
    closeflow::rst_refused(port, SRV);
}
