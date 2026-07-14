mod descriptor;
mod input_len;
mod input_mode;
mod input_register;
mod power;
mod probe;
mod report_desc;

pub use descriptor::{valid_descriptor, HID_DESC_LEN};
pub use input_len::input_len;
pub use input_mode::set_ptp_mode;
pub use input_register::input_register;
pub use power::wake;
pub use probe::{probe_addr, probe_bus};
pub use report_desc::{decode_touch, parse as parse_report_descriptor, TouchLayout};
