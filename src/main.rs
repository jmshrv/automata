use anyhow::{Ok, Result};

use automata::Automata;
use state::State;

mod automata;
mod state;

fn main() -> Result<()> {
    let mut automata = Automata::new();

    // let s1 = automata.add_state(State::new("s1".to_string(), false));
    // let s2 = automata.add_state(State::new("s2".to_string(), true));

    // automata.add_transition(s1, s2, true);
    // automata.add_transition(s1, s2, false);

    // let end_state = automata.end_state([true])?;

    let sa = automata.add_state(State::new('A', false));
    let sb = automata.add_state(State::new('B', true));
    let sc = automata.add_state(State::new('C', true));
    let sd = automata.add_state(State::new('D', true));
    let se = automata.add_state(State::new('E', true));
    let sf = automata.add_state(State::new('F', false));
    let sg = automata.add_state(State::new('G', true));
    let sh = automata.add_state(State::new('H', true));

    automata.add_transition(sa, sb, false);
    automata.add_transition(sa, sd, true);
    automata.add_transition(sb, sg, true);
    automata.add_transition(sb, sc, false);
    automata.add_transition(sc, sf, true);
    automata.add_transition(sc, sf, false);
    automata.add_transition(sd, sh, true);
    automata.add_transition(sd, se, false);
    automata.add_transition(se, sf, true);
    automata.add_transition(se, sf, false);
    automata.add_transition(sf, sf, true);
    automata.add_transition(sf, sf, false);
    automata.add_transition(sg, sg, true);
    automata.add_transition(sg, sf, false);
    automata.add_transition(sh, sh, true);
    automata.add_transition(sh, sf, false);

    let end_state_1 = automata.end_state([true, false, false, true, false, true, false])?; // F
    let end_state_2 = automata.end_state([false, false])?; // C

    println!("{:?}", end_state_1.1);
    println!("{:?}", end_state_2.1);

    Ok(())
}
