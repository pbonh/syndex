use egglog::ast::{Action, Symbol};
use itertools::Itertools;
use llhd::ir::prelude::*;

use super::inst::inst_expr;
use super::LLHDUnitArg;
use crate::llhd::inst::iterate_unit_insts;

pub(crate) fn iterate_unit_ids(module: &Module) -> impl Iterator<Item = UnitId> + '_ {
    module.units().map(|unit| unit.id())
}

pub(crate) fn iterate_unit_input_arg_defs<'unit>(
    unit: &'unit Unit,
) -> impl Iterator<Item = LLHDUnitArg> + 'unit {
    unit.input_args().map(|arg| (unit.id(), arg))
}

pub(crate) fn iterate_unit_arg_defs<'unit>(
    unit: &'unit Unit,
) -> impl Iterator<Item = LLHDUnitArg> + 'unit {
    iterate_unit_input_arg_defs(unit)
        .map(|(_unit_id, arg)| arg)
        .chain(unit.output_args())
        .map(|arg| (unit.id(), arg))
}

#[derive(Debug)]
pub(crate) struct LLHDDFGExprTree;

impl LLHDDFGExprTree {
    pub(crate) fn from_unit(unit: &Unit<'_>) -> Action {
        let unit_symbol = Symbol::new(unit.name().to_string());
        let insts = iterate_unit_insts(unit).collect_vec();
        let root_inst_data = &unit[insts
            .last()
            .expect("Empty Unit can't construct a valid Egglog Expr.")
            .1];
        let unit_expr = inst_expr(unit, root_inst_data);
        Action::Let((), unit_symbol, unit_expr)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use egglog::ast::{GenericCommand, GenericRunConfig, GenericSchedule};
    use egglog::EGraph;
    use llhd::table::TableKey;

    use super::*;
    use crate::llhd::inst::iterate_unit_value_defs;

    fn load_llhd_module(filename: &str) -> Module {
        let mut llhd_module_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        llhd_module_file_path.push("resources/llhd");
        llhd_module_file_path.push(filename);
        let llhd_module_str: String = fs::read_to_string(llhd_module_file_path).unwrap();
        llhd::assembly::parse_module(llhd_module_str)
            .expect(&format!("Error loading module: {}", filename))
    }

    fn load_egraph(filename: &str) -> (EGraph, Vec<String>) {
        let mut egglog_program_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        egglog_program_file_path.push("resources/egglog");
        egglog_program_file_path.push(filename);
        let egglog_program_str: String = fs::read_to_string(egglog_program_file_path).unwrap();
        let mut egraph = EGraph::default();
        let msgs = egraph
            .parse_and_run_program(&egglog_program_str)
            .expect("Failure to run program on egraph.");
        (egraph, msgs)
    }

    #[test]
    fn build_unit_component() {
        let module = load_llhd_module("testbench_example1.llhd");
        let units = iterate_unit_ids(&module).collect_vec();
        assert_eq!(2, units.len(), "There should be 2 Units present in Module.");
        let first_unit = module.units().collect_vec()[0];
        let second_unit = module.units().collect_vec()[1];
        let first_unit_args = iterate_unit_arg_defs(&first_unit).collect_vec();
        assert_eq!(
            4,
            first_unit_args.len(),
            "There should be 4 args present in first unit."
        );
        let second_unit_args = iterate_unit_arg_defs(&second_unit).collect_vec();
        assert_eq!(
            0,
            second_unit_args.len(),
            "There should be 3 args present in second unit."
        );
    }

    fn build_entity(name: UnitName) -> UnitData {
        let mut sig = Signature::new();
        let _clk = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
        let _rst = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
        let inp = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
        let _oup = sig.add_output(llhd::signal_ty(llhd::int_ty(32)));
        let mut ent = UnitData::new(UnitKind::Entity, name, sig);
        {
            let mut builder = UnitBuilder::new_anonymous(&mut ent);
            let v1 = builder.ins().const_int((1, 0));
            let v2 = builder.ins().const_int((1, 1));
            let v3 = builder.ins().add(v1, v2);
            let inp = builder.unit().arg_value(inp);
            let inp = builder.ins().prb(inp);
            builder.ins().add(v3, inp);
        }
        Unit::new_anonymous(&ent).verify();
        ent
    }

    fn trim_whitespace(s: &str) -> String {
        // first attempt: allocates a vector and a string
        let words: Vec<_> = s.split_whitespace().collect();
        words.join(" ")
    }

    #[test]
    fn llhd_egglog_dfg_expression_tree1() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let insts = iterate_unit_insts(&unit).collect_vec();
        let _value_refs = iterate_unit_value_defs(&unit).collect_vec();

        let const_int_1_inst = insts[0];
        let const_int_1_inst_data = &unit[const_int_1_inst.1];
        assert_eq!(
            Opcode::ConstInt,
            const_int_1_inst_data.opcode(),
            "Inst should be Const Int."
        );
        let const_int_2_inst = insts[1];
        let const_int_2_inst_data = &unit[const_int_2_inst.1];
        assert_eq!(
            Opcode::ConstInt,
            const_int_2_inst_data.opcode(),
            "Inst should be Const Int."
        );
        let add1_inst = insts[2];
        let add1_inst_data = &unit[add1_inst.1];
        assert_eq!(Opcode::Add, add1_inst_data.opcode(), "Inst should be Add.");
        let prb1_inst = insts[3];
        let prb1_inst_data = &unit[prb1_inst.1];
        assert_eq!(Opcode::Prb, prb1_inst_data.opcode(), "Inst should be Prb.");
        let add2_inst = insts[4];
        let add2_inst_data = &unit[add2_inst.1];
        assert_eq!(Opcode::Add, add2_inst_data.opcode(), "Inst should be Add.");

        let egglog_expr = LLHDDFGExprTree::from_unit(&unit);
        let expected_str = trim_whitespace(indoc::indoc! {"
            (let %0 (Add
                (Add
                    (ConstInt (Value \"i1 0\"))
                    (ConstInt (Value \"i1 1\")))
                (Prb (Value 2))))
        "});
        assert_eq!(
            expected_str,
            egglog_expr.to_string(),
            "Generated LLHD Egglog expression doesn't match expected value."
        );
    }

    #[test]
    fn llhd_egglog_dfg_expression_tree2() {
        let module = load_llhd_module("2and_1or.llhd");
        let units = iterate_unit_ids(&module).collect_vec();
        let unit = module.unit(*units.first().unwrap());
        let insts = iterate_unit_insts(&unit).collect_vec();
        let _value_refs = iterate_unit_value_defs(&unit).collect_vec();

        let const_time_inst = insts[0];
        let const_time_inst_data = &unit[const_time_inst.1];
        assert_eq!(
            Opcode::ConstTime,
            const_time_inst_data.opcode(),
            "Inst should be Const Time."
        );
        let and1_inst = insts[1];
        let and1_inst_data = &unit[and1_inst.1];
        assert_eq!(Opcode::And, and1_inst_data.opcode(), "Inst should be And.");
        let and2_inst = insts[2];
        let and2_inst_data = &unit[and2_inst.1];
        assert_eq!(Opcode::And, and2_inst_data.opcode(), "Inst should be And.");
        let or1_inst = insts[3];
        let or1_inst_data = &unit[or1_inst.1];
        assert_eq!(Opcode::Or, or1_inst_data.opcode(), "Inst should be Or.");
        let drv_inst = insts[4];
        let drv_inst_data = &unit[drv_inst.1];
        assert_eq!(Opcode::Drv, drv_inst_data.opcode(), "Inst should be Drv.");

        let egglog_expr = LLHDDFGExprTree::from_unit(&unit);
        let expected_str = trim_whitespace(indoc::indoc! {"
            (let @test_entity (Drv
                (Value 4) (Or
                    (And (Value 0) (Value 1))
                    (And (Value 2) (Value 3)))
                (ConstTime (Value \"0s 1e\"))))
        "});
        assert_eq!(
            expected_str,
            egglog_expr.to_string(),
            "Generated LLHD Egglog expression doesn't match expected value."
        );
    }

    #[test]
    #[should_panic(expected = "EGraph failed to run schedule.")]
    fn llhd_testbench_egglog_program() {
        let egraph_info = load_egraph("llhd_div_extract.egg");
        let mut egraph = egraph_info.0;
        assert_eq!(
            0,
            egraph.num_tuples(),
            "There should be 0 facts initially in the egraph."
        );

        let module = load_llhd_module("2and_1or.llhd");
        let units = iterate_unit_ids(&module).collect_vec();
        let unit = module.unit(*units.first().unwrap());
        let egglog_expr = LLHDDFGExprTree::from_unit(&unit);
        let egraph_run_facts = egraph.run_program(vec![GenericCommand::Action(egglog_expr)]);
        assert!(egraph_run_facts.is_ok(), "EGraph failed to run schedule.");

        let div_extract_ruleset_symbol = Symbol::new("div-ext");
        let div_extract_schedule = GenericRunConfig::<Symbol, Symbol, ()> {
            ruleset: div_extract_ruleset_symbol,
            until: None,
        };
        let extract_cmd =
            GenericCommand::RunSchedule(GenericSchedule::Run(div_extract_schedule).saturate());
        let egraph_run_schedule = egraph.run_program(vec![extract_cmd]);
        assert!(
            egraph_run_schedule.is_ok(),
            "EGraph failed to run schedule."
        );
        assert_eq!(
            13,
            egraph.num_tuples(),
            "There should be 13 facts remaining in the egraph."
        );
    }
}
