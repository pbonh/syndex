use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use evalexpr::{build_operator_tree, EvalexprError};

use super::nodes::CircuitNode;

pub type ModelName = String;
pub type VariableContextMap = HashMap<CircuitNode, DeviceEquation>;
pub type DeviceEquationMap = HashMap<ModelName, DeviceEquation>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct DeviceEquation(String);

impl FromStr for DeviceEquation {
    type Err = EvalexprError;

    fn from_str(eq_str: &str) -> Result<Self, Self::Err> {
        match build_operator_tree(eq_str) {
            Ok(_) => Ok(Self(eq_str.to_owned())),
            Err(error) => Err(error),
        }
    }
}

impl fmt::Display for DeviceEquation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CircuitEquation(String);

impl CircuitEquation {
    pub fn new(dev_eq: DeviceEquation, ctx: &VariableContextMap) -> Self {
        let mut cir_eq = dev_eq.to_string();
        ctx.iter().for_each(|(var, eq)| {
            cir_eq = cir_eq.replace(&var.to_string(), &eq.to_string());
        });
        Self(cir_eq)
    }
}

// impl FromStr for CircuitEquation {
//     type Err = String;
//
//     fn from_str(eq_str: &str) -> Result<Self, Self::Err> {
//         match build_operator_tree(eq_str) {
//             Ok(_) => Ok(Self(eq_str.to_owned())),
//             Err(error) => Err(error.to_string()),
//         }
//     }
// }

impl fmt::Display for CircuitEquation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_device_equation() {
        let eq = indoc::indoc! {"
            vd = (vp - vn);
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vd/(eta*Vt)) - 1)
        "};
        assert!(
            DeviceEquation::from_str(eq).is_ok(),
            "Equation is valid diode equation, should be able to build operator tree."
        );
    }

    #[test]
    fn valid_circuit_equation() {
        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vd/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();
        let node_eq = DeviceEquation::from_str("(n1 - n2)").unwrap();
        let ctx = VariableContextMap::from([(CircuitNode::from_str("vd").unwrap(), node_eq)]);
        let cir_eq = CircuitEquation::new(dev_eq, &ctx);
        let dev_eq_with_nodes = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^((n1 - n2)/(eta*Vt)) - 1)
        "};
        assert_eq!(
            dev_eq_with_nodes,
            cir_eq.to_string(),
            "Substituted equation should contain: `n1 - n2`, in-place of `vd`."
        );
    }

    #[test]
    fn invalid_device_equation() {
        let eq = indoc::indoc! {"
            1 ^ 1 1
        "};
        assert!(
            DeviceEquation::from_str(eq).is_err(),
            "Equation is not valid, should produce an error"
        );
    }

    #[test]
    fn device_equation_map() {
        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vd/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();
        let _device_eq_map = DeviceEquationMap::from([("m1".to_owned(), dev_eq)]);
    }
}
