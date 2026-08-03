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

use super::writer::Writer;
use crate::conn::Dest;

/// Render `dest` as the ASCII `host:port` a Nym exit parses, IPv6 bracketed
/// with full groups. Returns the byte count, or `None` if it does not fit.
pub fn write_hostport(dest: &Dest, out: &mut [u8]) -> Option<usize> {
    let mut w = Writer::new(out);
    match dest {
        Dest::V4(a, port) => {
            for (i, b) in a.iter().enumerate() {
                if i > 0 {
                    w.byte(b'.')?;
                }
                w.dec(*b as u16)?;
            }
            w.byte(b':')?;
            w.dec(*port)?;
        }
        Dest::V6(a, port) => {
            w.byte(b'[')?;
            for i in 0..8 {
                if i > 0 {
                    w.byte(b':')?;
                }
                let group = ((a[i * 2] as u16) << 8) | a[i * 2 + 1] as u16;
                w.hex(group)?;
            }
            w.byte(b']')?;
            w.byte(b':')?;
            w.dec(*port)?;
        }
        Dest::Domain { name, len, port } => {
            w.bytes(&name[..*len as usize])?;
            w.byte(b':')?;
            w.dec(*port)?;
        }
    }
    Some(w.len())
}
