pub const DONE: u8 = 6;

pub const K_ENTER: u32 = 0x0D;
pub const K_ENTER_LF: u32 = 0x0A;
pub const K_ESC: u32 = 0x1B;
pub const K_BACKSPACE: u32 = 0x08;

pub enum Outcome {
    Advance,
    Back,
    Stay,
}

pub fn apply(step: u8, outcome: Outcome) -> u8 {
    match outcome {
        Outcome::Advance => (step + 1).min(DONE),
        Outcome::Back => step.saturating_sub(1),
        Outcome::Stay => step,
    }
}

pub fn default_key(code: u32) -> Outcome {
    match code {
        K_ENTER | K_ENTER_LF => Outcome::Advance,
        K_ESC => Outcome::Back,
        _ => Outcome::Stay,
    }
}
