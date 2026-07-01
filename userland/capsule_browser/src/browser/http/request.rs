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

use crate::browser::url::Url;

pub fn build(url: &Url) -> String {
    let mut r = String::new();
    r.push_str("GET ");
    r.push_str(crate::browser::url::request_target(url));
    r.push_str(" HTTP/1.1\r\nHost: ");
    r.push_str(&crate::browser::url::authority(url));
    r.push_str("\r\nUser-Agent: nonos-browser/0.1\r\n");
    r.push_str("Accept: text/html,text/plain,application/json,*/*;q=0.1\r\n");
    r.push_str("Accept-Encoding: gzip, deflate\r\nConnection: close\r\n\r\n");
    r
}
