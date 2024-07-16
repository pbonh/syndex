pub mod gds_library;
pub mod lef_library;

use typestate::typestate;

#[typestate]
pub mod builder {
    use super::gds_library::LGdsLibrary;
    use super::lef_library::LLefLibrary;
    use crate::llhd::module::LLHDModule;

    #[derive(Debug)]
    #[automaton]
    pub struct TechnologyFlow {
        lef: LLefLibrary,
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
        fn construct_circuit(self) -> Physical;
    }

    pub trait Physical {
        fn load_gds(self) -> Bound;
    }

    pub trait Bound {
        fn bind_units(self);
    }

    impl AbstractState for TechnologyFlow<Abstract> {
        fn unbound_library() -> TechnologyFlow<Abstract> {
            TechnologyFlow::<Abstract> {
                lef: LLefLibrary::default(),
                gds: LGdsLibrary::default(),
                module: LLHDModule::default(),
                state: Abstract,
            }
        }

        fn load_lef(self, lef: LLefLibrary) -> TechnologyFlow<Analog> {
            TechnologyFlow::<Analog> {
                lef,
                gds: self.gds,
                module: self.module,
                state: Analog,
            }
        }
    }

    impl AnalogState for TechnologyFlow<Analog> {
        fn construct_circuit(self) -> TechnologyFlow<Physical> {
            todo!()
        }
    }

    impl PhysicalState for TechnologyFlow<Physical> {
        fn load_gds(self) -> TechnologyFlow<Bound> {
            todo!()
        }
    }

    impl BoundState for TechnologyFlow<Bound> {
        fn bind_units(self) {
            todo!()
        }
    }
}
