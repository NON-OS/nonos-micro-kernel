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

//! The driver against modelled capability regions and a part that consumes
//! the control queue.

mod harness;
mod model;
mod part;

mod wire;

mod legacy_tests;
mod modern_queue_tests;
mod modern_refusal_tests;
mod modern_tests;
mod queue_limit_tests;
mod queue_tests;
mod reply_refusal_tests;
mod reply_tests;
mod wire_scanout_tests;
mod wire_tests;
