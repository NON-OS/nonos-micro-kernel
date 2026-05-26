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

use crate::driver::Driver;
use crate::debug;
use crate::protocol::{
    Request, E_BAD_OP, E_INVAL, OP_ATTACH_BACKING, OP_CONTROLLER_INFO, OP_CONTROLQ_STATE,
    OP_CREATE_RESOURCE, OP_DISPLAY_INFO, OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_HEALTHCHECK,
    OP_MODE_LIST, OP_QUERY_CAPS, OP_SET_SCANOUT, OP_TRANSFER_TO_HOST,
};
use crate::server::{handlers, respond};

pub fn dispatch(driver: &Driver, sender_pid: u32, req: Request, body: &[u8], tx: &mut [u8]) {
    if sender_pid == 0x17 {
        match req.op {
            OP_HEALTHCHECK => debug::marker(b"health req"),
            OP_GET_PRIMARY_SURFACE => debug::marker(b"primary req"),
            _ => {}
        }
    }
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_CONTROLLER_INFO if body.is_empty() => handlers::controller::handle(driver, sender_pid, &req, tx),
        OP_DISPLAY_INFO if body.is_empty() => handlers::display::handle(driver, sender_pid, &req, tx),
        OP_CONTROLQ_STATE if body.is_empty() => handlers::controlq::handle(driver, sender_pid, &req, tx),
        OP_QUERY_CAPS if body.is_empty() => handlers::query_caps::handle(driver, sender_pid, &req, tx),
        OP_MODE_LIST if body.is_empty() => handlers::mode_list::handle(driver, sender_pid, &req, tx),
        OP_GET_PRIMARY_SURFACE if body.is_empty() => handlers::get_primary_surface::handle(driver, sender_pid, &req, tx),
        OP_CREATE_RESOURCE => handlers::create_resource::handle(driver, sender_pid, &req, body, tx),
        OP_ATTACH_BACKING => handlers::attach_backing::handle(driver, sender_pid, &req, body, tx),
        OP_TRANSFER_TO_HOST => handlers::transfer_to_host::handle(driver, sender_pid, &req, body, tx),
        OP_SET_SCANOUT => handlers::set_scanout::handle(driver, sender_pid, &req, body, tx),
        OP_FLUSH => handlers::flush::handle(driver, sender_pid, &req, body, tx),
        _ if body.is_empty() => {
            respond::status(sender_pid, &req, E_BAD_OP, tx);
        }
        _ => {
            respond::status(sender_pid, &req, E_INVAL, tx);
        }
    }
}
