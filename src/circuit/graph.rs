pub(super) mod edges;
pub(super) mod nodes;
pub(super) mod spice;

use std::ops::{Deref, DerefMut};

use bevy_ecs::prelude::Resource;
use mhgl::HGraph;

use super::equations::DeviceEquationMap;
use super::spice::SPICENetlist;
use crate::circuit::graph::edges::ElementHNode;
use crate::circuit::graph::nodes::VoltageHEdge;

pub type LCircuitNodeID = u32;
pub type LCircuitEdgeID = u64;

type LHGraph = HGraph<VoltageHEdge, ElementHNode, LCircuitNodeID, LCircuitEdgeID>;

#[derive(Debug, Clone, Resource)]
pub struct LCircuit(LHGraph);

impl From<(&SPICENetlist, &DeviceEquationMap)> for LCircuit {
    fn from(spice_netlist_and_map: (&SPICENetlist, &DeviceEquationMap)) -> Self {
        let _spice_netlist = spice_netlist_and_map.0;
        let _device_equation_map = spice_netlist_and_map.1;
        todo!()
    }
}

impl Default for LCircuit {
    fn default() -> Self {
        Self(HGraph::<
            VoltageHEdge,
            ElementHNode,
            LCircuitNodeID,
            LCircuitEdgeID,
        >::new())
    }
}

impl Deref for LCircuit {
    type Target = LHGraph;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LCircuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// impl AsRef<LHGraph> for LCircuit
// where
//     <Self as Deref>::Target: AsRef<LHGraph>,
// {
//     fn as_ref(&self) -> &LHGraph {
//         self.deref().as_ref()
//     }
// }
//
// impl AsMut<LHGraph> for LCircuit
// where
//     <Self as Deref>::Target: AsMut<LHGraph>,
// {
//     fn as_mut(&mut self) -> &mut LHGraph {
//         self.deref_mut().as_mut()
//     }
// }

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    use peginator::PegParser;

    use super::*;
    // use crate::circuit::elements::*;
    use crate::circuit::equations::*;
    use crate::circuit::nodes::CircuitNode;

    #[test]
    fn default_circuit() {
        let _circuit = LCircuit::default();
    }

    #[test]
    fn simple_inverter_manual_build() {
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

        let input_voltage = VoltageHEdge::new(input.clone());
        let out_voltage = VoltageHEdge::new(out.clone());
        let ground_voltage = VoltageHEdge::new(ground.clone());
        let vsupply_voltage = VoltageHEdge::new(vsupply.clone());
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

        let x1_nmos_transistor_hyperedge = ElementHNode::new(x1_nmos_transistor_eq);
        let x2_nmos_transistor_hyperedge = ElementHNode::new(x2_nmos_transistor_eq);
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

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn from_spice_netlist() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push("resources/spice3f5_examples/mosamp2.cir");
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();

        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vds/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();
        let device_eq_map = DeviceEquationMap::from([("m".to_string(), dev_eq)]);

        // let num_elements: usize = 33;
        // let graph_element_indices = (0..32).collect_vec();
        let _graph = LCircuit::from((&ast, &device_eq_map));
    }
}
