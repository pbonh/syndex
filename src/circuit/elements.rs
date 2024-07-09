pub(crate) mod transistor;

use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub(crate) struct CircuitElement(String);

impl FromStr for CircuitElement {
    type Err = String;

    fn from_str(element_str: &str) -> Result<Self, Self::Err> {
        if !element_str.contains(char::is_whitespace)
            && !element_str
                .chars()
                .next()
                .expect("Circuit Element String shouldn't be empty.")
                .is_numeric()
        {
            Ok(Self(element_str.to_owned()))
        } else {
            let mut error_str: String = "Invalid Element String: ".to_owned();
            error_str.push_str(element_str);
            Err(error_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_element() {
        let element = "f_1";
        assert!(
            CircuitElement::from_str(element).is_ok(),
            "Element is valid circuit element, should be able to create CircuitElement type."
        );
    }

    #[test]
    fn invalid_element_whitespace() {
        let element = "f _1";
        assert!(
            CircuitElement::from_str(element).is_err(),
            "Element is not valid, should produce an error"
        );
    }

    #[test]
    fn invalid_element_beginning_digit() {
        let element = "1f_1";
        assert!(
            CircuitElement::from_str(element).is_err(),
            "Element is not valid, should produce an error"
        );
    }
}
