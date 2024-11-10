#[derive(Debug, Copy, Clone)]
pub enum KeyMap {
    ENTER = 0x28,
    STRUMUP = 0x52,
    STRUMDOWN = 0x51,
    Q = 0x14,
    W = 0x1a,
    E = 0x08,
    R = 0x15,
    T = 0x17,
    NULL = 0x00,
}

pub struct Button {
    pub code: KeyMap,
    pub status: bool,
}

impl Button {
    fn new(code: KeyMap, status: bool) -> Self {
        Self { code, status }
    }
    pub fn get(&self) -> &KeyMap {
        if self.status {
            return &self.code;
        }

        &KeyMap::NULL
    }
}

pub struct GamePad {
    pub enter: Button,
    pub strum_up: Button,
    pub strum_down: Button,
    pub cross: Button,
    pub circle: Button,
    pub square: Button,
    pub triangule: Button,
    pub l1: Button,
}

impl GamePad {
    pub fn new() -> Self {
        let enter = Button::new(KeyMap::ENTER, false);
        let strum_up = Button::new(KeyMap::STRUMUP, false);
        let strum_down = Button::new(KeyMap::STRUMDOWN, false);
        let cross = Button::new(KeyMap::Q, false);
        let circle = Button::new(KeyMap::W, false);
        let square = Button::new(KeyMap::E, false);
        let triangule = Button::new(KeyMap::R, false);
        let l1 = Button::new(KeyMap::T, false);

        Self {
            enter,
            strum_up,
            strum_down,
            cross,
            circle,
            square,
            triangule,
            l1,
        }
    }
}
