#[derive(Debug, Copy, Clone)]
pub enum KeyMap {
    ENTER = 0x28,
    StrumUp = 0x52,
    StrumDown = 0x51,
    Q = 0x14,
    W = 0x1a,
    E = 0x08,
    R = 0x15,
    T = 0x17,
    NULL = 0x00,
}

impl KeyMap {
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

const CODES: [KeyMap; 7] = [
    KeyMap::StrumUp,
    KeyMap::StrumDown,
    KeyMap::Q,
    KeyMap::W,
    KeyMap::E,
    KeyMap::R,
    KeyMap::T,
];
pub struct GamePad;

impl GamePad {
    pub fn to_array_code(states: &[bool; 7], result: &mut[u8;7])  {

        for (i, state) in states.iter().enumerate() {
            if *state { result[i] = CODES[i].to_u8() }
        }

    }
}
