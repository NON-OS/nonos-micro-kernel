use crate::hid::{input_len, input_register, probe_bus};
use crate::i2c_client::resolve;
use crate::state::State;

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
    }
}
