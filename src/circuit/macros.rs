use crate::circuit::graph::LCircuit;

// Define a macro for the DSL
#[allow(unused_macros)]
macro_rules! circuit {
    // Entry point for the macro
    ($($name:ident { $($field:ident = $value:expr;)* })*) => {{
        let mut circuit = LCircuit::default();
        $(
            circuit.add_component(stringify!($name), vec![
                $((stringify!($field).to_string(), $value.to_string()),)*
            ]);
        )*
        circuit
    }};
}

// Netlist data structure
struct Netlist {
    components: Vec<Component>,
}

impl Netlist {
    fn new() -> Self {
        Netlist {
            components: Vec::new(),
        }
    }

    fn add_component(&mut self, name: &str, parameters: Vec<(String, String)>) {
        self.components.push(Component::new(name, parameters));
    }
}

impl std::fmt::Debug for Netlist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for component in &self.components {
            writeln!(f, "{:?}", component)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Component {
    name: String,
    parameters: Vec<(String, String)>,
}

impl Component {
    fn new(name: &str, parameters: Vec<(String, String)>) -> Self {
        Component {
            name: name.to_string(),
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_inverter_netlist() {
        // Simple CMOS inverter with RLC parasitics and transistor I-V equations
        let cmos_inverter = circuit! {
            transistor {
                name = "M1";
                drain = "out";
                gate = "in";
                source = "vss";
                body = "vss";
                type_ = "NMOS";
                model = "NMOS_IV";
            }
            transistor {
                name = "M2";
                drain = "out";
                gate = "in";
                source = "vdd";
                body = "vdd";
                type_ = "PMOS";
                model = "PMOS_IV";
            }
            resistor {
                name = "R1";
                n1 = "out";
                n2 = "vdd";
                value = "10k";
            }
            inductor {
                name = "L1";
                n1 = "out";
                n2 = "vdd";
                value = "10mH";
            }
            capacitor {
                name = "C1";
                n1 = "out";
                n2 = "vdd";
                value = "100nF";
            }
        };

        println!("{:?}", cmos_inverter);
    }
}
