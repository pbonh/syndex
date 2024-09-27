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
    use std::str::FromStr;

    use egglog::ast::*;
    use egglog::{EGraph, TermDag};
    use itertools::Itertools;
    use llhd::ir::prelude::*;

    use super::*;
    use crate::egraph::facts::EgglogFacts;
    use crate::egraph::rules::EgglogRules;
    use crate::egraph::schedule::EgglogSchedules;
    use crate::egraph::sorts::EgglogSorts;
    use crate::egraph::*;
    use crate::llhd_egraph::datatype::LLHDEgglogSorts;
    use crate::llhd_egraph::llhd::LLHDEgglogProgram;
    use crate::llhd_egraph::rules::LLHDEgglogRules;
    use crate::llhd_egraph::unit::{expr_to_unit_data, unit_symbol};
    use crate::llhd_egraph::LLHDEgglogFacts;

    impl From<LLHDEgglogSorts> for EgglogSorts {
        fn from(llhd_sorts: LLHDEgglogSorts) -> Self {
            Self::default().add_sorts(<LLHDEgglogSorts as Into<EgglogCommandList>>::into(
                llhd_sorts,
            ))
        }
    }

    impl From<LLHDEgglogFacts> for EgglogFacts {
        fn from(llhd_facts: LLHDEgglogFacts) -> Self {
            Self::default().add_facts(<LLHDEgglogFacts as Into<EgglogCommandList>>::into(
                llhd_facts,
            ))
        }
    }

    impl From<LLHDEgglogRules> for EgglogRules {
        fn from(llhd_rules: LLHDEgglogRules) -> Self {
            Self::default().add_rules(<LLHDEgglogRules as Into<EgglogCommandList>>::into(
                llhd_rules,
            ))
        }
    }

    impl From<&Module> for EgglogProgram {
        fn from(module: &Module) -> Self {
            let llhd_unit_symbols: EgglogSymbols =
                module.units().map(|unit| unit_symbol(&unit)).collect();
            let llhd_facts = LLHDEgglogFacts::from_module(module);
            let llhd_egglog_program = LLHDEgglogProgram::builder()
                .facts(llhd_facts)
                .rules(LLHDEgglogRules::default())
                .build();

            EgglogProgramBuilder::<InitState>::new()
                .sorts(llhd_egglog_program.sorts().clone().into())
                .facts(llhd_egglog_program.facts().clone().into())
                .rules(EgglogRules::default())
                .schedules(EgglogSchedules::default())
                .bindings(llhd_unit_symbols)
                .program()
        }
    }

    impl From<EgglogProgram> for Module {
        fn from(program: EgglogProgram) -> Self {
            let unit_symbols = program.bindings().to_owned();
            let mut module = Self::new();
            let mut egraph = EGraph::default();
            if let Err(err_msg) = egraph.run_program(program.into()) {
                panic!("Failure to run EgglogProgram. Err: {:?}", err_msg);
            }
            for unit_symbol in unit_symbols.into_iter() {
                let extract_cmd = GenericCommand::QueryExtract {
                    span: DUMMY_SPAN.clone(),
                    variants: 0,
                    expr: GenericExpr::Var(DUMMY_SPAN.clone(), unit_symbol.clone()),
                };
                if let Err(egraph_extract_err) = egraph.run_program(vec![extract_cmd]) {
                    println!("Cannot extract expression: {:?}", egraph_extract_err);
                }
                let mut extracted_termdag = TermDag::default();
                let (unit_sort, unit_symbol_value) = egraph
                    .eval_expr(&GenericExpr::Var(DUMMY_SPAN.clone(), unit_symbol))
                    .unwrap();
                let (_unit_cost, unit_term) =
                    egraph.extract(unit_symbol_value, &mut extracted_termdag, &unit_sort);
                let extracted_expr = extracted_termdag.term_to_expr(&unit_term);
                let mut sig = Signature::new();
                let _in1 = sig.add_input(llhd::int_ty(1));
                let _in2 = sig.add_input(llhd::int_ty(1));
                let _in3 = sig.add_input(llhd::int_ty(1));
                // let _in4 = sig.add_input(llhd::int_ty(1));
                let _out1 = sig.add_output(llhd::signal_ty(llhd::int_ty(1)));
                let unit_data = expr_to_unit_data(
                    extracted_expr,
                    UnitKind::Entity,
                    UnitName::Anonymous(0),
                    sig,
                );
                let _unit_id = module.add_unit(unit_data);
            }
            module
        }
    }

    #[test]
    fn monad_composition() {
        let init_module = |module: Module| SynthesisContext::load(module);
        let add_alpha_unit = |mut module: Module| {
            let new_unit = utilities::build_entity_alpha(UnitName::anonymous(0));
            let _new_unit_id = module.add_unit(new_unit);
            SynthesisContext::load(module)
        };
        let composed_fn = compose(init_module, add_alpha_unit);
        let _compose_result = composed_fn(utilities::load_llhd_module("2and_1or.llhd"));

        let monad_a = SynthesisContext::load(utilities::load_llhd_module("2and_1or.llhd"));
        let _bind_result =
            monad_a.bind(|_x| SynthesisContext::load(utilities::load_llhd_module("2and_1or.llhd")));
    }

    #[test]
    fn monad_lift() {
        let init_module = |module: Module| SynthesisContext::load(module);
        let add_div_extract_unit = |module: Module| {
            // let new_unit = utilities::build_entity_2and_1or_common(UnitName::anonymous(0));
            // let _new_unit_id = module.add_unit(new_unit);
            let llhd_facts = LLHDEgglogFacts::from_module(&module);
            let llhd_egglog_program = LLHDEgglogProgram::builder()
                .facts(llhd_facts)
                .rules(
                    LLHDEgglogRules::from_str(&utilities::get_egglog_commands(
                        "llhd_div_extract_schedule.egg",
                    ))
                    .unwrap(),
                )
                .build();
            let egglog_program = EgglogProgramBuilder::<InitState>::new()
                .sorts(EgglogSorts::default())
                .facts(EgglogFacts::default())
                .rules(llhd_egglog_program.rules().clone().into())
                .schedules(EgglogSchedules::default())
                .bindings(EgglogSymbols::default())
                .program();
            SynthesisContext {
                program: egglog_program,
                design: module,
            }
        };
        let composed_fn = compose(init_module, add_div_extract_unit);
        let compose_result = composed_fn(utilities::load_llhd_module("2and_1or_common.llhd"));
        let new_module = compose_result.resolve();

        assert_eq!(
            1,
            new_module.units().collect_vec().len(),
            "New Module should have Unit added(2and_1or_common)."
        );
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
