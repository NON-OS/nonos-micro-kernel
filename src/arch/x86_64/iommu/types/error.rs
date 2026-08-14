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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VtdError {
    NotPresent,
    /// Hardware exists but is not translating, so nothing here confines a
    /// device. Returned instead of success.
    NotEnforcing,
    DomainTableFull,
    DomainAlreadyExists,
    DomainNotFound,
    DeviceAlreadyAttached,
    DeviceNotAttached,
    AddressMisaligned,
    SizeMisaligned,
    RangeOutOfBounds,
    PageTableExhausted,
    /// The frame exists but is not reachable through the directmap, so it
    /// cannot be edited. Distinct from having no frame.
    TableUnreachable,
    RangeAlreadyMapped,
    RangeNotMapped,
    /// A map granting neither read nor write. The entry would read as absent,
    /// so the request is refused rather than reported as a mapping.
    NoPermissionsRequested,
    DepthUnknown,
    /// Firmware left translation enabled with its own tables. Taking them over
    /// would strand in-flight DMA.
    FirmwareOwnsUnit,
    Timeout,
}
