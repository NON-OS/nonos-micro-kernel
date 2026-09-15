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

//! One step of the part: serve the next head if the driver published one.

use std::sync::atomic::{fence, AtomicU16, Ordering};
use std::sync::Mutex;

use nonos_devmodel::FakeBar;

use super::ring::{bytes, desc, offset_of, put, u16_at};
use super::{answers, Answer, Seen};
use crate::constants::{VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_USED_OFFSET};
use crate::tests::model::QUEUE_SIZE;

pub fn step(bar: &FakeBar, served: &AtomicU16, log: &Mutex<Vec<Seen>>, answer: Answer) {
    let published = u16_at(bar, VQ_AVAIL_OFFSET + 2);
    let done = served.load(Ordering::Acquire);
    if published == done {
        return;
    }
    fence(Ordering::Acquire);
    let slot = (done % QUEUE_SIZE) as usize;
    let head = u16_at(bar, VQ_AVAIL_OFFSET + 4 + slot * 2);
    let first = desc(bar, head, VQ_DESC_OFFSET);
    let second = desc(bar, first.next, VQ_DESC_OFFSET);
    let request = bytes(bar, offset_of(bar, first.addr), first.len as usize);
    let response = answers::build(&request, second.len as usize, answer);
    put(bar, offset_of(bar, second.addr), &response);
    log.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).push(Seen {
        request,
        request_flags: first.flags,
        response_flags: second.flags,
    });
    let id = match answer {
        Answer::WrongDescriptor => head.wrapping_add(1) % QUEUE_SIZE,
        _ => head,
    };
    let entry = VQ_USED_OFFSET + 4 + slot * 8;
    put(bar, entry, &(id as u32).to_le_bytes());
    put(bar, entry + 4, &(response.len() as u32).to_le_bytes());
    fence(Ordering::Release);
    put(bar, VQ_USED_OFFSET + 2, &done.wrapping_add(1).to_le_bytes());
    served.store(done.wrapping_add(1), Ordering::Release);
}
