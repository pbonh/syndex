pub(crate) mod egglog_names;
pub mod facts;
#[macro_use]
pub mod macros;
pub mod rules;
pub mod schedule;
pub mod sorts;
use egglog::ast::Command;
use facts::EgglogFacts;
use frunk::monoid::Monoid;
use frunk::semigroup::Semigroup;
use itertools::Itertools;
use rules::EgglogRules;
use schedule::EgglogSchedules;
use sorts::EgglogSorts;

pub(crate) type EgglogCommandList = Vec<Command>;
type EgglogSortList = Vec<EgglogSorts>;
type EgglogFactList = Vec<EgglogFacts>;
type EgglogRuleList = Vec<EgglogRules>;
type EgglogScheduleList = Vec<EgglogSchedules>;

#[derive(Debug, Clone, Default)]
pub struct EgglogProgram {
    sorts: EgglogSortList,
    facts: EgglogFactList,
    rules: EgglogRuleList,
    schedules: EgglogScheduleList,
}

impl EgglogProgram {
    pub fn sorts(self, sorts: EgglogSorts) -> Self {
        Self {
            sorts: vec![sorts],
            facts: self.facts,
            rules: self.rules,
            schedules: self.schedules,
        }
    }

    pub fn rules(self, rules: EgglogRules) -> Self {
        Self {
            sorts: self.sorts,
            facts: self.facts,
            rules: vec![rules],
            schedules: self.schedules,
        }
    }

    pub fn schedule(self, schedules: EgglogSchedules) -> Self {
        Self {
            sorts: self.sorts,
            facts: self.facts,
            rules: self.rules,
            schedules: vec![schedules],
        }
    }
}

#[derive(Debug)]
pub struct EgglogProgramBuilder {
    sorts: EgglogSortList,
    facts: EgglogFactList,
}

impl EgglogProgramBuilder {
    pub fn sorts(input_sorts: EgglogSorts) -> Self {
        Self {
            sorts: vec![input_sorts],
            facts: vec![],
        }
    }

    pub fn facts(self, input_facts: EgglogFacts) -> Self {
        Self {
            sorts: self.sorts,
            facts: vec![input_facts],
        }
    }

    pub fn program(self) -> EgglogProgram {
        EgglogProgram {
            sorts: self.sorts,
            facts: self.facts,
            ..Default::default()
        }
    }
}

impl Semigroup for EgglogProgram {
    fn combine(&self, program_update: &Self) -> Self {
        let mut combined_sorts = self.sorts.clone();
        combined_sorts.append(&mut program_update.sorts.clone());
        let mut combined_rules = self.rules.clone();
        combined_rules.append(&mut program_update.rules.clone());
        let mut combined_schedules = self.schedules.clone();
        combined_schedules.append(&mut program_update.schedules.clone());
        Self {
            sorts: combined_sorts,
            facts: self.facts.clone(),
            rules: combined_rules,
            schedules: combined_schedules,
        }
    }
}

impl Monoid for EgglogProgram {
    fn empty() -> Self {
        Self::default()
    }
}

