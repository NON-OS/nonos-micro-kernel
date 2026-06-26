// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

// EFI_TCG2_EVENT_HEADER is packed in the TCG EFI Protocol spec: 4 + 2 + 4 + 4
// = 14 bytes with no padding. A natural repr(C) would pad to 16 and the
// firmware would reject the event.
#[repr(C, packed)]
pub struct Tcg2EventHeader {
    pub header_size: u32,
    pub header_version: u16,
    pub pcr_index: u32,
    pub event_type: u32,
}

// EFI_TCG2_EVENT: a leading Size covering the whole structure, the header, and
// a variable event body. HashLogExtendEvent reads Size first and validates it.
#[repr(C, packed)]
pub struct Tcg2Event {
    pub size: u32,
    pub header: Tcg2EventHeader,
    pub event: [u8; 32],
}
