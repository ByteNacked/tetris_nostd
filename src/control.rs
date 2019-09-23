#[derive(Copy, Clone)]
pub struct Control {
    pub left: bool,
    pub right: bool,
    pub dash: bool,
    pub fall: bool,
    pub rotate: bool,
}

impl Default for Control {
    fn default() -> Self {
        Control {
            left: false,
            right: false,
            dash: false,
            fall: false,
            rotate: false,
        }
    }
}
