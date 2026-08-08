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

use super::address::parse_address;
use super::exit::Exit;

/// Network requesters published by Nym, in the address form the network uses:
/// identity, encryption key, then the gateway they sit behind.
///
/// Shipping a list is a real choice with a real cost: an exit sees every host
/// a client asks for. These are published operators rather than ours, which
/// is the point. Whoever runs the machine can replace them, and the capsule
/// refuses to route at all rather than pick one unasked when the list is
/// exhausted.
pub const BOOTSTRAP_EXITS: [&str; 4] = [
    "Entztfv6Uaz2hpYHQJ6JKoaCTpDL5dja18SuQWVJAmmx.Cvhn9rBJw5Ay9wgHcbgCnVg89MPSV5s2muPV2YF1BXYu@Fo4f4SQLdoyoGkFae5TpVhRVoXCF8UiypLVGtGjujVPf",
    "D55ksecHzY6vAeqk8MCTzCfj2pqwJeKCKZCUUGnwGnn3.FS42vXS5a6GNTb1qk3aVk5mjSiJCAuawbBVyQZZVfhvt@DfNMqQRy6pPkU8Z5rBsxRwzDUzAMXHPFwMhjF16ScZqn",
    "DFdDtW7LNBATxQ4ef3jNbqs3cRE8b9wDZTCctHCQRULa.4AbKiTNVUwYFWHhy98o5pT9dELiUrkXoJQ9wHqPgf6GV@GJqd3ZxpXWSNxTfx7B1pPtswpetH4LnJdFeLeuY5KUuN",
    "6Y1HE1jJ92P9yoHer11TR4A2NdZePrLGaBNFf65MnYGe.FwXoh217odQDWNmViqzNX28fauYrjB3PYLrVvpqnQrX4@5vC8spDvw5VDQ8Zvd9fVvBhbUDv9jABR4cXzd4Kh5vz",
];

/// The nth published exit, parsed.
pub fn bootstrap_exit(index: usize) -> Option<Exit> {
    parse_address(BOOTSTRAP_EXITS.get(index)?.as_bytes())
}
