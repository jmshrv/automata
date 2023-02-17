use anyhow::{Ok, Result};

use automata::Automata;
use state::State;

mod automata;
mod state;

fn main() -> Result<()> {
    let mut automata = Automata::new();

    let s1 = automata.add_state(State::new("s1".to_string(), false));
    let s2 = automata.add_state(State::new("s2".to_string(), true));

    automata.add_transition(s1, s2, true);
    automata.add_transition(s1, s2, false);

    let end_state = automata.end_state([true])?;

    println!("{:?}", end_state.1);

    Ok(())
}
