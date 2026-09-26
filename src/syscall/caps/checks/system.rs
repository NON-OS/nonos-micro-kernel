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

use crate::capabilities::{Capability, CapabilityToken};

impl CapabilityToken {
    #[inline]
    pub fn can_debug(&self) -> bool {
        self.grants(Capability::Debug) && self.is_valid()
    }
    #[inline]
    pub fn can_admin(&self) -> bool {
        self.grants(Capability::Admin) && self.is_valid()
    }
    #[inline]
    pub fn can_local_sign(&self) -> bool {
        self.grants(Capability::LocalSign) && self.is_valid()
    }
    /// The right to ask for a package to be installed. Deliberately
    /// separate from `ForeignExec`: the asker never hosts anything.
    #[inline]
    pub fn can_app_install(&self) -> bool {
        self.grants(Capability::AppInstall) && self.is_valid()
    }
    #[inline]
    pub fn can_attest_read(&self) -> bool {
        self.grants(Capability::AttestRead) && self.is_valid()
    }
    #[inline]
    pub fn can_store_write(&self) -> bool {
        self.grants(Capability::StoreWrite) && self.is_valid()
    }
    #[inline]
    pub fn can_foreign_exec(&self) -> bool {
        self.grants(Capability::ForeignExec) && self.is_valid()
    }
}
