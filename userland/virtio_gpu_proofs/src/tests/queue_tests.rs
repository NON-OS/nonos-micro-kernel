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

//! The control queue against a part that consumes it.

use crate::constants::{
    LEG_QUEUE_NOTIFY, VG_RESP_OK_NODATA, VRING_DESC_F_NEXT, VRING_DESC_F_WRITE,
};
use crate::device::cmd::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use crate::tests::harness::harness;
use crate::tests::part::{answering, Answer};
use crate::tests::wire::request;

#[test]
fn a_command_goes_out_as_a_read_then_write_chain_and_the_answer_is_read_back() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    h.regs.present16(LEG_QUEUE_NOTIFY, 0xFFFF);
    let out = h.queue.submit(&request(0x0104), RESP_HDR_LEN as u32).expect("answered");
    assert_eq!(out.used_len as usize, RESP_HDR_LEN);
    let seen = part.seen.lock().expect("log");
    assert_eq!(seen[0].request, request(0x0104));
    assert_eq!(seen[0].request_flags, VRING_DESC_F_NEXT, "the part reads the request");
    assert_eq!(seen[0].response_flags, VRING_DESC_F_WRITE, "and writes the answer");
    assert_eq!(h.regs.wrote16(LEG_QUEUE_NOTIFY), 0, "the doorbell rang for queue zero");
    let mut resp = [0u8; RESP_HDR_LEN];
    h.queue.read_response(HDR_LEN, &mut resp);
    assert_eq!(Hdr::parse(&resp).map(|r| r.type_), Some(VG_RESP_OK_NODATA));
}

#[test]
fn a_part_that_completes_a_descriptor_the_driver_did_not_publish_is_refused() {
    let h = harness();
    let _part = answering(&h.region, Answer::WrongDescriptor);
    let err = h.queue.submit(&request(0x0104), RESP_HDR_LEN as u32).err();
    assert_eq!(err, Some("virtio-gpu: used id mismatch"));
}

#[test]
fn a_part_that_never_answers_is_given_up_on() {
    let h = harness();
    let err = h.queue.submit(&request(0x0104), RESP_HDR_LEN as u32).err();
    assert_eq!(err, Some("virtio-gpu: device timeout"));
}
