use crate::circuit::elements::CircuitElement;
use crate::circuit::equations::CircuitEquation;
use crate::circuit::nodes::CircuitNode;
use derive_getters::Getters;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default, TypedBuilder, Getters)]
pub struct Transistor {
    name: CircuitElement,
    equations: CircuitEquation,
    gate: CircuitNode,
    source: CircuitNode,
    drain: CircuitNode,
    body: CircuitNode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::equations::*;
    use std::str::FromStr;

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
        let ctx = VariableContextMap::from([("vd".to_string(), node_eq)]);
        let circuit_eq = CircuitEquation::new(dev_eq, ctx);
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
}
