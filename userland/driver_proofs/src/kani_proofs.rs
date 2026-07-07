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

//! Kani harness: the block I/O bounds hold for every request body and capacity.

use crate::constants::ata::MAX_SECTORS;
use crate::server::handlers::parse_rw;

// For every 12-byte request and every capacity: parsing is total, and an
// accepted request has a bounded sector count and never reaches past the disk.
#[kani::proof]
fn rw_parse_is_total_and_bounded() {
    let body: [u8; 12] = kani::any();
    let capacity: u64 = kani::any();

    if let Ok((lba, nsectors)) = parse_rw(&body, capacity) {
        assert!((1..=MAX_SECTORS).contains(&nsectors));
        let last = lba.checked_add(nsectors as u64);
        assert!(last.is_some());
        assert!(last.unwrap() <= capacity);
    }
}
