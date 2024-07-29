use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;
use peginator::{ParseError, PegParser};

use super::LCircuitNodeID;
use crate::circuit::elements::capacitor::Capacitor as LCapacitor;
use crate::circuit::elements::resistor::Resistor as LResistor;
use crate::circuit::elements::transistor::Transistor;
use crate::circuit::equations::{
    CircuitEquation, DeviceEquation, DeviceEquationMap, ModelName, VariableContext,
    VariableContextMap, CAPACITORMODELNAME, RESISTORMODELNAME,
};
use crate::circuit::nodes::CircuitNode;
use crate::circuit::spice::NetlistScope;
#[allow(unused_imports)]
use crate::circuit::spice::{
    Capacitor, CurrentSource, Diode, Element, Inductor, Instance, MosTransistor, Node as SPICENode,
    Resistor, SPICENetlist, VoltageSource,
};

fn resistor_nodes(resistor: &Resistor) -> Vec<SPICENode> {
    let resistor_nodes = vec![resistor.p.to_owned(), resistor.n.to_owned()];
    resistor_nodes
        .iter()
        .map(|spice_string| SPICENode::from(spice_string))
        .collect_vec()
}

fn capacitor_nodes(capacitor: &Capacitor) -> Vec<SPICENode> {
    let capacitor_nodes = vec![capacitor.p.to_owned(), capacitor.n.to_owned()];
    capacitor_nodes
        .iter()
        .map(|spice_string| SPICENode::from(spice_string))
        .collect_vec()
}

fn vsource_nodes(vsource: &VoltageSource) -> Vec<SPICENode> {
    let vsource_nodes = vec![vsource.p.to_owned(), vsource.n.to_owned()];
    vsource_nodes
        .iter()
        .map(|spice_string| SPICENode::from(spice_string))
        .collect_vec()
}

fn mos_transistor_nodes(mos_transistor: &MosTransistor) -> Vec<SPICENode> {
    let mos_transistor_nodes = vec![
        mos_transistor.source.to_owned(),
        mos_transistor.drain.to_owned(),
        mos_transistor.gate.to_owned(),
        mos_transistor.body.to_owned(),
    ];
    mos_transistor_nodes
        .iter()
        .map(|spice_string| SPICENode::from(spice_string))
        .collect_vec()
}

fn instance_nodes(instance: &Instance) -> Vec<SPICENode> {
    let instance_nodes = vec![
        instance.source.to_owned(),
        instance.drain.to_owned(),
        instance.gate.to_owned(),
        instance.body.to_owned(),
    ];
    instance_nodes
        .iter()
        .map(|spice_string| SPICENode::from(spice_string))
        .collect_vec()
}

fn netlist_scope_element_iter(netlist: &SPICENetlist) -> impl Iterator<Item = &Element> + '_ {
    let subcircuits = &netlist.netlist_scope.subcircuits;
    let top_scope = &netlist.netlist_scope;
    top_scope.elements.iter().chain(
        subcircuits
            .iter()
            .flat_map(|subcircuit_scope| subcircuit_scope.netlist_scope.elements.iter()),
    )
}

fn get_element_model_name(element: &Element) -> ModelName {
    let model = match element {
        Element {
            subcircuit: Some(subcircuit),
            ..
        } => subcircuit.model.clone(),
        Element {
            mostransistor: Some(mostransistor),
            ..
        } => mostransistor.model.clone(),
        Element {
            resistor: Some(_resistor),
            ..
        } => ModelName::from(RESISTORMODELNAME),
        Element {
            capacitor: Some(_capacitor),
            ..
        } => ModelName::from(CAPACITORMODELNAME),
        _ => ModelName::default(),
    };
    model
}

