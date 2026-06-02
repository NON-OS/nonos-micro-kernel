pub const DONE: u8 = 10;

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

pub fn list_nav(sel: &mut u8, len: u8, code: u32) -> Option<Outcome> {
    match code {
        0x6B => {
            *sel = sel.saturating_sub(1);
            Some(Outcome::Stay)
        }
        0x6A => {
            if *sel + 1 < len {
                *sel += 1;
            }
            Some(Outcome::Stay)
        }
        0x31..=0x39 => {
            let i = (code - 0x31) as u8;
            if i < len {
                *sel = i;
            }
            Some(Outcome::Stay)
        }
        _ => None,
    }
}
