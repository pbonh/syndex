use crate::egraph::{LLHDEGraph, LLHDEgglogFacts, LLHDEgglogProgram};

pub type SynthesisMonad<T> = (T, LLHDEgglogProgram);

pub fn cmap<SynthT>(chip: SynthT) -> SynthesisMonad<SynthT>
where
    LLHDEgglogFacts: for<'world> From<&'world SynthT>,
{
    let llhd_facts = LLHDEgglogFacts::from(&chip);
    let egraph = LLHDEgglogProgram::from(llhd_facts);
    (chip, egraph)
}

pub fn synthesis_compose<SynthAT, SynthBT, SynthCT, SynthF1, SynthF2>(
    func1: SynthF1,
    func2: SynthF2,
) -> impl Fn(SynthAT) -> SynthesisMonad<SynthCT>
where
    SynthF1: Fn(SynthAT) -> SynthesisMonad<SynthBT> + 'static,
    SynthF2: Fn(SynthBT) -> SynthesisMonad<SynthCT> + 'static,
{
    move |chip: SynthAT| {
        let synth_step1 = func1(chip);
        let synth_step2 = func2(synth_step1.0);
        (synth_step2.0, synth_step1.1 + synth_step2.1)
    }
}

pub fn clift<SynthT>(chipm: SynthesisMonad<SynthT>) -> SynthT
where
    SynthT: From<LLHDEGraph>,
{
    let (_chip, egglog_program) = chipm;
    let egraph = LLHDEGraph::try_from(egglog_program)
        .expect("Failure to convert egglog program into EGraph.");
    SynthT::from(egraph)
}

#[cfg(test)]
mod tests {
    use specs::prelude::*;

    use super::*;

    #[test]
    fn synthesize_dummy_data_with_egraph() {
        let m1 = |x: i32| (x + 1, LLHDEgglogProgram::default());
        let m2 = |x: i32| (x * 2, LLHDEgglogProgram::default());

        let synthesized = synthesis_compose(m1, m2);
        let _result = synthesized(5);

        // println!("{:?}", result); // Output: (12, 30)
    }

    #[derive(Debug)]
    struct Vel(f32);

    impl Component for Vel {
        type Storage = VecStorage<Self>;
    }

    #[derive(Debug)]
    struct Pos(f32);

    impl Component for Pos {
        type Storage = VecStorage<Self>;
    }

    impl From<&World> for LLHDEgglogFacts {
        fn from(_value: &World) -> Self {
            todo!()
        }
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn initialize_egraph_with_ecs() {
        let world = World::new();
        let _synthesis_monad = cmap(world);
    }

    #[test]
    fn synthesize_ecs_with_egraph() {
        let world = World::new();

        let m1 = |mut chip_world: World| {
            chip_world.register::<Pos>();
            chip_world.create_entity().with(Pos(0.0)).build();
            (chip_world, LLHDEgglogProgram::default())
        };
        let m2 = |mut chip_world: World| {
            chip_world.register::<Vel>();
            chip_world
                .create_entity()
                .with(Vel(4.0))
                .with(Pos(1.6))
                .build();
            (chip_world, LLHDEgglogProgram::default())
        };
        let synthesizer12 = synthesis_compose(m1, m2);
        let synthesizer121 = synthesis_compose(m1, synthesizer12);
        let _synthesized121 = synthesizer121(world);
    }

    fn add_position(mut chip_world: World) -> SynthesisMonad<World> {
        chip_world.register::<Pos>();
        chip_world.create_entity().with(Pos(0.0)).build();
        (chip_world, LLHDEgglogProgram::default())
    }

    fn add_velocity(mut chip_world: World) -> SynthesisMonad<World> {
        chip_world.register::<Vel>();
        chip_world
            .create_entity()
            .with(Vel(4.0))
            .with(Pos(1.6))
            .build();
        (chip_world, LLHDEgglogProgram::default())
    }

    #[test]
    fn synthesize_macro_ecs_with_egraph() {
        let synthesize121 = synthesize!(add_position, add_velocity, add_position);

        let world = World::new();
        let _synthesized121 = synthesize121(world);
    }

    #[test]
    #[should_panic]
    fn synthesize_macro_ecs_with_egraph_fail() {
        let synthesize121 = synthesize!(add_velocity, add_position, add_velocity);

        let world = World::new();
        let _synthesized121 = synthesize121(world);
    }
}
