pub use builder::*;
use typestate::typestate;

#[typestate]
pub mod builder {
    use crate::circuit::equations::DeviceEquationMap;
    use crate::circuit::graph::LCircuit;
    use crate::circuit::spice::SPICENetlist;

    #[derive(Debug)]
    #[automaton]
    pub struct NetlistFlow {
        physics: DeviceEquationMap,
        graph: LCircuit,
        spice: SPICENetlist,
    }

    #[derive(Debug)]
    #[state]
    pub struct DevicePhysics;

    #[derive(Debug)]
    #[state]
    pub struct Connectivity;

    #[derive(Debug)]
    #[state]
    pub struct AnalogCircuit;

    pub trait DevicePhysics {
        fn initialize() -> DevicePhysics;
        fn equations(self, map: DeviceEquationMap) -> Connectivity;
    }

    pub trait Connectivity {
        fn spice(self, spice_netlist: SPICENetlist) -> AnalogCircuit;
    }

    pub trait AnalogCircuit {
        fn build(self) -> Self;
    }

    impl DevicePhysicsState for NetlistFlow<DevicePhysics> {
        fn initialize() -> NetlistFlow<DevicePhysics> {
            todo!()
        }

        fn equations(self, _map: DeviceEquationMap) -> NetlistFlow<Connectivity> {
            todo!()
        }
    }

    impl ConnectivityState for NetlistFlow<Connectivity> {
        fn spice(self, _spice_netlist: SPICENetlist) -> NetlistFlow<AnalogCircuit> {
            todo!()
        }
    }

    impl AnalogCircuitState for NetlistFlow<AnalogCircuit> {
        fn build(self) -> Self {
            todo!()
        }
    }
}
#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    use peginator::PegParser;

    use crate::circuit::equations::{DeviceEquation, DeviceEquationMap};
    use crate::circuit::netlist::*;
    use crate::circuit::spice::SPICENetlist;

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn spice_sky130_dk_a211o_2_netlist() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.spice",
        );
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();

        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vds/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();
        let device_eq_map = DeviceEquationMap::from([("m".to_owned(), dev_eq)]);

        let spice_netlist = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let _netlist = NetlistFlow::initialize()
            .equations(device_eq_map)
            .spice(spice_netlist)
            .build();
    }
}
