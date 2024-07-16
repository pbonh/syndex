mod typestate_doc_example;

use typestate::typestate;

#[typestate]
pub mod builder {
    use crate::llhd::module::LLHDModule;
    use crate::llhd_world::world::LLHDWorld;

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
        fn load(module: LLHDModule) -> Technology;
        fn export(self);
    }

    pub trait Technology {
        fn constrain(self) -> Synthesis;
    }

    pub trait Synthesis {
        fn synthesize(self) -> Design;
    }

    impl DesignState for Flow<Design> {
        fn load(module: LLHDModule) -> Flow<Technology> {
            let world = LLHDWorld::new(module);
            Flow::<Technology> {
                world,
                state: Technology,
            }
        }

        fn export(self) {
            todo!()
        }
    }

    impl TechnologyState for Flow<Technology> {
        fn constrain(self) -> Flow<Synthesis> {
            todo!()
        }
    }

    impl SynthesisState for Flow<Synthesis> {
        fn synthesize(self) -> Flow<Design> {
            todo!()
        }
    }
}
