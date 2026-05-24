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

use crate::arch::aarch64::context::types::SavedUser;
use crate::arch::aarch64::exceptions::frame::ExceptionFrame;

pub fn copy(saved: &mut SavedUser, frame: &ExceptionFrame) {
    saved.gprs[0] = frame.x0;
    saved.gprs[1] = frame.x1;
    saved.gprs[2] = frame.x2;
    saved.gprs[3] = frame.x3;
    saved.gprs[4] = frame.x4;
    saved.gprs[5] = frame.x5;
    saved.gprs[6] = frame.x6;
    saved.gprs[7] = frame.x7;
    saved.gprs[8] = frame.x8;
    saved.gprs[9] = frame.x9;
    saved.gprs[10] = frame.x10;
    saved.gprs[11] = frame.x11;
    saved.gprs[12] = frame.x12;
    saved.gprs[13] = frame.x13;
    saved.gprs[14] = frame.x14;
    saved.gprs[15] = frame.x15;
    saved.gprs[16] = frame.x16;
    saved.gprs[17] = frame.x17;
    saved.gprs[18] = frame.x18;
    saved.gprs[19] = frame.x19;
    saved.gprs[20] = frame.x20;
    saved.gprs[21] = frame.x21;
    saved.gprs[22] = frame.x22;
    saved.gprs[23] = frame.x23;
    saved.gprs[24] = frame.x24;
    saved.gprs[25] = frame.x25;
    saved.gprs[26] = frame.x26;
    saved.gprs[27] = frame.x27;
    saved.gprs[28] = frame.x28;
    saved.gprs[29] = frame.x29;
    saved.gprs[30] = frame.x30;
}
