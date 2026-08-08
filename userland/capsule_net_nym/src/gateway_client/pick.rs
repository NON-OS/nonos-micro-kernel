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

use crate::state::{
    bootstrap_gateway, directory_gateway, directory_gateway_count, Gateway, BOOTSTRAP_GATEWAYS,
};

/// Choose a gateway to try.
///
/// The directory is preferred: the compiled list ages as operators come and
/// go, and a client that only ever dials the same few addresses is both
/// easier to starve and easier to recognise. The compiled list is what gets
/// us to a directory in the first place, so it stays as the cold start path
/// and nothing more.
pub fn pick(index: usize, offset: usize) -> Option<Gateway> {
    let published = directory_gateway_count();
    if published > 0 {
        crate::trace::say_num(b"dialling a directory gateway of", published as u64);
        return directory_gateway((offset + index) % published);
    }
    crate::trace::say(b"no directory gateways yet, dialling a compiled one");
    bootstrap_gateway((offset + index) % BOOTSTRAP_GATEWAYS.len())
}
