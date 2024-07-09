use crate::circuit::elements::CircuitElement;
use crate::circuit::equations::CircuitEquation;
use crate::circuit::nodes::CircuitNode;
use evalexpr::HashMapContext;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct Transistor {
    name: CircuitElement,
    context: HashMapContext,
    equations: CircuitEquation,
    gate: CircuitNode,
    source: CircuitNode,
    drain: CircuitNode,
    body: CircuitNode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use evalexpr::context_map;
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
        let circuit_eq = CircuitEquation::from_str(eq).unwrap();
        let x1 = CircuitElement::from_str("x1").unwrap();
        let n1 = CircuitNode::from_str("n1").unwrap();
        let n2 = CircuitNode::from_str("n2").unwrap();
        let n3 = CircuitNode::from_str("n3").unwrap();
        let n4 = CircuitNode::from_str("n4").unwrap();
        let context = context_map! {
            "vd" => n1.to_string() + " - " + &n2.to_string(),
        }
        .unwrap();
        let _transistor = Transistor::builder()
            .name(x1)
            .context(context)
            .equations(circuit_eq)
            .gate(n1)
            .source(n2)
            .drain(n3)
            .body(n4)
            .build();
    }
}
