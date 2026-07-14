use crate::hid::TouchLayout;
use crate::input::TouchGesture;

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
    pub woke: bool,
    /// The touch field map has decoded at least one real report. Until then
    /// the relative fallback stays available: a device that never streams
    /// its absolute collection must not go silent. Once proven, unmatched
    /// frames are dropped instead of being misread as boot-mouse packets.
    pub touch_decoded: bool,
    /// Polls since the last successful touch decode. When the absolute
    /// stream goes quiet for long enough (mode changed back, device reset),
    /// the relative fallback reopens rather than staying muted forever.
    pub polls_since_touch: u32,
    /// The GPIO doorbell has fired at least once, proving the platform's
    /// interrupt-status bit really tracks this pad. From then on the driver
    /// reads the i2c input register only on a fired doorbell: exact
    /// interrupt pacing, no stale re-reads. Until proven, timed polling
    /// continues so a doorbell that never latches cannot silence input.
    pub doorbell_proven: bool,
    /// Snapshot of the last raw frame and how often it has repeated
    /// verbatim. Polled reads return the current report whether or not it is
    /// new; without this, one stale motion report re-read at poll rate is a
    /// phantom input stream that drifts the cursor on its own.
    pub last_frame: [u8; 16],
    pub last_frame_len: usize,
    pub frame_repeats: u32,
    /// Raw frames already dumped to the boot console (bounded one-shot).
    pub frame_dumps: u32,
    // Absolute-touchpad field map parsed from the HID report descriptor, and the
    // gesture state that turns those reports into pointer events. Empty when the
    // device is a plain relative mouse.
    pub touch_layout: TouchLayout,
    pub gesture: TouchGesture,
}

impl State {
    pub fn new(i2c_port: u32, i2c_pid: u32) -> Self {
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
            woke: false,
            touch_decoded: false,
            polls_since_touch: 0,
            doorbell_proven: false,
            last_frame: [0; 16],
            last_frame_len: 0,
            frame_repeats: 0,
            frame_dumps: 0,
            touch_layout: TouchLayout::default(),
            gesture: TouchGesture::default(),
        }
    }

    pub fn found(&self) -> bool {
        self.descriptor_len != 0
    }
}
