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

// The real capability arithmetic of the spawn gate. check_ceiling and
// check_grant in the kernel delegate to this.
#[allow(dead_code)]
#[path = "../../../../src/security/capsule_manifest/verify/caps_bits.rs"]
pub mod caps_bits;

// The real expiry meet of a capability delegation. create_delegation
// delegates to this.
#[allow(dead_code)]
#[path = "../../../../src/capabilities/delegation/lifetime.rs"]
pub mod lifetime;
