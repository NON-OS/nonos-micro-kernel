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

use nonos_app_skeleton::{
    App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer, WindowKind, KEY_ESC,
};
use nonos_std::collections::HashMap;
use nonos_std::format;

pub struct GuiProof {
    clicks: u32,
    labels: HashMap<u32, &'static str>,
}

impl GuiProof {
    pub fn new() -> Self {
        let mut labels = HashMap::default();
        labels.insert(0, "nonos_std drives this GUI");
        Self { clicks: 0, labels }
    }
}

impl App for GuiProof {
    fn manifest(&self) -> AppManifest {
        AppManifest {
            title: b"std GUI proof",
            window_id: 0x5354_4447,
            kind: WindowKind::Normal,
            initial_x: 160,
            initial_y: 140,
            width: 360,
            height: 200,
            input_kind_mask: (1 << 0) | (1 << 5),
        }
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        match event.kind {
            InputKind::ButtonDown => {
                self.clicks += 1;
                EventOutcome::Repaint
            }
            InputKind::KeyDown if event.code == KEY_ESC => EventOutcome::Close,
            _ => EventOutcome::Idle,
        }
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        fb.clear(0xFF10_1820);
        fb.fill_rect(20, 20, 320, 60, 0xFF2A_7FFF);
        let label = self.labels.get(&0).copied().unwrap_or("");
        fb.text(34, 44, label.as_bytes(), 0xFFFF_FFFF);
        let line = format!("clicks: {}", self.clicks);
        fb.text(34, 120, line.as_bytes(), 0xFFCF_E8FF);
    }
}
