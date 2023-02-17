use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    Graph,
};
use thiserror::Error;

use crate::state::State;

#[derive(Error, Debug)]
pub enum AutomataErr {
    #[error("The automata contained no nodes")]
    NoNodes,

    #[error("A node contained an invalid number of valid transitions (found {0}, expected 1)")]
    InvalidTransitionCount(usize),
}

pub struct Automata<T> {
    graph: Graph<State, T>,
}

impl<T: PartialEq> Automata<T> {
    pub fn new() -> Automata<T> {
        Automata {
            graph: Graph::new(),
        }
    }

    pub fn add_state(&mut self, state: State) -> NodeIndex {
        self.graph.add_node(state)
    }

    pub fn add_transition(&mut self, a: NodeIndex, b: NodeIndex, symbol: T) -> EdgeIndex {
        self.graph.add_edge(a, b, symbol)
    }

    pub fn is_in_language<I>(&self, word: I) -> Result<bool, AutomataErr>
    where
        I: IntoIterator<Item = T>,
    {
        let head_index = self.root().ok_or(AutomataErr::NoNodes)?;

        for token in word {
            let valid_nodes: Vec<_> = self
                .graph
                .edges(head_index)
                .filter(|edge| *edge.weight() == token)
                .collect();

            if valid_nodes.len() != 1 {
                return Err(AutomataErr::InvalidTransitionCount(valid_nodes.len()));
            }
        }

        Ok(true)
    }

    fn root(&self) -> Option<NodeIndex> {
        self.graph.node_indices().next()
    }
}
