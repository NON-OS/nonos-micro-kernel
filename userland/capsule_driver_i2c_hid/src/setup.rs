use crate::hid::{input_len, input_register, parse_report_descriptor, probe_bus};
use crate::i2c_client::resolve;
use crate::i2c_client::write_read;
use crate::state::State;

/// Upper bound on the report descriptor we will read and parse.
const REPORT_DESC_MAX: usize = 512;

pub fn run() -> Result<State, &'static str> {
    let (port, pid) = resolve().ok_or("i2c-hid: missing i2c controller")?;
    let mut state = State::new(port, pid);
    reprobe(&mut state);
    Ok(state)
}

pub fn reprobe(state: &mut State) {
    state.probes += 1;
    if let Some((addr, len)) = probe_bus(state.i2c_port, &mut state.descriptor) {
        state.addr = addr;
        state.descriptor_len = len;
        state.input_register = input_register(&state.descriptor);
        state.input_len = input_len(&state.descriptor);
        state.touch_layout = read_touch_layout(state);
    }
}

// Fetch the HID report descriptor named in the HID descriptor and parse it into
// a touchpad field map. A device that is not an absolute touchpad simply yields
// an empty layout, and the driver stays on the relative decode path.
fn read_touch_layout(state: &State) -> crate::hid::TouchLayout {
    let desc = &state.descriptor;
    let rd_len = u16::from_le_bytes([desc[4], desc[5]]) as usize;
    let rd_reg = u16::from_le_bytes([desc[6], desc[7]]);
    if !(4..=REPORT_DESC_MAX).contains(&rd_len) {
        return crate::hid::TouchLayout::default();
    }
    let mut buf = [0u8; REPORT_DESC_MAX];
    match write_read(state.i2c_port, state.addr, &rd_reg.to_le_bytes(), &mut buf[..rd_len]) {
        Some(n) => parse_report_descriptor(&buf[..n]),
        None => crate::hid::TouchLayout::default(),
    }
}
