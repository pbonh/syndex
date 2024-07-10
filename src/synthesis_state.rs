mod typestate_doc_example;

use typestate::typestate;

#[typestate]
pub mod builder {
    use crate::{llhd::module::LLHDModule, llhd_world::world::LLHDWorld};

    #[derive(Debug)]
    #[automaton]
    pub struct Flow {
        world: LLHDWorld,
    }

    #[state]
    pub struct Design;
    #[state]
    pub struct Technology;
    #[state]
    pub struct Synthesis;

    pub trait Design {
        fn setup(self) -> Technology;
        fn load(module: LLHDModule) -> Design;
        fn export(self);
    }

    pub trait Technology {
        fn load_rules(self) -> Synthesis;
    }

    pub trait Synthesis {
        fn synthesize(self) -> Design;
    }

    impl DesignState for Flow<Design> {
        fn setup(self) -> Flow<Technology> {
            todo!()
        }

        fn load(module: LLHDModule) -> Flow<Design> {
            let world = LLHDWorld::new(module);
            Self {
                world,
                state: Design,
            }
        }

        fn export(self) {
            todo!()
        }
    }

    impl TechnologyState for Flow<Technology> {
        fn load_rules(self) -> Flow<Synthesis> {
            todo!()
        }
    }

    impl SynthesisState for Flow<Synthesis> {
        fn synthesize(self) -> Flow<Design> {
            todo!()
        }
    }
}
