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

use super::decode;
use super::store::Store;

// Decode a fetched image body and record the outcome against `url`.
pub fn ingest(store: &mut Store, url: &str, body: &[u8]) {
    match decode::decode_body(body) {
        Ok(d) => store.set_ready(url, d),
        Err(_) => store.set_failed(url),
    }
}
