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

// Copy a file by reading the source whole and writing it to a freshly
// truncated destination. The store has no server-side copy, so it goes
// through the same open/read/write path a caller would.

use crate::io;
use crate::path::Path;
use crate::sys::fs::nonos::file::File;
use crate::sys::fs::nonos::open_options::OpenOptions;
use crate::vec::Vec;

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    let mut ro = OpenOptions::new();
    ro.read(true);
    let mut data = Vec::new();
    {
        let f = File::open(from, &ro)?;
        let mut chunk = [0u8; 4096];
        loop {
            let n = f.read(&mut chunk)?;
            if n == 0 {
                break;
            }
            data.extend_from_slice(&chunk[..n]);
        }
    }
    let mut wo = OpenOptions::new();
    wo.write(true);
    wo.create(true);
    wo.truncate(true);
    let out = File::open(to, &wo)?;
    out.write(&data)?;
    Ok(data.len() as u64)
}
