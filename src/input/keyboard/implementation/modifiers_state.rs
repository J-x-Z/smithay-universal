use xkbcommon::xkb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ModifiersState {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub logo: bool,
    pub serialized: SerializedMods,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SerializedMods {
    pub depressed: u32,
    pub latched: u32,
    pub locked: u32,
    pub group: u32,
    pub layout_effective: u32,
}

impl ModifiersState {
    pub fn update_with(&mut self, _state: &xkb::State) {
        // Dummy impl
    }

    pub fn serialize_back(&self, _state: &xkb::State) -> SerializedMods {
        SerializedMods::default()
    }
}
