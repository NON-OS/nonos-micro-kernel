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
use super::types::RegisterGrant;
use nonos_libc::{mk_mmio_unmap, mk_pio_release};

impl RegisterGrant {
    pub fn release(self) -> bool {
        match self {
            Self::Mmio(g) => mk_mmio_unmap(g.grant_id) >= 0,
            Self::Pio(g) => mk_pio_release(g.grant_id) >= 0,
        }
    }
}
