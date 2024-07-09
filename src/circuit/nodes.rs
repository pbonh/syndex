use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub(crate) struct CircuitNode(String);

impl FromStr for CircuitNode {
    type Err = String;

    fn from_str(node_str: &str) -> Result<Self, Self::Err> {
        if !node_str.contains(char::is_whitespace)
            && !node_str
                .chars()
                .next()
                .expect("Circuit Node String shouldn't be empty.")
                .is_numeric()
        {
            Ok(Self(node_str.to_owned()))
        } else {
            let mut error_str: String = "Invalid Node String: ".to_owned();
            error_str.push_str(node_str);
            Err(error_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_node() {
        let node = "f_1";
        assert!(
            CircuitNode::from_str(node).is_ok(),
            "Node is valid circuit node, should be able to create CircuitNode type."
        );
    }

    #[test]
    fn invalid_equation_whitespace() {
        let node = "f _1";
        assert!(
            CircuitNode::from_str(node).is_err(),
            "Node is not valid, should produce an error"
        );
    }

    #[test]
    fn invalid_equation_beginning_digit() {
        let node = "1f_1";
        assert!(
            CircuitNode::from_str(node).is_err(),
            "Node is not valid, should produce an error"
        );
    }
}
