use std::fmt::{Display, Formatter, Result};

use crate::circuit::equations::CircuitEquation;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ElementHNode {
    equations: CircuitEquation,
}

impl ElementHNode {
    pub const fn new(equations: CircuitEquation) -> Self {
        Self { equations }
    }
}

impl Display for ElementHNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "equations: {}", self.equations)
    }
}
