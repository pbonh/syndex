use crate::circuit::equations::CircuitEquation;
use crate::circuit::nodes::CircuitNode;
use bevy_ecs::prelude::Entity;
use bevy_ecs::prelude::Resource;
use hypergraph::Hypergraph;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct VoltageNode<'node_str> {
    node: &'node_str CircuitNode,
}

impl<'node_str> VoltageNode<'node_str> {
    pub const fn new(node: &'node_str CircuitNode) -> Self {
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
    equations: &'eq_str CircuitEquation,
}

impl<'eq_str> CircuitHyperEdge<'eq_str> {
    pub const fn new(equations: &'eq_str CircuitEquation, entity_id: Entity) -> Self {
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
            self.entity_id.to_bits(),
            self.equations
        )
    }
}

impl<'eq_str> Into<usize> for CircuitHyperEdge<'eq_str> {
    fn into(self) -> usize {
        self.entity_id
            .to_bits()
            .try_into()
            .expect("Unable to convert u64 to usize.")
    }
}

#[derive(Debug, Default, Resource)]
pub struct LCircuit<'node_str, 'eq_str>(
    Hypergraph<VoltageNode<'node_str>, CircuitHyperEdge<'eq_str>>,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::elements::*;
    use crate::circuit::equations::*;
    use std::str::FromStr;

    #[test]
    fn default_circuit() {
        let _circuit = LCircuit::default();
    }

    #[test]
    fn simple_inverter_macro() {
        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vd/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();
        let x1 = CircuitElement::from_str("x1").unwrap();
        let n1 = CircuitNode::from_str("n1").unwrap();
        let n2 = CircuitNode::from_str("n2").unwrap();
        let n3 = CircuitNode::from_str("n3").unwrap();
        let n4 = CircuitNode::from_str("n4").unwrap();
        let node_eq_str: String = "(".to_owned() + &n1.to_string() + " - " + &n2.to_string() + ")";
        let node_eq = DeviceEquation::from_str(&node_eq_str).unwrap();
        let ctx = VariableContextMap::from([("vd".to_string(), node_eq)]);
        let circuit_eq = CircuitEquation::new(dev_eq, ctx);
        let _transistor = transistor::Transistor::builder()
            .name(x1)
            .equations(circuit_eq)
            .gate(n1)
            .source(n2)
            .drain(n3)
            .body(n4)
            .build();
        // let cmos_inverter = circuit! {
        //     transistor {
        //         name = "M1";
        //         drain = "out";
        //         gate = "in";
        //         source = "vss";
        //         body = "vss";
        //         type_ = "NMOS";
        //         model = "NMOS_IV";
        //     }
        //     transistor {
        //         name = "M2";
        //         drain = "out";
        //         gate = "in";
        //         source = "vdd";
        //         body = "vdd";
        //         type_ = "PMOS";
        //         model = "PMOS_IV";
        //     }
        // };
        // resistor {
        //     name = "R1";
        //     n1 = "out";
        //     n2 = "vdd";
        //     value = "10k";
        // }
        // inductor {
        //     name = "L1";
        //     n1 = "out";
        //     n2 = "vdd";
        //     value = "10mH";
        // }
        // capacitor {
        //     name = "C1";
        //     n1 = "out";
        //     n2 = "vdd";
        //     value = "100nF";
        // }
    }
}
