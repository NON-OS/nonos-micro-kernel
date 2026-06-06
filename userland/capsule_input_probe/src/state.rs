pub struct Context {
    pub base: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub compositor_port: u32,
    pub router_port: u32,
    pub cursor: usize,
    pub buf: [u8; 64],
}

impl Context {
    pub fn new(
        base: u64,
        width: u32,
        height: u32,
        stride: u32,
        compositor_port: u32,
        router_port: u32,
    ) -> Self {
        Self {
            base,
            width,
            height,
            stride,
            compositor_port,
            router_port,
            cursor: 0,
            buf: [0u8; 64],
        }
    }
}
