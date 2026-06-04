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
pub struct RoundTrip<'a> {
    pub target_port: u64,
    pub reply_port: u32,
    pub magic: u32,
    pub op: u16,
    pub flags: u16,
    pub payload: &'a [u8],
    pub timeout_ms: u64,
}

pub struct Response<'a> {
    pub op: u16,
    pub errno: u16,
    pub request_id: u32,
    pub payload: &'a [u8],
}
