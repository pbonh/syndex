pub(crate) mod transistor;

use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub(crate) struct CircuitElement(String);

impl FromStr for CircuitElement {
    type Err = String;

    fn from_str(element_str: &str) -> Result<Self, Self::Err> {
        let no_whitespace = !element_str.contains(char::is_whitespace);
        let non_numeric_start = element_str
            .chars()
            .next()
            .expect("Circuit Element String shouldn't be empty.")
            .is_numeric();
        let multiple_characters = element_str.chars().count() > 1;
        if no_whitespace && !non_numeric_start && multiple_characters {
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

    #[test]
    fn invalid_element_single_char() {
        let element = "f";
        assert!(
            CircuitElement::from_str(element).is_err(),
            "Element is not valid, should produce an error"
        );
    }
}
