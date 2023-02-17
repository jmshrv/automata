pub struct State {
    name: String,
    is_final: bool,
}

impl State {
    pub fn new(name: String, is_final: bool) -> State {
        State { name, is_final }
    }
}
