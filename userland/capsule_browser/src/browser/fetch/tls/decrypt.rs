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

use alloc::vec::Vec;

use crate::browser::fetch::types::Fetch;
use crate::browser::tls13;

pub(in crate::browser::fetch) fn decrypt(f: &Fetch) -> Option<Vec<u8>> {
    let tls = f.tls.as_ref()?;
    tls13::application_plaintext(&tls.cf, &tls.flight, &f.buf, f.url.host.as_bytes(), tls.now)
}
