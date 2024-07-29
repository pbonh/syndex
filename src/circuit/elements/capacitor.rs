use std::str::FromStr;

use derive_getters::Getters;
use typed_builder::TypedBuilder;

use crate::circuit::elements::CircuitElement;
use crate::circuit::equations::{CircuitEquation, DeviceEquation, VariableContext};
use crate::circuit::nodes::CircuitNode;

#[derive(Debug, Clone, Default, TypedBuilder, Getters)]
pub struct Capacitor {
    name: CircuitElement,
    equations: CircuitEquation,
    p_node: CircuitNode,
    n_node: CircuitNode,
}

impl Capacitor {
    fn capacitor_node_subst(p_node: &CircuitNode, n_node: &CircuitNode) -> DeviceEquation {
        let build_node_difference_str = |node1: &str, node2: &str| {
            let mut node_difference_str = "(".to_owned();
            node_difference_str.push_str(node1);
            node_difference_str.push('-');
            node_difference_str.push_str(node2);
            node_difference_str.push(')');
            DeviceEquation::from_str(&node_difference_str).expect("Invalid resulting equation")
        };

        let vpn = build_node_difference_str(&p_node.to_string(), &n_node.to_string());
        vpn
    }
    pub(crate) fn capacitor_variable_ctx(
        p_node: &CircuitNode,
        n_node: &CircuitNode,
    ) -> Vec<VariableContext> {
        let vpn = CircuitNode::from_str("vpn").expect("Invalid CircuitNode Name.");
        let vpn_eq = Self::capacitor_node_subst(&p_node, &n_node);
        vec![(vpn, vpn_eq)]
    }
}
