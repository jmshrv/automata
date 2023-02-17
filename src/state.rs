#[derive(Debug)]
pub struct State<T> {
    name: T,
    is_final: bool,
}

impl<T> State<T> {
    pub fn new(name: T, is_final: bool) -> State<T> {
        State { name, is_final }
    }

    pub fn is_final(&self) -> bool {
        self.is_final
    }
}
