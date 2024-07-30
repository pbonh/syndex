use std::fmt::{Display, Formatter, Result};

use crate::circuit::nodes::CircuitNode;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct VoltageHEdge {
    node: CircuitNode,
}

impl VoltageHEdge {
    pub const fn new(node: CircuitNode) -> Self {
        Self { node }
    }
}

impl Display for VoltageHEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Node Name: {}", self.node)
    }
}
