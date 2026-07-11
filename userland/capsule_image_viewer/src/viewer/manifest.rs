use nonos_app_skeleton::{AppManifest, WindowKind};

pub fn manifest() -> AppManifest {
    AppManifest {
        title: b"Images",
        window_id: 0x1004,
        kind: WindowKind::Normal,
        initial_x: 180,
        initial_y: 120,
        width: 720,
        height: 520,
        input_kind_mask: (1 << 0) | (1 << 3) | (1 << 4) | (1 << 5),
    }
}
