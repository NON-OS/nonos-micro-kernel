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

use crate::arch::aarch64::context::save_user_frame;
use crate::arch::aarch64::exceptions::frame::ExceptionFrame;
use crate::arch::aarch64::gic::{acknowledge_interrupt, dispatch_irq, end_interrupt};

use super::fatal::fatal;

#[no_mangle]
pub extern "C" fn aarch64_exc_irq_current(frame: *mut ExceptionFrame) {
    let frame = unsafe { &*frame };
    handle(frame, b"IRQ EL1")
}

#[no_mangle]
pub extern "C" fn aarch64_exc_irq_lower(frame: *mut ExceptionFrame) {
    let frame = unsafe { &*frame };

    save_user_frame(frame);
    handle(frame, b"IRQ EL0")
}

fn handle(frame: &ExceptionFrame, tag: &[u8]) {
    let intid = match acknowledge_interrupt() {
        Some(i) => i,
        None => return,
    };
    if dispatch_irq(intid) {
        end_interrupt(intid);
        return;
    }
    end_interrupt(intid);
    fatal(tag, frame)
}
