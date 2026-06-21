// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use spin::Once;

use super::backend::Backend;

static SELECTED: Once<Backend> = Once::new();

pub fn selected() -> Backend {
    *SELECTED.call_once(|| {
        if matches!(crate::hardware::nvme_capsule::capacity(), Ok(s) if s > 0) {
            return Backend::Nvme;
        }
        if matches!(crate::hardware::ahci_capsule::capacity(), Ok(s) if s > 0) {
            return Backend::Ahci;
        }
        Backend::VirtioBlk
    })
}
