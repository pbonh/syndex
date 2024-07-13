mod typestate_doc_example;

use typestate::typestate;

/// Build A Synthesis Flow
///
/// 1) Start with a Digital Design(LLHD Module)
/// 2) Constrain the Synthesis Flow to a Technology
/// 3) Apply Synthesis Rules to Design
///
/// ```rust
/// # use syndex::synthesis_state::builder::*;
/// # let input = indoc::indoc! {"
/// #         entity @test_entity (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
/// #             %null = const time 0s 1e
/// #             %and1 = and i1 %in1, %in2
/// #             %and2 = and i1 %in3, %in4
/// #             %or1 = or i1 %and1, %and2
/// #             drv i1$ %out1, %or1, %null
/// #         }
/// #     "};
///
/// # let module = llhd::assembly::parse_module(input).unwrap();
/// let _technology_flow = Flow::load(module.into());
/// ```
///

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
