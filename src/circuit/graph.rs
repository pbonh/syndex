use std::fmt::{Display, Formatter, Result};
use std::ops::{Deref, DerefMut};

use bevy_ecs::prelude::Resource;
use mhgl::HGraph;

use crate::circuit::equations::CircuitEquation;
use crate::circuit::nodes::CircuitNode;

pub type VertexIndex = u32;
pub type HyperedgeIndex = u64;

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

pub type LCircuitNodeID = u32;
pub type LCircuitEdgeID = u64;

#[derive(Debug, Resource)]
pub struct LCircuit(HGraph<VoltageNode, CircuitHyperEdge, LCircuitNodeID, LCircuitEdgeID>);

impl Default for LCircuit {
    fn default() -> Self {
        Self(HGraph::<
            VoltageNode,
            CircuitHyperEdge,
            LCircuitNodeID,
            LCircuitEdgeID,
        >::new())
    }
}

impl Deref for LCircuit {
    type Target = HGraph<VoltageNode, CircuitHyperEdge, LCircuitNodeID, LCircuitEdgeID>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LCircuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// impl AsRef<HGraph<VoltageNode, CircuitHyperEdge, LCircuitNodeID, LCircuitEdgeID>> for LCircuit
// where
//     <Self as Deref>::Target: AsRef<HGraph<VoltageNode, CircuitHyperEdge, LCircuitNodeID, LCircuitEdgeID>>,
// {
//     fn as_ref(&self) -> &HGraph<VoltageNode, CircuitHyperEdge, LCircuitNodeID, LCircuitEdgeID> {
//         self.deref().as_ref()
//     }
// }
//
// impl AsMut<HGraph<VoltageNode, CircuitHyperEdge, LCircuitNodeID, LCircuitEdgeID>> for LCircuit
// where
//     <Self as Deref>::Target: AsMut<HGraph<VoltageNode, CircuitHyperEdge, LCircuitNodeID, LCircuitEdgeID>>,
// {
//     fn as_mut(&mut self) -> &mut HGraph<VoltageNode, CircuitHyperEdge, LCircuitNodeID, LCircuitEdgeID> {
//         self.deref_mut().as_mut()
//     }
// }

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    // use crate::circuit::elements::*;
    use crate::circuit::equations::*;

    #[test]
    fn default_circuit() {
        let _circuit = LCircuit::default();
    }

    #[test]
    fn simple_inverter_macro() {
        let mut circuit = LCircuit::default();
        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vd/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();
        let input = CircuitNode::from_str("input").unwrap();
        let out = CircuitNode::from_str("out").unwrap();
        let ground = CircuitNode::from_str("ground").unwrap();
        let vsupply = CircuitNode::from_str("vsupply").unwrap();

        let input_voltage = VoltageNode::new(input.clone());
        let out_voltage = VoltageNode::new(out.clone());
        let ground_voltage = VoltageNode::new(ground.clone());
        let vsupply_voltage = VoltageNode::new(vsupply.clone());
        let input_id = circuit.add_node(input_voltage);
        let out_id = circuit.add_node(out_voltage);
        let ground_id = circuit.add_node(ground_voltage);
        let vsupply_id = circuit.add_node(vsupply_voltage);
        // assert_eq!(circuit.count_vertices(), 4);

        // let x1 = CircuitElement::from_str("x1").unwrap();
        let x1_nmos_node_eq_str: String =
            "(".to_owned() + &input.to_string() + " - " + &out.to_string() + ")";
        let x1_nmos_node_eq = DeviceEquation::from_str(&x1_nmos_node_eq_str).unwrap();
        let x1_nmos_ctx =
            VariableContextMap::from([(CircuitNode::from_str("vd").unwrap(), x1_nmos_node_eq)]);
        let x1_nmos_transistor_eq = CircuitEquation::new(dev_eq.clone(), &x1_nmos_ctx);
        let x2_nmos_node_eq_str: String =
            "(".to_owned() + &input.to_string() + " - " + &out.to_string() + ")";
        let x2_nmos_node_eq = DeviceEquation::from_str(&x2_nmos_node_eq_str).unwrap();
        let x2_nmos_ctx =
            VariableContextMap::from([(CircuitNode::from_str("vd").unwrap(), x2_nmos_node_eq)]);
        let x2_nmos_transistor_eq = CircuitEquation::new(dev_eq, &x2_nmos_ctx);

        let x1_nmos_transistor_hyperedge = CircuitHyperEdge::new(x1_nmos_transistor_eq);
        let x2_nmos_transistor_hyperedge = CircuitHyperEdge::new(x2_nmos_transistor_eq);
        let x1_nmos_transistor_id = circuit
            .add_edge(
                vec![input_id, out_id, ground_id, ground_id],
                x1_nmos_transistor_hyperedge,
            )
            .unwrap();
        let x2_nmos_transistor_id = circuit
            .add_edge(
                vec![input_id, vsupply_id, out_id, vsupply_id],
                x2_nmos_transistor_hyperedge,
            )
            .unwrap();
        assert_eq!(x1_nmos_transistor_id, 0);
        assert_eq!(x2_nmos_transistor_id, 1);
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