fn get_element_nodes_eqs(element: &Element) -> Vec<VariableContext> {
    match element {
        Element {
            subcircuit: Some(subcircuit),
            ..
        } => Transistor::transistor_variable_ctx(
            &CircuitNode::from_str(&subcircuit.source).expect("Invalid CircuitNode Name."),
            &CircuitNode::from_str(&subcircuit.drain).expect("Invalid CircuitNode Name."),
            &CircuitNode::from_str(&subcircuit.gate).expect("Invalid CircuitNode Name."),
            &CircuitNode::from_str(&subcircuit.body).expect("Invalid CircuitNode Name."),
        ),
        Element {
            mostransistor: Some(mostransistor),
            ..
        } => Transistor::transistor_variable_ctx(
            &CircuitNode::from_str(&mostransistor.source).expect("Invalid CircuitNode Name."),
            &CircuitNode::from_str(&mostransistor.drain).expect("Invalid CircuitNode Name."),
            &CircuitNode::from_str(&mostransistor.gate).expect("Invalid CircuitNode Name."),
            &CircuitNode::from_str(&mostransistor.body).expect("Invalid CircuitNode Name."),
        ),
        Element {
            resistor: Some(resistor),
            ..
        } => LResistor::resistor_variable_ctx(
            &CircuitNode::from_str(&resistor.p).expect("Invalid CircuitNode Name."),
            &CircuitNode::from_str(&resistor.n).expect("Invalid CircuitNode Name."),
        ),
        Element {
            capacitor: Some(capacitor),
            ..
        } => LCapacitor::capacitor_variable_ctx(
            &CircuitNode::from_str(&capacitor.p).expect("Invalid CircuitNode Name."),
            &CircuitNode::from_str(&capacitor.n).expect("Invalid CircuitNode Name."),
        ),
        _ => Vec::<VariableContext>::default(),
    }
}

fn get_element_circuit_equation(
    element: &Element,
    dev_eq_map: &DeviceEquationMap,
) -> CircuitEquation {
    let model = get_element_model_name(element);
    let dev_eq: DeviceEquation = dev_eq_map[&model].clone();

    let ctx: VariableContextMap = get_element_nodes_eqs(element).into_iter().collect();
    CircuitEquation::new(dev_eq, &ctx)
}

pub(super) type SPICENodeMap = HashMap<SPICENode, LCircuitNodeID>;

#[derive(Debug, Clone, Default)]
pub(super) struct SPICENodeSet(HashSet<SPICENode>);

impl SPICENodeSet {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn contains(&self, node: &SPICENode) -> bool {
        self.0.contains(node)
    }

    fn collect_netlist_nodes(netlist: &SPICENetlist) -> HashSet<SPICENode> {
        let mut netlist_nodes: HashSet<SPICENode> = HashSet::default();
        let subcircuits = &netlist.netlist_scope.subcircuits;
        let top_scope = &netlist.netlist_scope;
        let netlist_scope_iter = |nodes: &mut HashSet<String>, netlist_scope: &NetlistScope| {
            netlist_scope.elements.iter().for_each(|element: &Element| {
                if let Some(mos_transistor) = &element.mostransistor {
                    nodes.extend(mos_transistor_nodes(mos_transistor));
                }
                if let Some(resistor) = &element.resistor {
                    nodes.extend(resistor_nodes(resistor));
                }
                if let Some(capacitor) = &element.capacitor {
                    nodes.extend(capacitor_nodes(capacitor));
                }
                if let Some(instance) = &element.subcircuit {
                    nodes.extend(instance_nodes(instance));
                }
                if let Some(vsource) = &element.voltagesource {
                    nodes.extend(vsource_nodes(vsource));
                }
            });
        };
        netlist_scope_iter(&mut netlist_nodes, top_scope);
        subcircuits.iter().for_each(|subcircuit_scope| {
            netlist_scope_iter(&mut netlist_nodes, &subcircuit_scope.netlist_scope);
        });
        netlist_nodes
    }
}

impl FromStr for SPICENodeSet {
    type Err = ParseError;

