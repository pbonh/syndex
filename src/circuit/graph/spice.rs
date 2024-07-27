use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;

use itertools::Itertools;
use peginator::{ParseError, PegParser};

use super::LCircuitNodeID;
#[allow(unused_imports)]
use crate::circuit::spice::{
    Capacitor, CurrentSource, Diode, Element, Inductor, Instance, MosTransistor, Node as SPICENode,
    Resistor, SPICENetlist, VoltageSource,
};

#[derive(Debug, Clone, Default)]
pub(super) struct SPICENodeMap(HashMap<SPICENode, LCircuitNodeID>);

impl SPICENodeMap {
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }

    fn resistor_nodes(resistor: &Resistor)  -> Vec<SPICENode> {
        let resistor_nodes = vec![resistor.p.to_owned(), resistor.n.to_owned()];
        resistor_nodes.iter().map(|spice_string| SPICENode::from(spice_string)).collect_vec()
    }

    fn mos_transistor_nodes(mos_transistor: &MosTransistor)  -> Vec<SPICENode> {
        let mos_transistor_nodes = vec![mos_transistor.source.to_owned(), mos_transistor.drain.to_owned(), mos_transistor.gate.to_owned(), mos_transistor.body.to_owned()];
        mos_transistor_nodes.iter().map(|spice_string| SPICENode::from(spice_string)).collect_vec()
    }

    fn instance_nodes(instance: &Instance)  -> Vec<SPICENode> {
        let instance_nodes = vec![instance.source.to_owned(), instance.drain.to_owned(), instance.gate.to_owned(), instance.body.to_owned()];
        instance_nodes.iter().map(|spice_string| SPICENode::from(spice_string)).collect_vec()
    }

    fn collect_netlist_nodes(netlist: &SPICENetlist) -> Vec<SPICENode> {
        let mut nodes: Vec<SPICENode> = vec![];
        netlist
            .netlist_scope
            .elements
            .iter()
            .for_each(|element: &Element| {
                if let Some(mos_transistor) = &element.mostransistor {
                    nodes.extend(Self::mos_transistor_nodes(mos_transistor));
                }
                if let Some(resistor) = &element.resistor {
                    nodes.extend(Self::resistor_nodes(resistor));
                }
                if let Some(instance) = &element.subcircuit {
                    nodes.extend(Self::instance_nodes(instance));
                }
            });
            nodes
    }
}

impl FromStr for SPICENodeMap {
    type Err = ParseError;

    fn from_str(spice_netlist_str: &str) -> Result<Self, Self::Err> {
        match SPICENetlist::parse(spice_netlist_str) {
            Ok(ast) => {
                let mut idx = LCircuitNodeID::min_value();
                let mut node_map = HashMap::<SPICENode, LCircuitNodeID>::default();
                Self::collect_netlist_nodes(&ast).iter().for_each(|spice_node| {
                    node_map.insert(spice_node.to_owned(), idx);
                    idx += 1;
                });
                Ok(Self(node_map))
            }
            Err(error) => Err(error),
        }
    }
}

impl Index<SPICENode> for SPICENodeMap {
    type Output = LCircuitNodeID;

    fn index(&self, index: SPICENode) -> &Self::Output {
        &self.0[&index]
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn netlist_node_count() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push("resources/spice3f5_examples/mosamp2.cir");
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let netlist_nodes = SPICENodeMap::collect_netlist_nodes(&ast);
        assert_eq!(20, netlist_nodes.len());
    }

    #[test]
    fn map_of_nodes() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push("resources/spice3f5_examples/mosamp2.cir");
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let netlist_nodes = SPICENodeMap::from_str(&spice_netlist_str).unwrap();
        assert_eq!(20, netlist_nodes.len());
        assert_eq!(0, netlist_nodes["15".to_owned()]);
        assert_eq!(1, netlist_nodes["1".to_owned()]);
        assert_eq!(2, netlist_nodes["32".to_owned()]);
    }

    #[test]
    fn sky130_map_of_nodes() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.spice",
        );
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let netlist_nodes = SPICENodeMap::from_str(&spice_netlist_str).unwrap();
        assert_eq!(24, netlist_nodes.len());
        assert_eq!(0, netlist_nodes["A1".to_owned()]);
        assert_eq!(1, netlist_nodes["A2".to_owned()]);
        assert_eq!(2, netlist_nodes["B1".to_owned()]);
        assert_eq!(4, netlist_nodes["VGND".to_owned()]);
    }
}
