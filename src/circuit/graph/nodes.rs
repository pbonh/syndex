use std::fmt::{Display, Formatter, Result};

use crate::circuit::nodes::CircuitNode;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct VoltageNode {
    node: CircuitNode,
}

impl VoltageNode {
    pub const fn new(node: CircuitNode) -> Self {
        Self { node }
    }
}

impl Display for VoltageNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.node)
    }
}
