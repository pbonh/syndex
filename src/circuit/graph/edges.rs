use std::fmt::{Display, Formatter, Result};

use crate::circuit::equations::CircuitEquation;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CircuitHyperEdge {
    equations: CircuitEquation,
}

impl CircuitHyperEdge {
    pub const fn new(equations: CircuitEquation) -> Self {
        Self { equations }
    }
}

impl Display for CircuitHyperEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "equations: {}", self.equations)
    }
}
