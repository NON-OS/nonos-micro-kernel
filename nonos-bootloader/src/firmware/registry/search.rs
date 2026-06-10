// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::database::FirmwareDatabase;
use super::metadata::FirmwareMetadata;
use crate::firmware::detection::version::FirmwareVersion;
use crate::firmware::types::FirmwareType;

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub vendor_filter: Option<[u8; 32]>,
    pub version_min: Option<FirmwareVersion>,
    pub version_max: Option<FirmwareVersion>,
    pub feature_mask: u32,
    pub compatibility_flags: u16,
}
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub firmware_type: FirmwareType,
    pub metadata: FirmwareMetadata,
    pub relevance_score: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchFilter {
    ByVendor,
    ByVersion,
    ByFeatures,
    ByCompatibility,
    All,
}
impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            vendor_filter: None,
            version_min: None,
            version_max: None,
            feature_mask: 0,
            compatibility_flags: 0,
        }
    }
}

pub fn search_firmware(
    db: &FirmwareDatabase,
    query: &SearchQuery,
    filter: SearchFilter,
) -> alloc::vec::Vec<SearchResult> {
    let mut results = alloc::vec::Vec::new();
    for i in 0..256 {
        if let Some(entry) = &db.entries[i] {
            if matches_query(&entry.metadata, query, filter) {
                let score = relevance(&entry.metadata, query);
                if score > 0 {
                    results.push(SearchResult {
                        firmware_type: entry.firmware_type,
                        metadata: entry.metadata.clone(),
                        relevance_score: score,
                    });
                }
            }
        }
    }
    results.sort_by(|a, b| b.relevance_score.cmp(&a.relevance_score));
    results
}

fn matches_query(m: &FirmwareMetadata, q: &SearchQuery, f: SearchFilter) -> bool {
    match f {
        SearchFilter::ByVendor => q.vendor_filter.as_ref().map(|v| m.vendor == *v).unwrap_or(true),
        SearchFilter::ByVersion => {
            q.version_min.as_ref().map(|min| m.version.major >= min.major).unwrap_or(true)
                && q.version_max.as_ref().map(|max| m.version.major <= max.major).unwrap_or(true)
        }
        SearchFilter::ByFeatures => (m.features & q.feature_mask) == q.feature_mask,
        SearchFilter::ByCompatibility => {
            (m.compatibility_flags & q.compatibility_flags) == q.compatibility_flags
        }
        SearchFilter::All => {
            matches_query(m, q, SearchFilter::ByVendor)
                && matches_query(m, q, SearchFilter::ByVersion)
                && matches_query(m, q, SearchFilter::ByFeatures)
                && matches_query(m, q, SearchFilter::ByCompatibility)
        }
    }
}

fn relevance(m: &FirmwareMetadata, q: &SearchQuery) -> u8 {
    let mut s = 50u8;
    if q.vendor_filter.as_ref().map(|v| m.vendor == *v).unwrap_or(false) {
        s = s.saturating_add(30);
    }
    if q.version_min.as_ref().map(|min| m.version.major >= min.major).unwrap_or(false) {
        s = s.saturating_add(10);
    }
    if (m.features & q.feature_mask) == q.feature_mask {
        s = s.saturating_add(20);
    }
    if (m.compatibility_flags & q.compatibility_flags) == q.compatibility_flags {
        s = s.saturating_add(15);
    }
    s
}
