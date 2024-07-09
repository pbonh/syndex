use evalexpr::build_operator_tree;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub(crate) struct CircuitEquation(String);

impl FromStr for CircuitEquation {
    type Err = String;

    fn from_str(eq_str: &str) -> Result<Self, Self::Err> {
        match build_operator_tree(eq_str) {
            Ok(_) => Ok(Self(eq_str.to_owned())),
            Err(error) => Err(error.to_string()),
        }
    }
}

impl fmt::Display for CircuitEquation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_equation() {
        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vd/(eta*Vt)) - 1)
        "};
        assert!(
            CircuitEquation::from_str(eq).is_ok(),
            "Equation is valid diode equation, should be able to build operator tree."
        );
    }

    #[test]
    fn invalid_equation() {
        let eq = indoc::indoc! {"
            1 ^ 1 1
        "};
        assert!(
            CircuitEquation::from_str(eq).is_err(),
            "Equation is not valid, should produce an error"
        );
    }
}
