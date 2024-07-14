use crate::circuit::equations::CircuitEquation;
use crate::circuit::nodes::CircuitNode;
use bevy_ecs::prelude::Entity;
use bevy_ecs::prelude::Resource;
use hypergraph::Hypergraph;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Deref, DerefMut};

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
pub struct CircuitHyperEdge<'eq_str> {
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

impl<'node_str, 'eq_str> Deref for LCircuit<'node_str, 'eq_str> {
    type Target = Hypergraph<VoltageNode<'node_str>, CircuitHyperEdge<'eq_str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'node_str, 'eq_str> DerefMut for LCircuit<'node_str, 'eq_str> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'node_str, 'eq_str> AsRef<Hypergraph<VoltageNode<'node_str>, CircuitHyperEdge<'eq_str>>>
    for LCircuit<'node_str, 'eq_str>
where
    <Self as Deref>::Target: AsRef<Hypergraph<VoltageNode<'node_str>, CircuitHyperEdge<'eq_str>>>,
{
    fn as_ref(&self) -> &Hypergraph<VoltageNode<'node_str>, CircuitHyperEdge<'eq_str>> {
        self.deref().as_ref()
    }
}

impl<'node_str, 'eq_str> AsMut<Hypergraph<VoltageNode<'node_str>, CircuitHyperEdge<'eq_str>>>
    for LCircuit<'node_str, 'eq_str>
where
    <Self as Deref>::Target: AsMut<Hypergraph<VoltageNode<'node_str>, CircuitHyperEdge<'eq_str>>>,
{
    fn as_mut(&mut self) -> &mut Hypergraph<VoltageNode<'node_str>, CircuitHyperEdge<'eq_str>> {
        self.deref_mut().as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::circuit::elements::*;
    use crate::circuit::equations::*;
    use hypergraph::HyperedgeIndex;
    use std::str::FromStr;

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

        let input_voltage = VoltageNode::new(&input);
        let out_voltage = VoltageNode::new(&out);
        let ground_voltage = VoltageNode::new(&ground);
        let vsupply_voltage = VoltageNode::new(&vsupply);
        let input_id = circuit.add_vertex(input_voltage).unwrap();
        let out_id = circuit.add_vertex(out_voltage).unwrap();
        let ground_id = circuit.add_vertex(ground_voltage).unwrap();
        let vsupply_id = circuit.add_vertex(vsupply_voltage).unwrap();
        assert_eq!(circuit.count_vertices(), 4);

        // let x1 = CircuitElement::from_str("x1").unwrap();
        let x1_nmos_node_eq_str: String =
            "(".to_owned() + &input.to_string() + " - " + &out.to_string() + ")";
        let x1_nmos_node_eq = DeviceEquation::from_str(&x1_nmos_node_eq_str).unwrap();
        let x1_nmos_ctx = VariableContextMap::from([("vd".to_string(), x1_nmos_node_eq)]);
        let x1_nmos_transistor_eq = CircuitEquation::new(dev_eq.clone(), x1_nmos_ctx);
        let x2_nmos_node_eq_str: String =
            "(".to_owned() + &input.to_string() + " - " + &out.to_string() + ")";
        let x2_nmos_node_eq = DeviceEquation::from_str(&x2_nmos_node_eq_str).unwrap();
        let x2_nmos_ctx = VariableContextMap::from([("vd".to_string(), x2_nmos_node_eq)]);
        let x2_nmos_transistor_eq = CircuitEquation::new(dev_eq, x2_nmos_ctx);

        let x1_nmos_transistor_entity_id = Entity::from_raw(0);
        let x2_nmos_transistor_entity_id = Entity::from_raw(1);
        let x1_nmos_transistor_hyperedge =
            CircuitHyperEdge::new(&x1_nmos_transistor_eq, x1_nmos_transistor_entity_id);
        let x2_nmos_transistor_hyperedge =
            CircuitHyperEdge::new(&x2_nmos_transistor_eq, x2_nmos_transistor_entity_id);
        let x1_nmos_transistor_id = circuit
            .add_hyperedge(
                vec![input_id, out_id, ground_id, ground_id],
                x1_nmos_transistor_hyperedge,
            )
            .unwrap();
        let x2_nmos_transistor_id = circuit
            .add_hyperedge(
                vec![input_id, vsupply_id, out_id, vsupply_id],
                x2_nmos_transistor_hyperedge,
            )
            .unwrap();
        assert_eq!(x1_nmos_transistor_id, HyperedgeIndex(0));
        assert_eq!(x2_nmos_transistor_id, HyperedgeIndex(1));
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