    fn from_str(spice_netlist_str: &str) -> Result<Self, Self::Err> {
        match SPICENetlist::parse(spice_netlist_str) {
            Ok(ast) => {
                let mut node_map = HashSet::<SPICENode>::default();
                Self::collect_netlist_nodes(&ast)
                    .iter()
                    .for_each(|spice_node| {
                        node_map.insert(spice_node.to_owned());
                    });
                Ok(Self(node_map))
            }
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct SPICEElements(Vec<Element>);

impl SPICEElements {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn collect_netlist_elements(netlist: &SPICENetlist) -> Self {
        Self(
            netlist
                .netlist_scope
                .elements
                .iter()
                .map(|element: &Element| element.to_owned())
                .collect_vec(),
        )
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
        let netlist_nodes = SPICENodeSet::collect_netlist_nodes(&ast);
        assert_eq!(21, netlist_nodes.len());
    }

    #[test]
    fn sky130_netlist_node_count() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.spice",
        );
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let netlist_nodes = SPICENodeSet::collect_netlist_nodes(&ast);
        assert_eq!(13, netlist_nodes.len());
    }

    #[test]
    fn map_of_nodes() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push("resources/spice3f5_examples/mosamp2.cir");
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let netlist_nodes = SPICENodeSet::from_str(&spice_netlist_str).unwrap();
        assert_eq!(21, netlist_nodes.len());
        assert!(netlist_nodes.contains(&"15".to_owned()));
        assert!(netlist_nodes.contains(&"1".to_owned()));
        assert!(netlist_nodes.contains(&"32".to_owned()));
    }

    #[test]
    fn sky130_map_of_nodes() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.spice",
        );
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let netlist_nodes = SPICENodeSet::from_str(&spice_netlist_str).unwrap();
        assert_eq!(13, netlist_nodes.len());
        assert!(netlist_nodes.contains(&"A1".to_owned()));
        assert!(netlist_nodes.contains(&"A2".to_owned()));
        assert!(netlist_nodes.contains(&"B1".to_owned()));
        assert!(netlist_nodes.contains(&"VGND".to_owned()));
    }

    #[test]
    fn netlist_element_count() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push("resources/spice3f5_examples/mosamp2.cir");
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let netlist_elements = SPICEElements::collect_netlist_elements(&ast);
        assert_eq!(33, netlist_elements.len());
    }

    #[test]
    fn sky130_netlist_element_count() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.spice",
        );
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let netlist_elements = netlist_scope_element_iter(&ast).collect_vec();
        assert_eq!(12, netlist_elements.len());
    }

    #[test]
    fn netlist_element_data() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push("resources/spice3f5_examples/mosamp2.cir");
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let netlist_elements = netlist_scope_element_iter(&ast).collect_vec();
        assert_eq!(33, netlist_elements.len());
        let m1_instance = netlist_elements[0];

        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vgs/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();
        let device_eq_map = DeviceEquationMap::from([("m".to_owned(), dev_eq)]);

        let m1_dev_eq = get_element_circuit_equation(m1_instance, &device_eq_map);
        let eq_expected_str = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^((1-15)/(eta*Vt)) - 1)
        "};
        let eq_expected = CircuitEquation::from_str(eq_expected_str).unwrap();
        assert_eq!(eq_expected, m1_dev_eq);
    }

    #[test]
    fn sky130_netlist_element_data() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.spice",
        );
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let netlist_elements = netlist_scope_element_iter(&ast).collect_vec();
        assert_eq!(12, netlist_elements.len());
        let x0_instance = netlist_elements[0];

        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vgs/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();
        let device_eq_map =
            DeviceEquationMap::from([("sky130_fd_pr__nfet_01v8".to_owned(), dev_eq)]);

        let x0_dev_eq = get_element_circuit_equation(x0_instance, &device_eq_map);
        let eq_expected_str = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^((a_399_74#-VGND)/(eta*Vt)) - 1)
        "};
        let eq_expected = CircuitEquation::from_str(eq_expected_str).unwrap();
        assert_eq!(eq_expected, x0_dev_eq);
    }
}
