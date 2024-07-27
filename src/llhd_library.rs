pub mod circuit_library;
pub mod gds_library;
pub mod lef_library;

pub use builder::*;
use typestate::typestate;

#[typestate]
pub mod builder {
    use super::gds_library::LGdsLibrary;
    use super::lef_library::LLefLibrary;
    use crate::circuit::netlist::{AnalogCircuit, NetlistFlow};
    use crate::llhd::module::LLHDModule;

    #[derive(Debug, Default)]
    #[automaton]
    pub struct TechnologyFlow {
        lef: LLefLibrary,
        netlist: NetlistFlow<AnalogCircuit>,
        gds: LGdsLibrary,
        module: LLHDModule,
    }

    #[derive(Debug, Default)]
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
                state: Abstract,
                ..Default::default()
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
    use super::gds_library::LGdsLibrary;
    use super::lef_library::LLefLibrary;
    use super::*;
    use crate::circuit::netlist::*;

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn build_technology_flow() {
        let lef = LLefLibrary::default();
        let netlist = NetlistFlow::default();
        let gds = LGdsLibrary::default();
        TechnologyFlow::unbound_library()
            .load_lef(lef)
            .construct_circuit(netlist)
            .load_gds(gds)
            .bind_units();
    }
}
