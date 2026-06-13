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

mod bufread;
mod bufreader;
mod bufwriter;
mod copy;
mod cursor;
mod error;
mod lines;
mod macros;
mod read;
mod seek;
mod slice;
mod stdio;
mod write;
mod writevec;

pub use bufread::BufRead;
pub use bufreader::BufReader;
pub use bufwriter::BufWriter;
pub use copy::copy;
pub use cursor::Cursor;
pub use error::{Error, ErrorKind, Result};
pub use lines::Lines;
pub use read::Read;
pub use seek::{Seek, SeekFrom};
pub use stdio::{stderr, stdout, Stderr, Stdout};
pub use write::Write;
