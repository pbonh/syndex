mod peginator_doc_example;

use peginator_macro::peginate;

peginate!(
    "
@export
SPICENetlist =
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
    use super::*;
    use peginator::PegParser;

    #[test]
    fn SPICE_netlist_example1() {
        let result = SPICENetlist::parse("Pizza with sausage, bacon and cheese").unwrap();
        println!("{:?}", result.toppings);
    }
}
