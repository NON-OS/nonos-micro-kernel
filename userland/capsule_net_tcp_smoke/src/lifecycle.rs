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

use crate::client;
use ops::{connect, mark};

const SRV: [u8; 4] = [10, 0, 2, 200];

pub fn run() {
    let port = match client::lookup() {
        Some(p) => p,
        None => return,
    };
    let handle = match connect(port, SRV, 7) {
        Some(h) => h,
        None => return,
    };
    mark(b"[TCP] CONNECT OK\n");
    activeflow::echo(port, handle);
    if activeflow::close_active(port, handle) {
        activeflow::timewait(port, handle);
        activeflow::closed(port, handle);
    }
    closeflow::passive_close(port, SRV);
    closeflow::rst_refused(port, SRV);
}
