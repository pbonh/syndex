use std::str::FromStr;

use derive_getters::Getters;
use typed_builder::TypedBuilder;

use crate::circuit::elements::CircuitElement;
use crate::circuit::equations::{
    CircuitEquation, DeviceEquation, DeviceEquationMap, VariableContextMap,
};
use crate::circuit::nodes::CircuitNode;
use crate::circuit::spice::MosTransistor;

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
        let vgs_str = gate.to_string() + "-" + &source.to_string();
        let vgd_str = gate.to_string() + "-" + &drain.to_string();
        let vgb_str = gate.to_string() + "-" + &body.to_string();
        let vds_str = drain.to_string() + "-" + &source.to_string();
        let vdb_str = drain.to_string() + "-" + &body.to_string();
        let vsb_str = source.to_string() + "-" + &body.to_string();
        let vgs = DeviceEquation::from_str(&vgs_str).expect("Invalid resulting equation");
        let vgd = DeviceEquation::from_str(&vgd_str).expect("Invalid resulting equation");
        let vgb = DeviceEquation::from_str(&vgb_str).expect("Invalid resulting equation");
        let vds = DeviceEquation::from_str(&vds_str).expect("Invalid resulting equation");
        let vdb = DeviceEquation::from_str(&vdb_str).expect("Invalid resulting equation");
        let vsb = DeviceEquation::from_str(&vsb_str).expect("Invalid resulting equation");
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
        assert_eq!("n3-n1", vgs.to_string());
        assert_eq!("n3-n2", vgd.to_string());
        assert_eq!("n3-n4", vgb.to_string());
        assert_eq!("n2-n1", vds.to_string());
        assert_eq!("n2-n4", vdb.to_string());
        assert_eq!("n1-n4", vsb.to_string());
    }

    #[test]
    fn sky130_from_device_equation_map() {
        let source = CircuitNode::from_str("v_source")
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let drain = CircuitNode::from_str("v_drain")
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let gate = CircuitNode::from_str("v_gate")
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let body = CircuitNode::from_str("v_body")
            .expect("Failure to convert `SPICENetlist` `Node` to `CircuitNode`");
        let (vgs, vgd, vgb, vds, vdb, vsb) =
            Transistor::transistor_node_subst(&source, &drain, &gate, &body);
        assert_eq!("v_gate-v_source", vgs.to_string());
        assert_eq!("v_gate-v_drain", vgd.to_string());
        assert_eq!("v_gate-v_body", vgb.to_string());
        assert_eq!("v_drain-v_source", vds.to_string());
        assert_eq!("v_drain-v_body", vdb.to_string());
        assert_eq!("v_source-v_body", vsb.to_string());

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
            I = Is*(e^(5-15/(eta*Vt)) - 1)
        "};
        assert_eq!(eq_expected, xtor.equations.to_string());
    }
}
