pub mod circuit_library;
pub mod gds_library;
pub mod lef_library;

#[allow(unused_imports)]
use builder::*;
use typestate::typestate;

#[typestate]
pub mod builder {
    use super::gds_library::LGdsLibrary;
    use super::lef_library::LLefLibrary;
    use crate::circuit::graph::LCircuit;
    use crate::circuit::netlist::LNetlist;
    use crate::llhd::module::LLHDModule;

    #[derive(Debug)]
    #[automaton]
    pub struct TechnologyFlow {
        lef: LLefLibrary,
        circuit: LCircuit,
        gds: LGdsLibrary,
        module: LLHDModule,
    }

    #[state]
    pub struct Abstract;
    #[state]
    pub struct Analog;
    #[state]
    pub struct Physical;
    #[state]
    pub struct Bound;

    pub trait Abstract {
        fn unbound_library() -> Abstract;
        fn load_lef(self, library_lef: LLefLibrary) -> Analog;
    }

    pub trait Analog {
        fn construct_circuit(self, netlist: LNetlist) -> Physical;
    }

    pub trait Physical {
        fn load_gds(self, library_gds: LGdsLibrary) -> Bound;
    }

    pub trait Bound {
        fn bind_units(self);
    }

    impl AbstractState for TechnologyFlow<Abstract> {
        fn unbound_library() -> TechnologyFlow<Abstract> {
            Self {
                lef: LLefLibrary::default(),
                circuit: LCircuit::default(),
                gds: LGdsLibrary::default(),
                module: LLHDModule::default(),
                state: Abstract,
            }
        }

        fn load_lef(self, lef: LLefLibrary) -> TechnologyFlow<Analog> {
            TechnologyFlow::<Analog> {
                lef,
                circuit: self.circuit,
                gds: self.gds,
                module: self.module,
                state: Analog,
            }
        }
    }

    impl AnalogState for TechnologyFlow<Analog> {
        fn construct_circuit(self, _netlist: LNetlist) -> TechnologyFlow<Physical> {
            todo!()
        }
    }

    impl PhysicalState for TechnologyFlow<Physical> {
        fn load_gds(self, _gds: LGdsLibrary) -> TechnologyFlow<Bound> {
            todo!()
        }
    }

    impl BoundState for TechnologyFlow<Bound> {
        fn bind_units(self) {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::gds_library::LGdsLibrary;
    use super::lef_library::LLefLibrary;
    use super::*;
    use crate::circuit::netlist::LNetlist;

    #[test]
    #[should_panic]
    fn build_technology_flow() {
        let lef = LLefLibrary::default();
        let netlist = LNetlist::default();
        let gds = LGdsLibrary::default();
        TechnologyFlow::unbound_library()
            .load_lef(lef)
            .construct_circuit(netlist)
            .load_gds(gds)
            .bind_units();
    }
}
