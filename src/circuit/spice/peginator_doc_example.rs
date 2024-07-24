use peginator_macro::peginate;

peginate!(
    "
@export
PizzaRule =
    'Pizza' 'with'
    toppings:Topping
    {',' toppings:Topping}
    ['and' toppings:Topping]
;
@string
Topping = 'sausage' | 'pineapple' | 'bacon' | 'cheese';
"
);

#[cfg(test)]
mod tests {
    use peginator::PegParser;

    use super::*;

    #[test]
    fn doc_example() {
        let result = PizzaRule::parse("Pizza with sausage, bacon and cheese").unwrap();
        println!("{:?}", result.toppings);
    }
}
