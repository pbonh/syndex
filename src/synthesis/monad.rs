use crate::egraph::EgglogProgram;

#[derive(Debug, Clone)]
struct SynthesisContext<Design> {
    program: EgglogProgram,
    design: Design,
}

impl<Design> SynthesisContext<Design> {
    fn load(design: Design) -> Self
    where
        EgglogProgram: for<'design> From<&'design Design>,
    {
        Self {
            program: EgglogProgram::from(&design),
            design,
        }
    }

    fn resolve(self) -> Design
    where
        Design: From<EgglogProgram>,
    {
        Design::from(self.program)
    }

    fn bind<SynthCtx, Synth>(self, synth: Synth) -> SynthesisContext<SynthCtx>
    where
        Synth: Fn(Design) -> SynthesisContext<SynthCtx> + 'static,
    {
        let mut synth_ctx = synth(self.design);
        synth_ctx.program = self.program + synth_ctx.program;
        synth_ctx
    }
}

fn compose<A, B, C, SynthAB, SynthBC>(
    pass_ab: SynthAB,
    pass_bc: SynthBC,
) -> impl Fn(A) -> SynthesisContext<C>
where
    SynthAB: Fn(A) -> SynthesisContext<B> + 'static,
    SynthBC: Fn(B) -> SynthesisContext<C> + 'static,
{
    move |val: A| {
        let synth_pass_ab = pass_ab(val);
        let synth_pass_bc = pass_bc(synth_pass_ab.design);
        SynthesisContext {
            program: synth_pass_ab.program + synth_pass_bc.program,
            design: synth_pass_bc.design,
        }
    }
}

macro_rules! bind_chain {
    ($monad:expr, $func:expr) => {
        $monad.bind($func)
    };
    ($monad:expr, $func:expr, $($rest:expr),+) => {
        bind_chain!($monad.bind($func), $($rest),+)
    };
}

macro_rules! compose_chain {
    ($first:expr) => {
        $first
    };
    ($first:expr, $($rest:expr),+) => {
        compose($first, compose_chain!($($rest),+))
    };
}

#[cfg(test)]
mod tests {
    use llhd::ir::prelude::*;

    use super::*;

    impl From<&Module> for EgglogProgram {
        fn from(_module: &Module) -> Self {
            todo!()
        }
    }

    impl From<Module> for EgglogProgram {
        fn from(_module: Module) -> Self {
            todo!()
        }
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn monad_composition() {
        let f = |module: Module| SynthesisContext::load(module);
        let g = |mut module: Module| {
            let new_unit = utilities::build_entity_alpha(UnitName::anonymous(0));
            let _new_unit_id = module.add_unit(new_unit);
            SynthesisContext::load(module)
        };
        let composed_fn = compose(f, g);
        let _compose_result = composed_fn(utilities::load_llhd_module("2and_1or.llhd"));

        let monad_a = SynthesisContext::load(utilities::load_llhd_module("2and_1or.llhd"));
        let _bind_result =
            monad_a.bind(|_x| SynthesisContext::load(utilities::load_llhd_module("2and_1or.llhd")));

        // assert_eq!(
        //     compose_result, bind_result,
        //     "Compose and bind should produce the same result."
        // );
    }

    // #[test]
    // fn monad_composition_macro() {
    //     let f = |x: i32| SynthesisContext::load(x * 2);
    //     let g = |x: i32| SynthesisContext::load(x + 10);
    //     let h = |x: i32| SynthesisContext::load(x - 1);
    //
    //     let monad = SynthesisContext::load(5);
    //     let bind_result = bind_chain!(monad, f, g, h);
    //     println!("Bind chain result: {:?}", bind_result);
    //
    //     let composed_fn = compose_chain!(f, g, h);
    //     let compose_result = composed_fn(5);
    //     println!("Compose chain result: {:?}", compose_result);
    //
    //     // assert_eq!(
    //     //     compose_result.design, bind_result.design,
    //     //     "Compose and bind should produce the same result."
    //     // );
    // }
}
