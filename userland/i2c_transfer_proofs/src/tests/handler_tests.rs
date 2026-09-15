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

//! The OP_TRANSFER handler: a request body in, a reply the HID driver's wire
//! code will accept out, both laid out as i2c_hid's i2c_client/wire.rs does.

use nonos_i2cmodel::{descriptor, Config};
use nonos_libc::take_reply;

use super::fixture::{bench, PAD};
use crate::protocol::{parse, Request, E_INVAL, E_NACK, E_OK, IPC_PAYLOAD_MAX, OP_TRANSFER};
use crate::server::handlers::transfer::handle;

const CALLER: u32 = 42;

/// addr, reserved, write length, read length, flags, then the write bytes.
fn body(addr: u8, write: &[u8], read_len: u16, flags: u16) -> Vec<u8> {
    let mut b = vec![addr, 0];
    b.extend_from_slice(&(write.len() as u16).to_le_bytes());
    b.extend_from_slice(&read_len.to_le_bytes());
    b.extend_from_slice(&flags.to_le_bytes());
    b.extend_from_slice(write);
    b
}

/// Run the handler and give back (errno, reply body) as the wire carries them.
fn call(b: &super::fixture::Bench, body: &[u8]) -> (i32, Vec<u8>) {
    let req = Request { op: OP_TRANSFER, request_id: 7 };
    let mut out = [0u8; IPC_PAYLOAD_MAX];
    handle(&b.driver, CALLER, &req, body, &mut out);
    let (to, bytes) = take_reply().expect("the handler replied");
    assert_eq!(to, CALLER, "reply addressed to the wrong process");
    let (echo, payload) = parse(&bytes).expect("reply header parses");
    assert_eq!((echo.op, echo.request_id), (OP_TRANSFER, 7), "reply does not echo the request");
    let errno = i32::from_le_bytes(payload[0..4].try_into().expect("errno"));
    (errno, payload[4..].to_vec())
}

#[test]
fn a_register_read_over_ipc_carries_the_length_the_abort_source_and_the_bytes() {
    let b = bench(Config::LPSS);
    let (errno, reply) = call(&b, &body(PAD, &[0x01, 0x00], descriptor::LEN as u16, 1));
    assert_eq!(errno, E_OK);
    assert_eq!(u16::from_le_bytes([reply[0], reply[1]]) as usize, descriptor::LEN);
    assert_eq!(u32::from_le_bytes(reply[4..8].try_into().expect("abort")), 0);
    assert_eq!(&reply[8..], b.pad.lock().descriptor());
}

#[test]
fn a_body_whose_write_length_disagrees_with_its_size_is_invalid_before_the_bus() {
    let b = bench(Config::LPSS);
    let mut short = body(PAD, &[0x01, 0x00], 2, 1);
    short.pop();
    assert_eq!(call(&b, &short).0, E_INVAL);
    assert!(super::fixture::core(|c| c.bus().trace().is_empty()), "the bus was touched");
}

#[test]
fn a_nack_reaches_the_caller_as_its_own_errno() {
    let b = bench(Config::LPSS);
    assert_eq!(call(&b, &body(0x2C, &[0x01, 0x00], 2, 1)).0, E_NACK);
}
