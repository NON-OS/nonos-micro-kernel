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

use super::limit::identity_limit;
use crate::arch::x86_64::iommu::domain::create_domain;
use crate::arch::x86_64::iommu::globals::allocate_domain_id;
use crate::arch::x86_64::iommu::globals::state::STATE;
use crate::arch::x86_64::iommu::mapping::map_identity;
use crate::arch::x86_64::iommu::regs::cap;
use crate::arch::x86_64::iommu::types::{DomainId, VtdError};

/// The domain enumerated devices start in, mapped one-to-one.
pub(super) fn identity_domain(levels: u8, cap_word: u64) -> Result<(DomainId, u64), VtdError> {
    let id = DomainId::new(allocate_domain_id() as u16);
    create_domain(id)?;
    let root = {
        let state = STATE.lock();
        state.domains[id.as_u16() as usize].root
    };
    map_identity(root, levels, identity_limit(), cap::best_leaf_level(cap_word))?;
    Ok((id, root))
}
