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


use nonos_app_skeleton::clients::vfs::{read_file, stat};
use nonos_avi::parse_avih;

use super::kind::MediaKind;
use super::media::MediaItem;
use super::thumb;

const HEAD_BYTES: u32 = 512 * 1024;
const AVIH: &[u8; 4] = b"avih";

pub fn probe(owner_pid: u32, item: &mut MediaItem) {
    if let Ok((size, _)) = stat(owner_pid, item.path.as_bytes()) {
        item.size = size;
    }
    if item.kind != MediaKind::Avi {
        return;
    }
    let Ok(head) = read_file(owner_pid, item.path.as_bytes(), HEAD_BYTES) else {
        return;
    };
    let Some(at) = find_avih(&head) else { return };
    let Ok(header) = parse_avih(&head[at..]) else {
        return;
    };
    item.width = header.width;
    item.height = header.height;
    item.duration_ms =
        (header.total_frames as u64 * header.micro_sec_per_frame as u64 / 1000) as i64;
    item.thumb = thumb::extract(&head, header.width, header.height);
}

fn find_avih(head: &[u8]) -> Option<usize> {
    head.windows(4).position(|w| w == AVIH).map(|at| at + 8)
}
