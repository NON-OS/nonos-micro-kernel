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
mod descriptors;
mod direction;
mod header;
mod publish;
use super::layout::Queue;
pub use direction::Direction;
impl Queue {
    pub fn post_request(&self, dir: Direction, lba: u64, nsectors: u32) {
        unsafe {
            self.write_header(dir, lba);
            self.write_descriptor_chain(dir, nsectors);
            self.publish_avail();
        }
    }
}