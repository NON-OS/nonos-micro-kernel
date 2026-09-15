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

//! What the status line says about a vault this machine will not open.
//!
//! Two situations, told apart by whether a key was produced at all. No key
//! means the TPM did not answer, and the machine needs its boot state looked
//! at. A key that did not open the record means the record was sealed under
//! a different key: this machine before a firmware or kernel change, or
//! another machine. The bytes cannot say which, so the sentence names both
//! ways out rather than pretending to know.

pub(super) fn sealed(machine_changed: bool) -> &'static [u8] {
    if machine_changed {
        b"a wallet is stored here; no key came from the TPM this boot"
    } else {
        b"a wallet is stored here, sealed under a key this boot does not derive: earlier firmware, or another machine, or the phrase"
    }
}
