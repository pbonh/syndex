use std::str::FromStr;

use derive_getters::Getters;
use typed_builder::TypedBuilder;

use crate::circuit::elements::CircuitElement;
use crate::circuit::equations::{
    CircuitEquation, DeviceEquation, DeviceEquationMap, VariableContextMap,
};
use crate::circuit::nodes::CircuitNode;
use crate::circuit::spice::{Instance, MosTransistor};

#[derive(Debug, Clone, Default, TypedBuilder, Getters)]
pub struct Transistor {
    name: CircuitElement,
    equations: CircuitEquation,
    source: CircuitNode,
    drain: CircuitNode,
    gate: CircuitNode,
    body: CircuitNode,
}

impl Transistor {
    fn transistor_node_subst(
        source: &CircuitNode,
        drain: &CircuitNode,
        gate: &CircuitNode,
        body: &CircuitNode,
    ) -> (
        DeviceEquation,
        DeviceEquation,
        DeviceEquation,
        DeviceEquation,
        DeviceEquation,
        DeviceEquation,
    ) {
        let build_node_difference_str = |node1: &str, node2: &str| {
            let mut node_difference_str = "(".to_owned();
            node_difference_str.push_str(node1);
            node_difference_str.push('-');
            node_difference_str.push_str(node2);
            node_difference_str.push(')');
            DeviceEquation::from_str(&node_difference_str).expect("Invalid resulting equation")
        };

        let vgs = build_node_difference_str(&gate.to_string(), &source.to_string());
        let vgd = build_node_difference_str(&gate.to_string(), &drain.to_string());
        let vgb = build_node_difference_str(&gate.to_string(), &body.to_string());
        let vds = build_node_difference_str(&drain.to_string(), &source.to_string());
        let vdb = build_node_difference_str(&drain.to_string(), &body.to_string());
        let vsb = build_node_difference_str(&source.to_string(), &body.to_string());
        (vgs, vgd, vgb, vds, vdb, vsb)
    }
}

impl From<(MosTransistor, DeviceEquationMap)> for Transistor {
    fn from(tech: (MosTransistor, DeviceEquationMap)) -> Self {
        let ast = tech.0;
        let id = CircuitElement(ast.id);
        let source = CircuitNode::from_str(&ast.source)
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let drain = CircuitNode::from_str(&ast.drain)
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let gate = CircuitNode::from_str(&ast.gate)
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let body = CircuitNode::from_str(&ast.body)
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let model = ast.model;
        let device_equation = tech.1[&model].clone();
        let (vgs, vgd, vgb, vds, vdb, vsb) = (
            CircuitNode::from_str("vgs").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vgd").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vgb").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vds").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vdb").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vsb").expect("Invalid CircuitNode Name."),
        );
        let (vgs_eq, vgd_eq, vgb_eq, vds_eq, vdb_eq, vsb_eq) =
            Self::transistor_node_subst(&source, &drain, &gate, &body);
        let ctx = VariableContextMap::from([
            (vgs, vgs_eq),
            (vgd, vgd_eq),
            (vgb, vgb_eq),
            (vds, vds_eq),
            (vdb, vdb_eq),
            (vsb, vsb_eq),
        ]);
        let equations = CircuitEquation::new(device_equation, &ctx);
        Self {
            name: id,
            source,
            drain,
            gate,
            body,
            equations,
        }
    }
}

