use crate::circuit::equations::CircuitEquation;
use crate::circuit::nodes::CircuitNode;
use evalexpr::HashMapContext;

#[derive(Debug,Clone)]
pub struct Transistor {
    context: HashMapContext,
    equations: CircuitEquation,
    gate: CircuitNode,
    source: CircuitNode,
    drain: CircuitNode,
    body: CircuitNode,
}

