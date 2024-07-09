use crate::circuit::equations::CircuitEquation;
use crate::circuit::nodes::CircuitNode;
use evalexpr::HashMapContext;

#[derive(Debug,Clone,Default)]
pub struct Transistor {
    context: HashMapContext,
    equations: CircuitEquation,
    gate: CircuitNode,
    source: CircuitNode,
    drain: CircuitNode,
    body: CircuitNode,
}

