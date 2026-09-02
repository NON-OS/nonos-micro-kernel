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

mod build_request;
mod call;
mod call_boot;
mod call_with_timeout;
mod payload_slice;
mod read_status;

pub const NVGP_MAGIC: u32 = 0x4E56_4750;
pub const NVGP_VERSION: u16 = 1;
pub const NVGP_HDR_LEN: usize = 20;
// A present round-trip that misses is a dropped frame, not a reason to freeze
// the whole desktop for a second. Bound it near a couple of frames: a slow or
// lost driver reply costs one late frame instead of a visible full-UI stall.
const CALL_REPLY_TIMEOUT_MS: u64 = 100;
const BOOT_REPLY_TIMEOUT_MS: u64 = 250;

pub use build_request::build_request;
pub use call::call;
pub use call_boot::call_boot;
pub(super) use call_with_timeout::call_with_timeout;
pub use payload_slice::payload_slice;
pub use read_status::read_status;