impl Into<EgglogCommandList> for EgglogProgram {
    fn into(self) -> EgglogCommandList {
        self.sorts
            .into_iter()
            .flatten()
            .chain(
                self.facts.into_iter().flatten().chain(
                    self.rules
                        .into_iter()
                        .flatten()
                        .chain(self.schedules.into_iter().flatten()),
                ),
            )
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use egglog::EGraph;

    use super::*;

    #[test]
    fn init_empty_egglog_program() {
        let _empty_egglog_program = EgglogProgram::default();
    }

    #[test]
    fn build_egglog_program() {
        let input_sorts: EgglogSorts = Default::default();
        let input_facts: EgglogFacts = Default::default();
        let egglog_program = EgglogProgramBuilder::sorts(input_sorts)
            .facts(input_facts)
            .program();
        let rules1: EgglogRules = Default::default();
        let schedule1: EgglogSchedules = Default::default();
        egglog_program.rules(rules1).schedule(schedule1);
    }

    #[test]
    fn combine_egglog_programs() {
        let sort_str = utilities::get_egglog_commands("llhd_dfg_example2_sorts.egg");
        let input_sorts = EgglogSorts::default().add_sort_str(&sort_str);
        let facts_str = utilities::get_egglog_commands("llhd_dfg_example2_facts.egg");
        let input_facts = EgglogFacts::default().add_facts_str(&facts_str);

        let rules_str = utilities::get_egglog_commands("llhd_dfg_example2_rules.egg");
        let rules1 = EgglogRules::default().add_rule_str(&rules_str);
        let schedule1_str = utilities::get_egglog_commands("llhd_dfg_example2_schedule.egg");
        let schedule1 = EgglogSchedules::default().add_schedule_str(&schedule1_str);
        let egglog_program = EgglogProgramBuilder::sorts(input_sorts)
            .facts(input_facts)
            .program()
            .rules(rules1)
            .schedule(schedule1);

        let sort2_str = utilities::get_egglog_commands("llhd_dfg_example2_sorts_updated.egg");
        let sorts2 = EgglogSorts::default().add_sort_str(&sort2_str);
        let rules2_str = utilities::get_egglog_commands("llhd_dfg_example2_rules_updated.egg");
        let rules2 = EgglogRules::default().add_rule_str(&rules2_str);
        let schedule2_str =
            utilities::get_egglog_commands("llhd_dfg_example2_schedule_updated.egg");
        let schedule2 = EgglogSchedules::default().add_schedule_str(&schedule2_str);
        let egglog_program_update = EgglogProgram::default()
            .sorts(sorts2)
            .rules(rules2)
            .schedule(schedule2);
        let updated_egglog_program = egglog_program.combine(&egglog_program_update);
        assert_eq!(2, updated_egglog_program.sorts.len());
        assert_eq!(1, updated_egglog_program.facts.len());
        assert_eq!(2, updated_egglog_program.rules.len());
        assert_eq!(2, updated_egglog_program.schedules.len());
        let updated_egglog_program_cmds: EgglogCommandList = updated_egglog_program.into();
        assert_eq!(18, updated_egglog_program_cmds.len());
        assert!(matches!(
            updated_egglog_program_cmds[0],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[1], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[2],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[3], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[4],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[5],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[6], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[7],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[8],
            Command::Datatype { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[9],
            Command::Datatype { .. }
        ));
        assert!(matches!(updated_egglog_program_cmds[10], Command::Sort(..)));
        assert!(matches!(
            updated_egglog_program_cmds[11],
            Command::Action { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[12],
            Command::AddRuleset(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[13],
            Command::Rewrite(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[14],
            Command::AddRuleset(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[15],
            Command::Rule { .. }
        ));
        assert!(matches!(
            updated_egglog_program_cmds[16],
            Command::RunSchedule(..)
        ));
        assert!(matches!(
            updated_egglog_program_cmds[17],
            Command::RunSchedule(..)
        ));
        if let Err(err_msg) = EGraph::default().run_program(updated_egglog_program_cmds) {
            panic!("Failure to run program: {:?}", err_msg);
        }
    }

    #[test]
    fn egglog_type_macros() {
        use egglog::ast::*;

        // let set_option_cmd = cmd!(SetOption {
        //     name: "node_limit",
        //     value: 1000,
        // });

        let _datatype_cmd: Command = cmd!(Datatype {
            span: span!(),
            name: "Math",
            variants: vec![
                variant!("Num", ["i64"]),
                variant!("Var", ["String"]),
                variant!("Add", ["Math", "Math"]),
                variant!("Mul", ["Math", "Math"]),
            ],
        });

        // let function_cmd = cmd!(
        //     Function(function_decl!(
        //         "Add",
        //         inputs = ["Math", "Math"],
        //         output = "Math"
        //     ))
        // );
        // Optional fields can be added here
        // default = expr!(0),
        // cost = Some(1),

        // let print_function_cmd = cmd!(PrintFunction(span!(), "Add", 20));

        // let rewrite_cmd = cmd!(Rewrite(
        //     symbol!("commute_add"),
        //     GenericRewrite {
        //         span: span!(),
        //         lhs: expr!("Add", var "a", var "b"),
        //         rhs: expr!("Add", var "b", var "a"),
        //         conditions: vec![],
        //     },
        //     false,
        // ));
        //
        // let run_schedule_cmd = cmd!(
        //     RunSchedule(
        //         schedule!(sequence [
        //             saturate run "my_ruleset_1",
        //             run "my_ruleset_2", until = [("eq", [expr!(var "x"), 0])]
        // ])));
        //
        // let check_cmd = cmd!(
        //     Check(
        //         span!(),
        //         facts = [
        //             eq [expr!(var "x"), 0],
        //             expr!("greater_than", var "y", 10)
        //         ]
        //     )
        // );
    }

    #[test]
    const fn egglog_syntax_macros() {
        use egglog_syntax::egglog_expr_str;

        let _llhd_dfg_egglog_expr = egglog_expr_str!(
            r#"
            (datatype LLHDValue (Value u64)) (sort LLHDVecValue (Vec u64))
            (datatype LLHDBlock (Block u64)) (sort LLHDVecBlock (Vec LLHDBlock))
            (datatype LLHDExtUnit (ExtUnit u64))
            (datatype LLHDRegMode
                (Low)
                (High)
                (Rise)
                (Fall)
                (Both))
            (sort LLHDVecRegMode (Vec LLHDRegMode))
            (datatype LLHDDFG
                (ValueRef u64)
                (ConstInt String)
                (ConstTime String)
                (Alias LLHDDFG)
                (ArrayUniform u64 LLHDDFG)
                (Array LLHDVecValue)
                (Struct LLHDVecValue)
                (Not LLHDDFG)
                (Neg LLHDDFG)
                (Add LLHDDFG LLHDDFG)
                (Sub LLHDDFG LLHDDFG)
                (And LLHDDFG LLHDDFG)
                (Or LLHDDFG LLHDDFG)
                (Xor LLHDDFG LLHDDFG)
                (Smul LLHDDFG LLHDDFG)
                (Sdiv LLHDDFG LLHDDFG)
                (Smod LLHDDFG LLHDDFG)
                (Srem LLHDDFG LLHDDFG)
                (Umul LLHDDFG LLHDDFG)
                (Udiv LLHDDFG LLHDDFG)
                (Umod LLHDDFG LLHDDFG)
                (Urem LLHDDFG LLHDDFG)
                (Eq LLHDDFG LLHDDFG)
                (Neq LLHDDFG LLHDDFG)
                (Slt LLHDDFG LLHDDFG)
                (Sgt LLHDDFG LLHDDFG)
                (Sle LLHDDFG LLHDDFG)
                (Sge LLHDDFG LLHDDFG)
                (Ult LLHDDFG LLHDDFG)
                (Ugt LLHDDFG LLHDDFG)
                (Ule LLHDDFG LLHDDFG)
                (Uge LLHDDFG LLHDDFG)
                (Shl LLHDDFG LLHDDFG LLHDDFG)
                (Shr LLHDDFG LLHDDFG LLHDDFG)
                (Mux LLHDDFG LLHDDFG)
                (Reg LLHDVecValue LLHDVecRegMode)
                (InsField LLHDDFG LLHDDFG u64 u64)
                (InsSlice LLHDDFG LLHDDFG u64 u64)
                (ExtField LLHDDFG LLHDDFG u64 u64)
                (ExtSlice LLHDDFG LLHDDFG u64 u64)
                (Con LLHDDFG LLHDDFG)
                (Del LLHDDFG LLHDDFG LLHDDFG)
                (Call LLHDExtUnit u64 LLHDVecValue)
                (Inst LLHDExtUnit u64 LLHDVecValue)
                (Sig LLHDDFG)
                (Prb LLHDDFG)
                (Drv LLHDDFG LLHDDFG LLHDDFG)
                (DrvCond LLHDDFG LLHDDFG LLHDDFG LLHDDFG)
                (Var LLHDDFG)
                (Ld LLHDDFG)
                (St LLHDDFG LLHDDFG)
                (Halt)
                (Ret)
                (RetValue LLHDDFG)
                (Phi LLHDVecValue LLHDVecBlock)
                (Br LLHDBlock)
                (BrCond LLHDDFG LLHDBlock LLHDBlock)
                (Wait LLHDBlock LLHDVecValue)
                (WaitTime LLHDBlock LLHDVecValue)
                (LLHDUnit LLHDDFG)
            )
        "#
        );
    }
}
