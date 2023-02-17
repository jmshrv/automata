#[derive(Debug)]
pub struct State {
    name: String,
    is_final: bool,
}

impl State {
    pub fn new(name: String, is_final: bool) -> State {
        State { name, is_final }
    }

    pub fn is_final(&self) -> bool {
        self.is_final
    }
}
