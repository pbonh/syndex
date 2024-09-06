use crate::egraph::LLHDEGraph;

pub type SynthesisMonad<T> = (T, LLHDEGraph);

pub fn cmap<T>(chip: T) -> SynthesisMonad<T>
where
    LLHDEGraph: for<'world> From<&'world T>,
{
    let egraph = LLHDEGraph::from(&chip);
    (chip, egraph)
}

pub fn synthesize<A, B, C, F1, F2>(func1: F1, func2: F2) -> impl Fn(A) -> SynthesisMonad<C>
where
    F1: Fn(A) -> SynthesisMonad<B> + 'static,
    F2: Fn(B) -> SynthesisMonad<C> + 'static,
{
    move |x: A| {
        let app_func1 = func1(x);
        let app_func2 = func2(app_func1.0);
        (app_func2.0, app_func1.1 + app_func2.1) // TODO: Do something to combine the egraph commands/runs
    }
}

#[cfg(test)]
mod tests {
    use specs::prelude::*;

    use super::*;

    #[test]
    fn synthesize_dummy_data_with_egraph() {
        let m1 = |x: i32| (x + 1, LLHDEGraph::try_from(vec![]).unwrap());
        let m2 = |x: i32| (x * 2, LLHDEGraph::try_from(vec![]).unwrap());

        let synthesized = synthesize(m1, m2);
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
            (chip_world, LLHDEGraph::try_from(vec![]).unwrap())
        };
        let m2 = |mut chip_world: World| {
            chip_world.register::<Vel>();
            chip_world
                .create_entity()
                .with(Vel(4.0))
                .with(Pos(1.6))
                .build();
            (chip_world, LLHDEGraph::try_from(vec![]).unwrap())
        };
        let synthesizer = synthesize(m1, m2);
        let _synthesized = synthesizer(world);
    }
}
