// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::protocol::{response, Request};
use nonos_libc::mk_ipc_reply;

pub fn send(sender_pid: u32, req: &Request, errno: i32, body: &[u8], out: &mut [u8]) -> i64 {
    let n = response(req.op, req.request_id, errno, body, out);
    mk_ipc_reply(sender_pid, out.as_ptr(), n)
}
