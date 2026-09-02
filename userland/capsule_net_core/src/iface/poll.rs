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

use nonos_libc::mk_time_millis;
use smoltcp::time::Instant;

use crate::device::budget;
use crate::iface::dhcp;
use crate::state;

pub fn pump() {
    budget::open_poll();
    state::with_iface(|iface, sockets, device| {
        let now = Instant::from_millis(mk_time_millis());
        iface.poll(now, device, sockets);
    });
    budget::close_poll();
    dhcp::poll_event();
}
