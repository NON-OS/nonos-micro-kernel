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

use alloc::string::String;
use alloc::vec::Vec;

use super::error::{Error, ErrorKind, Result};
use super::read::Read;

pub trait BufRead: Read {
    fn fill_buf(&mut self) -> Result<&[u8]>;
    fn consume(&mut self, amt: usize);

    fn read_until(&mut self, delim: u8, out: &mut Vec<u8>) -> Result<usize> {
        let mut total = 0;
        loop {
            let (done, used) = {
                let available = self.fill_buf()?;
                if available.is_empty() {
                    return Ok(total);
                }
                match available.iter().position(|&b| b == delim) {
                    Some(i) => {
                        out.extend_from_slice(&available[..=i]);
                        (true, i + 1)
                    }
                    None => {
                        out.extend_from_slice(available);
                        (false, available.len())
                    }
                }
            };
            self.consume(used);
            total += used;
            if done {
                return Ok(total);
            }
        }
    }

    fn read_line(&mut self, out: &mut String) -> Result<usize> {
        let mut bytes = Vec::new();
        let n = self.read_until(b'\n', &mut bytes)?;
        match core::str::from_utf8(&bytes) {
            Ok(s) => {
                out.push_str(s);
                Ok(n)
            }
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "stream is not valid utf-8")),
        }
    }
}
