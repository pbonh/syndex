pub mod circuit_library;
pub mod gds_library;
pub mod lef_library;

pub use builder::*;
use typestate::typestate;

#[typestate]
pub mod builder {
    use super::gds_library::LGdsLibrary;
    use super::lef_library::LLefLibrary;
    use crate::circuit::netlist::*;
    use crate::llhd::module::LLHDModule;

    #[derive(Debug)]
    #[automaton]
    pub struct TechnologyFlow {
        lef: LLefLibrary,
        netlist: Option<NetlistFlow<AnalogCircuit>>,
        gds: LGdsLibrary,
        module: LLHDModule,
    }

    #[derive(Debug)]
    #[state]
    pub struct Abstract;

    #[derive(Debug)]
    #[state]
    pub struct Analog;

    #[derive(Debug)]
    #[state]
    pub struct Physical;

    #[derive(Debug)]
    #[state]
    pub struct Bound;

    pub trait Abstract {
        fn unbound_library() -> Abstract;
        fn load_lef(self, library_lef: LLefLibrary) -> Analog;
    }

    pub trait Analog {
        fn construct_circuit(self, netlist: NetlistFlow<AnalogCircuit>) -> Physical;
    }

    pub trait Physical {
        fn load_gds(self, library_gds: LGdsLibrary) -> Bound;
    }

    pub trait Bound {
        fn bind_units(self) -> Self;
    }

    impl AbstractState for TechnologyFlow<Abstract> {
        fn unbound_library() -> TechnologyFlow<Abstract> {
            Self {
                lef: LLefLibrary::default(),
                netlist: None,
                gds: LGdsLibrary::default(),
                module: LLHDModule::default(),
                state: Abstract,
            }
        }

        fn load_lef(self, lef: LLefLibrary) -> TechnologyFlow<Analog> {
            TechnologyFlow::<Analog> {
                lef,
                netlist: self.netlist,
                gds: self.gds,
                module: self.module,
                state: Analog,
            }
        }
    }

    impl AnalogState for TechnologyFlow<Analog> {
        fn construct_circuit(
            self,
            _netlist: NetlistFlow<AnalogCircuit>,
        ) -> TechnologyFlow<Physical> {
            todo!()
        }
    }

    impl PhysicalState for TechnologyFlow<Physical> {
        fn load_gds(self, _gds: LGdsLibrary) -> TechnologyFlow<Bound> {
            todo!()
        }
    }

    impl BoundState for TechnologyFlow<Bound> {
        fn bind_units(self) -> Self {
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

    use super::gds_library::LGdsLibrary;
    use super::lef_library::LLefLibrary;
    use super::*;
    use crate::circuit::equations::{DeviceEquation, DeviceEquationMap};
    use crate::circuit::netlist::*;
    use crate::circuit::spice::SPICENetlist;

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn build_technology_flow() {
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
        let netlist = NetlistFlow::initialize()
            .equations(device_eq_map)
            .spice(spice_netlist)
            .build();

        let lef = LLefLibrary::default();
        let gds = LGdsLibrary::default();
        TechnologyFlow::unbound_library()
            .load_lef(lef)
            .construct_circuit(netlist)
            .load_gds(gds)
            .bind_units();
    }
}
