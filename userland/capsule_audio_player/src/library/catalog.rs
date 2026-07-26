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

extern crate alloc;
use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::list_paths;
use nonos_libc::mk_getpid;

use super::track::Track;

const AUDIO_DIR: &[u8] = b"/audio";

pub struct Library {
    pub tracks: Vec<Track>,
}

impl Library {
    pub fn scan() -> Self {
        let mut tracks = Vec::new();
        if let Ok(paths) = list_paths(mk_getpid(), AUDIO_DIR) {
            for p in paths {
                if is_audio(&p) {
                    tracks.push(Track::from_path(&p));
                }
            }
        }
        Library { tracks }
    }

    pub fn get(&self, i: usize) -> Option<&Track> {
        self.tracks.get(i)
    }
}

fn is_audio(path: &str) -> bool {
    let ext = match path.rfind('.') {
        Some(i) => &path[i + 1..],
        None => return false,
    };
    ext.eq_ignore_ascii_case("wav")
        || ext.eq_ignore_ascii_case("mp3")
        || ext.eq_ignore_ascii_case("flac")
        || ext.eq_ignore_ascii_case("ogg")
}
