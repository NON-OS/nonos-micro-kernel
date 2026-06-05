pub struct State {
    pub i2c_port: u32,
    pub i2c_pid: u32,
    pub addr: u8,
    pub descriptor: [u8; 30],
    pub descriptor_len: usize,
    pub input_register: u16,
    pub input_len: usize,
    pub last_buttons: u8,
    pub probes: u64,
    pub input_polls: u64,
    pub input_reports: u64,
    pub post_failures: u64,
}

impl State {
    pub const fn new(i2c_port: u32, i2c_pid: u32) -> Self {
        Self {
            i2c_port,
            i2c_pid,
            addr: 0,
            descriptor: [0; 30],
            descriptor_len: 0,
            input_register: 0,
            input_len: 0,
            last_buttons: 0,
            probes: 0,
            input_polls: 0,
            input_reports: 0,
            post_failures: 0,
        }
    }

    pub fn found(&self) -> bool {
        self.descriptor_len != 0
    }
}
