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

use super::stream::restart;
use crate::audio;
use crate::protocol::{Request, E_OK};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle(driver: &Driver, req: &Request, tx: &mut [u8], played: &mut bool) {
    audio::fill(driver.sample.user_va, driver.sample.length as usize);
    restart(driver);
    *played = false;
    reply_with_status(tx, req, E_OK);
}
