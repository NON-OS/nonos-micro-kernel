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

//! The HID driver's call to the controller service, answered by whatever
//! the test registered under the service's name.

use std::cell::RefCell;

use crate::raw;

pub const SERVICE_NAME: &[u8] = b"driver.i2c_pci0";
pub const SERVICE_PORT: u32 = 7;
pub const SERVICE_PID: u32 = 9;

type Handler = Box<dyn FnMut(&[u8], &mut [u8]) -> i64>;

thread_local! {
    static SERVICE: RefCell<Option<Handler>> = const { RefCell::new(None) };
}

/// Unregisters the service when dropped, so a later test on the same thread
/// finds no controller until it registers its own.
pub struct Served;

impl Drop for Served {
    fn drop(&mut self) {
        SERVICE.with(|s| *s.borrow_mut() = None);
    }
}

/// `handler` gets the request bytes and the caller's reply buffer, and
/// returns the reply length or a negative errno, as the kernel would.
pub fn serve(handler: impl FnMut(&[u8], &mut [u8]) -> i64 + 'static) -> Served {
    SERVICE.with(|s| *s.borrow_mut() = Some(Box::new(handler)));
    Served
}

pub fn mk_service_lookup(name: *const u8, name_len: usize, port: *mut u32, pid: *mut u32) -> i64 {
    let registered = SERVICE.with(|s| s.borrow().is_some());
    if raw::bytes(name, name_len) != SERVICE_NAME || !registered {
        return -1;
    }
    raw::store(port, SERVICE_PORT);
    raw::store(pid, SERVICE_PID);
    0
}

pub fn mk_ipc_call_timeout(
    port: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    resp_len: usize,
    _timeout_ms: u64,
) -> i64 {
    if port != SERVICE_PORT as u64 {
        return -1;
    }
    let (request, reply) = (raw::bytes(req, req_len), raw::bytes_mut(resp, resp_len));
    SERVICE.with(|s| s.borrow_mut().as_mut().map_or(-1, |handler| handler(request, reply)))
}
