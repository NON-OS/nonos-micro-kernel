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

use nonos_policy_proto::Category;

use crate::settings::state::refresh_wifi::enter_wifi;
use crate::settings::state::set_category::set_category;
use crate::settings::state::State;

/// Advance to the next top-level section, cycling the three policy categories and
/// then the Wi-Fi tab: Display -> Network -> Security -> Wi-Fi -> Display.
pub fn next_category(state: &mut State) {
    if state.wifi_active {
        set_category(state, Category::User);
        return;
    }
    match state.category {
        Category::User => set_category(state, Category::Identity),
        Category::Identity => set_category(state, Category::Kernel),
        Category::Kernel => enter_wifi(state),
    }
}
