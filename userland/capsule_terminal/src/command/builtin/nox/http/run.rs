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

use super::emit::emit;
use super::url::parse_url;
use super::{plain, secure};
use crate::term::state::State;

const USAGE: &[u8] =
    b"usage: http <url>   e.g. http example.com  |  http https://host/path  |  http host:8080";

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    let Some(&raw) = args.first() else {
        state.scrollback.push_error(USAGE);
        return false;
    };
    let Some(url) = parse_url(raw) else {
        state.scrollback.push_error(b"http: bad url");
        return false;
    };

    let result = if url.secure { secure::get(&url) } else { plain::get(&url) };
    match result {
        Ok(response) => {
            emit(state, &response);
            true
        }
        Err(reason) => {
            let mut line = alloc::vec::Vec::from(*b"http: ");
            line.extend_from_slice(reason.as_bytes());
            state.scrollback.push_error(&line);
            false
        }
    }
}
