use crate::circuit::equations::CircuitEquation;
use crate::circuit::nodes::CircuitNode;
use evalexpr::HashMapContext;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct Transistor {
    context: HashMapContext,
    equations: CircuitEquation,
    gate: CircuitNode,
    source: CircuitNode,
    drain: CircuitNode,
    body: CircuitNode,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use evalexpr::context_map;

    use super::*;

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
        let context = context_map! {
            "five" => 5,
            "twelve" => 12,
        }
        .unwrap();
        let n1 = CircuitNode::from_str("n1").unwrap();
        let n2 = CircuitNode::from_str("n2").unwrap();
        let n3 = CircuitNode::from_str("n3").unwrap();
        let n4 = CircuitNode::from_str("n4").unwrap();
        let _transistor = Transistor::builder()
            .context(context)
            .equations(circuit_eq)
            .gate(n1)
            .source(n2)
            .drain(n3)
            .body(n4);
    }
}
