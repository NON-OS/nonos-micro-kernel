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
        match list_paths(mk_getpid(), AUDIO_DIR) {
            Ok(paths) => {
                let mut m = [0u8; 24];
                m[..16].copy_from_slice(b"[AP] audio n=   ");
                m[16] = b'0' + (paths.len() % 10) as u8;
                m[17] = b'\n';
                nonos_libc::mk_debug(m.as_ptr(), 18);
                for p in paths {
                    if is_audio(&p) {
                        tracks.push(Track::from_path(&p));
                    }
                }
            }
            Err(e) => {
                nonos_libc::mk_debug(b"[AP] audio list ERR: ".as_ptr(), 20);
                nonos_libc::mk_debug(e.as_ptr(), e.len());
                nonos_libc::mk_debug(b"\n".as_ptr(), 1);
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
