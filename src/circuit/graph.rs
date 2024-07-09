use bevy_ecs::prelude::Resource;
use bevy_ecs::prelude::Entity;
use hypergraph::Hypergraph;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct VoltageNode<'node_str> {
    node: &'node_str str,
}

impl<'node_str> VoltageNode<'node_str> {
    pub const fn new(node: &'node_str str) -> Self {
        Self { node }
    }
}

impl<'node_str> Display for VoltageNode<'node_str> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.node)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct CircuitHyperEdge<'eq_str> {
    entity_id: Entity,
    equations: &'eq_str str,
}

impl<'eq_str> CircuitHyperEdge<'eq_str> {
    pub const fn new(equations: &'eq_str str, entity_id: Entity) -> Self {
        Self {
            entity_id,
            equations,
        }
    }
}

impl<'eq_str> Display for CircuitHyperEdge<'eq_str> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "element_id: {} equations: {}",
            self.entity_id.to_bits(), self.equations
        )
    }
}

impl<'eq_str> Into<usize> for CircuitHyperEdge<'eq_str> {
    fn into(self) -> usize {
        self.entity_id.to_bits().try_into().expect("Unable to convert u64 to usize.")
    }
}

#[derive(Debug, Default, Resource)]
pub struct LCircuit<'node_str, 'eq_str>(
    Hypergraph<VoltageNode<'node_str>, CircuitHyperEdge<'eq_str>>,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_circuit() {
        let _circuit = LCircuit::default();
    }

    // #[test]
    // fn simple_inverter_macro() {
    //     let cmos_inverter = circuit! {
    //         transistor {
    //             name = "M1";
    //             drain = "out";
    //             gate = "in";
    //             source = "vss";
    //             body = "vss";
    //             type_ = "NMOS";
    //             model = "NMOS_IV";
    //         }
    //         transistor {
    //             name = "M2";
    //             drain = "out";
    //             gate = "in";
    //             source = "vdd";
    //             body = "vdd";
    //             type_ = "PMOS";
    //             model = "PMOS_IV";
    //         }
    //     };
    //         // resistor {
    //         //     name = "R1";
    //         //     n1 = "out";
    //         //     n2 = "vdd";
    //         //     value = "10k";
    //         // }
    //         // inductor {
    //         //     name = "L1";
    //         //     n1 = "out";
    //         //     n2 = "vdd";
    //         //     value = "10mH";
    //         // }
    //         // capacitor {
    //         //     name = "C1";
    //         //     n1 = "out";
    //         //     n2 = "vdd";
    //         //     value = "100nF";
    //         // }
    // }
}
