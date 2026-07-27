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

pub const STATUS_LEN: usize = 4;
pub const MAX_PCM_CHUNK: usize = 4096;
pub const CONTROLLER_INFO_PAYLOAD_LEN: usize = 28;
pub const CODEC_MASK_PAYLOAD_LEN: usize = 8;
pub const STREAM_LAYOUT_HEADER_BYTES: usize = 4;
pub const STREAM_ENTRY_BYTES: usize = 8;
pub const MAX_STREAM_LAYOUT_BYTES: usize = STREAM_LAYOUT_HEADER_BYTES + 64 * STREAM_ENTRY_BYTES;
pub const CODEC_LIST_HEADER_BYTES: usize = 4;
pub const CODEC_ENTRY_BYTES: usize = 8;
pub const MAX_CODEC_LIST_BYTES: usize = CODEC_LIST_HEADER_BYTES + 15 * CODEC_ENTRY_BYTES;
