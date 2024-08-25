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
    use llhd::table::TableKey;

    use super::*;
    use crate::llhd::inst::iterate_unit_value_defs;

    #[test]
    fn build_unit_component() {
        let input = indoc::indoc! {"
            proc %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %in3_prb = prb i1$ %in2
                %and1 = and i1 %in1_prb, %in2_prb
                %and2 = and i1 %in3_prb, %and1
                drv i1$ %out1, %and2, %epsilon
                wait %init for %epsilon
            }

            entity @top () -> () {
                %top_input1 = const i1 0
                %in1 = sig i1 %top_input1
                %top_input2 = const i1 1
                %in2 = sig i1 %top_input2
                %top_input3 = const i1 1
                %in3 = sig i1 %top_input3
                %top_out1 = const i1 0
                %out1 = sig i1 %top_out1
                inst %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1)
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
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
    fn llhd_egglog_dfg_expression_tree() {
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
}
