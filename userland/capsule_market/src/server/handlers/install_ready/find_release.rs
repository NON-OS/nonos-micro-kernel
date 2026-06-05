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

use nonos_marketplace_abi::{CapsuleRelease, MarketplaceIndex};

pub(super) fn find_release<'a>(
    index: &'a MarketplaceIndex,
    listing_id: &str,
    release_id: &str,
) -> Option<(usize, usize, &'a CapsuleRelease)> {
    index.entries.iter().enumerate().find_map(|(entry_index, e)| {
        if e.listing_id != listing_id {
            return None;
        }
        e.releases
            .iter()
            .enumerate()
            .find(|(_, r)| r.release_id == release_id)
            .map(|(release_index, r)| (entry_index, release_index, r))
    })
}
