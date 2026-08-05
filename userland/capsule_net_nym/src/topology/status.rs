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

use super::directory::Provenance;
use super::types::TopologyStatus;
use super::{clock, store};

pub fn current() -> TopologyStatus {
    let Some(meta) = store::meta() else {
        return TopologyStatus::Missing;
    };
    let Ok(now) = clock::now_ms() else {
        return TopologyStatus::Clock;
    };
    if now < meta.not_before_ms || now >= meta.not_after_ms {
        return TopologyStatus::Expired;
    }
    // Image tables are covered by the boot chain that already verified them,
    // and their age is bounded by the rollback index, not a directory window.
    if meta.provenance == Provenance::Image {
        return TopologyStatus::Ready;
    }
    if !super::admissible::admissible(meta, crate::state::trusted_authority) {
        return TopologyStatus::UntrustedAuthority;
    }
    TopologyStatus::Ready
}
