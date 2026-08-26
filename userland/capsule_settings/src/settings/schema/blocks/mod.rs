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

mod appearance;
mod developer;
mod general;
mod network;
mod privacy;
mod security;
mod sound;
mod storage;
mod updates;
mod wifi;

pub use appearance::APPEARANCE;
pub use developer::DEVELOPER;
pub use general::GENERAL;
pub use network::NETWORK;
pub use privacy::PRIVACY;
pub use security::SECURITY;
pub use sound::SOUND;
pub use storage::STORAGE;
pub use updates::UPDATES;
pub use wifi::WIFI;
