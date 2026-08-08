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

//! The package-install prompt the consent modal is raised for.

use alloc::vec::Vec;

/// A /pkgs package whose install is waiting on the user, together with the
/// summary the installer already verified. The digest travels with it so the
/// commit re-verifies the exact bytes the prompt described, and never a file
/// swapped underneath in the meantime.
pub struct PkgInstallPrompt {
    pub path: Vec<u8>,
    pub summary: crate::installer_client::PkgSummary,
}
