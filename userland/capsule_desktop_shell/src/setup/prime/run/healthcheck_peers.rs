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

//! The window manager is required: without it there is no desktop to paint.
//! The capsule market is not. Discovery already yields port 0 when the market
//! is absent, and every caller tolerates that, so a market that registers but
//! does not answer is disabled here rather than aborting the whole startup.

use crate::market_client;
use crate::setup::prime::peers::Peers;
use crate::wm_client;

pub fn healthcheck_peers(peers: &Peers) -> Result<(), &'static str> {
    super::require_status::require_status(wm_client::healthcheck(peers.wm_port, 2))?;
    if peers.market_port != 0
        && super::require_status::require_status(market_client::healthcheck(peers.market_port, 4))
            .is_err()
    {
        crate::setup::discover::disable_market();
    }
    Ok(())
}
