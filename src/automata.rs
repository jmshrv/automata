use petgraph::{
    stable_graph::{EdgeIndex, NodeIndex},
    Graph,
};

use crate::{state::State, symbol::Symbol};

pub struct Automata {
    graph: Graph<State, Symbol>,
}

impl Automata {
    pub fn new() -> Automata {
        Automata {
            graph: Graph::new(),
        }
    }

    pub fn add_state(&mut self, state: State) -> NodeIndex {
        self.graph.add_node(state)
    }

    pub fn add_transition(&mut self, a: NodeIndex, b: NodeIndex, symbol: Symbol) -> EdgeIndex {
        self.graph.add_edge(a, b, symbol)
    }
}