impl From<(Instance, DeviceEquationMap)> for Transistor {
    fn from(tech: (Instance, DeviceEquationMap)) -> Self {
        let ast = tech.0;
        let id = CircuitElement(ast.id);
        let source = CircuitNode::from_str(&ast.source)
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let drain = CircuitNode::from_str(&ast.drain)
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let gate = CircuitNode::from_str(&ast.gate)
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let body = CircuitNode::from_str(&ast.body)
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let model = ast.model;
        let device_equation = tech.1[&model].clone();
        let (vgs, vgd, vgb, vds, vdb, vsb) = (
            CircuitNode::from_str("vgs").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vgd").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vgb").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vds").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vdb").expect("Invalid CircuitNode Name."),
            CircuitNode::from_str("vsb").expect("Invalid CircuitNode Name."),
        );
        let (vgs_eq, vgd_eq, vgb_eq, vds_eq, vdb_eq, vsb_eq) =
            Self::transistor_node_subst(&source, &drain, &gate, &body);
        let ctx = VariableContextMap::from([
            (vgs, vgs_eq),
            (vgd, vgd_eq),
            (vgb, vgb_eq),
            (vds, vds_eq),
            (vdb, vdb_eq),
            (vsb, vsb_eq),
        ]);
        let equations = CircuitEquation::new(device_equation, &ctx);
        Self {
            name: id,
            source,
            drain,
            gate,
            body,
            equations,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    use peginator::PegParser;

    use super::*;
    use crate::circuit::equations::*;
    use crate::circuit::spice::SPICENetlist;

    #[test]
    fn build_transistor_simple() {
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
        let ctx = VariableContextMap::from([(CircuitNode::from_str("vd").unwrap(), node_eq)]);
        let circuit_eq = CircuitEquation::new(dev_eq, &ctx);
        let transistor = Transistor::builder()
            .name(x1)
            .equations(circuit_eq)
            .gate(n1)
            .source(n2)
            .drain(n3)
            .body(n4)
            .build();
        let transistor_eq = transistor.equations().to_string();
        let eval_eq = indoc::indoc! {"
            I = Is*(e^((n1 - n2)/(eta*Vt)) - 1)
        "};
        assert!(
            transistor_eq.to_string().contains(eval_eq),
            "Evaluated equation is incorrect."
        );
    }

    #[test]
    fn substitute_transistor_node_names() {
        let source = CircuitNode::from_str("n1")
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let drain = CircuitNode::from_str("n2")
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let gate = CircuitNode::from_str("n3")
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let body = CircuitNode::from_str("n4")
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let (vgs, vgd, vgb, vds, vdb, vsb) =
            Transistor::transistor_node_subst(&source, &drain, &gate, &body);
        assert_eq!("(n3-n1)", vgs.to_string());
        assert_eq!("(n3-n2)", vgd.to_string());
        assert_eq!("(n3-n4)", vgb.to_string());
        assert_eq!("(n2-n1)", vds.to_string());
        assert_eq!("(n2-n4)", vdb.to_string());
        assert_eq!("(n1-n4)", vsb.to_string());
    }

    #[test]
    fn spice_netlist_from_device_equation_map() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push("resources/spice3f5_examples/mosamp2.cir");
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let m4_xtor = ast.netlist_scope.elements[3].clone().mostransistor.unwrap();

        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vds/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();

        let device_eq_map = DeviceEquationMap::from([(m4_xtor.model.clone(), dev_eq)]);
        let xtor = Transistor::from((m4_xtor, device_eq_map));
        let eq_expected = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^((5-15)/(eta*Vt)) - 1)
        "};
        assert_eq!(eq_expected, xtor.equations.to_string());
    }

    #[test]
    fn sky130_from_device_equation_map() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.spice",
        );
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let ast = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let netlist_scope = &ast.netlist_scope;
        let subcircuit_scope = &netlist_scope.subcircuits[0].netlist_scope;
        let x4_xtor = subcircuit_scope.elements[3].clone().subcircuit.unwrap();

        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vds/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();

        let device_eq_map = DeviceEquationMap::from([(x4_xtor.model.clone(), dev_eq)]);
        let xtor = Transistor::from((x4_xtor, device_eq_map));
        let eq_expected = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^((A2-a_317_392#)/(eta*Vt)) - 1)
        "};
        assert_eq!(eq_expected, xtor.equations.to_string());
    }
}
