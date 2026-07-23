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

mod decode;
mod encode;
mod endpoint;
mod errno;
mod header;
mod limits;
mod ops;

pub use decode::decode_request;
pub use encode::{encode_response_header, write_status};
pub use endpoint::{KERNEL_REPLY_ENDPOINT, SERVICE_NAME};
pub use errno::{E_AGAIN, E_INVAL, E_OK};
pub use header::{Request, HDR_LEN, RESP_HDR_LEN};
pub use limits::{
    CODEC_ENTRY_BYTES, CODEC_LIST_HEADER_BYTES, CODEC_MASK_PAYLOAD_LEN,
    CONTROLLER_INFO_PAYLOAD_LEN, MAX_CODEC_LIST_BYTES, MAX_PCM_CHUNK, MAX_STREAM_LAYOUT_BYTES,
    STATUS_LEN, STREAM_ENTRY_BYTES, STREAM_LAYOUT_HEADER_BYTES,
};
pub use ops::{
    OP_CODEC_LIST, OP_CODEC_MASK, OP_CONTROLLER_INFO, OP_HEALTHCHECK, OP_PLAY_TONE,
    OP_STREAM_LAYOUT, OP_STREAM_START, OP_STREAM_STOP, OP_WRITE_PCM,
};
