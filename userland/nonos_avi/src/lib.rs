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

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod bytes;
mod chunk;
mod error;
mod header;
mod index;
mod stream;

pub use bytes::{fourcc_at, u16_at, u32_at};
pub use chunk::{chunks, Chunk, ChunkIter};
pub use error::AviError;
pub use header::{parse_avih, AviHeader};
pub use index::{parse_idx1, FrameRef};
pub use stream::{is_video_strh, parse_strf_video, parse_strh_rate, VideoInfo};
