use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::{EdgeRef, IntoNodeReferences},
    Direction, Graph,
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

pub struct Automata<S, T> {
    graph: Graph<State<S>, T>,
}

impl<S, T: PartialEq> Automata<S, T> {
    pub fn new() -> Automata<S, T> {
        Automata {
            graph: Graph::new(),
        }
    }

    pub fn add_state(&mut self, state: State<S>) -> NodeIndex {
        self.graph.add_node(state)
    }

    pub fn add_transition(&mut self, a: NodeIndex, b: NodeIndex, symbol: T) -> EdgeIndex {
        self.graph.add_edge(a, b, symbol)
    }

    pub fn end_state<I>(&self, word: I) -> Result<(NodeIndex, &State<S>), AutomataErr>
    where
        I: IntoIterator<Item = T>,
    {
        // Get the head of the automata, or return an error otherwise
        let mut node = self.root().ok_or(AutomataErr::NoNodes)?;

        for token in word {
            let valid_edges: Vec<_> = self
                .graph
                .edges_directed(node.0, Direction::Outgoing)
                .filter(|edge| {
                    return *edge.weight() == token;
                })
                .collect();

            if valid_edges.len() != 1 {
                return Err(AutomataErr::InvalidTransitionCount(valid_edges.len()));
            }

            let edge_to_follow = valid_edges
                .first()
                .expect("No nodes despite just checking?");

            node = (
                edge_to_follow.target(),
                &self.graph[edge_to_follow.target()],
            );
        }

        Ok(node)
    }

    fn root(&self) -> Option<(NodeIndex, &State<S>)> {
        self.graph.node_references().next()
    }
}
