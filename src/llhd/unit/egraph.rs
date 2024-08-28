use std::collections::VecDeque;

use egglog::ast::{Action, Expr, GenericExpr, Literal, Symbol};
use itertools::Itertools;
use llhd::ir::prelude::*;

use crate::llhd::{LLHDEGraph, LLHDUtils};

type ExprList = Vec<Expr>;
type ValueStack = VecDeque<Value>;

impl LLHDEGraph {
    pub(crate) fn unit_symbol(unit: &Unit<'_>) -> Symbol {
        let unit_name = unit.name().to_string().replace(&['@', '%', ','][..], "");
        Symbol::new(unit_name)
    }

    pub(crate) fn from_unit(unit: &Unit<'_>) -> Action {
        let insts = LLHDUtils::iterate_unit_insts(unit).collect_vec();
        let root_inst_data = &unit[insts
            .last()
            .expect("Empty Unit can't construct a valid Egglog Expr.")
            .1];
        let root_inst_expr = Self::inst_expr(unit, root_inst_data);
        let unit_expr = GenericExpr::call(Self::unit_root_variant_symbol(), vec![root_inst_expr]);
        Action::Let((), Self::unit_symbol(unit), unit_expr)
    }

    fn traverse_bottom_up(expr: &Expr) {
        match expr {
            GenericExpr::Lit(_span, _literal) => {}
            GenericExpr::Var(_span, _symbol) => {
                // Leaf node, apply the function
                // f(self);
            }
            GenericExpr::Call(_span, _symbol, args) => {
                // Traverse child nodes first (bottom-up)
                for _arg in args {
                    Self::traverse_bottom_up(expr);
                }
                // Apply the function to the current node after children
                // f(self);
            }
        }
    }

    fn process_expr(expr: &Expr, expr_list: &mut ExprList) {
        match expr {
            GenericExpr::Lit(_span, literal) => {
                // Do nothing for literals, or handle them as needed
                expr_list.push(Expr::Lit((), literal.to_owned()));
            }
            GenericExpr::Var(_span, symbol) => {
                // Process the leaf node (Var) here
                // process_leaf(symbol)
                expr_list.push(Expr::Var((), symbol.to_owned()));
            }
            GenericExpr::Call(_, symbol, dependencies) => {
                // First, process all dependencies (bottom-up traversal)
                for dep in dependencies {
                    Self::process_expr(dep, expr_list);
                }
                // Then, process the current Call node
                // Here you can add logic to handle the current Call node if needed
                if Self::get_symbol_opcode(symbol).is_some() {
                    expr_list.push(Expr::Call((), symbol.to_owned(), vec![]));
                }
            }
        }
    }

    fn process_expr_list(
        expr_list: ExprList,
        value_stack: &mut ValueStack,
        unit_builder: &mut UnitBuilder,
    ) {
        for expr in expr_list {
            match expr {
                GenericExpr::Lit(_span, literal) => match literal {
                    Literal::Int(_value) => {
                        value_stack.push_back(Self::expr_value_data(&literal));
                    }
                    _ => {}
                },
                GenericExpr::Var(_span, _symbol) => {}
                GenericExpr::Call(_, symbol, _dependencies) => match Self::symbol_opcode(symbol) {
                    Opcode::Or => {
                        let arg2_value = value_stack
                            .pop_back()
                            .expect("Stack empty despite still trying to process operation.");
                        let arg1_value = value_stack
                            .pop_back()
                            .expect("Stack empty despite still trying to process operation.");
                        value_stack.push_back(unit_builder.ins().or(arg1_value, arg2_value));
                    }
                    Opcode::And => {
                        let arg2_value = value_stack
                            .pop_back()
                            .expect("Stack empty despite still trying to process operation.");
                        let arg1_value = value_stack
                            .pop_back()
                            .expect("Stack empty despite still trying to process operation.");
                        value_stack.push_back(unit_builder.ins().and(arg1_value, arg2_value));
                    }
                    Opcode::ConstTime => {
                        let _arg1_value = value_stack
                            .pop_back()
                            .expect("Stack empty despite still trying to process operation.");
                        // value_stack.push_back(unit_builder.ins().const_time(arg1_value));
                    }
                    Opcode::Drv => {
                        let arg3_value = value_stack
                            .pop_back()
                            .expect("Stack empty despite still trying to process operation.");
                        let arg2_value = value_stack
                            .pop_back()
                            .expect("Stack empty despite still trying to process operation.");
                        let arg1_value = value_stack
                            .pop_back()
                            .expect("Stack empty despite still trying to process operation.");
                        unit_builder.ins().drv(arg1_value, arg2_value, arg3_value);
                    }
                    _ => {}
                },
            }
        }
        // let v1 = builder.ins().const_int((1, 0));
        // let v2 = builder.ins().const_int((1, 1));
        // let _v3 = builder.ins().add(v1, v2);
    }

    pub(crate) fn to_unit(
        unit_expr: Expr,
        unit_kind: UnitKind,
        unit_name: UnitName,
        unit_sig: Signature,
    ) -> UnitData {
        let mut unit_data = UnitData::new(unit_kind, unit_name, unit_sig);
        let mut unit_builder = UnitBuilder::new_anonymous(&mut unit_data);
        let mut expr_list: ExprList = Default::default();

        Self::process_expr(&unit_expr, &mut expr_list);
        let _root_expr = expr_list.pop();

        let mut expr_stack: ValueStack = Default::default();
        Self::process_expr_list(expr_list, &mut expr_stack, &mut unit_builder);

        unit_data
    }
}

#[cfg(test)]
mod tests {
    use egglog::ast::{GenericCommand, GenericExpr, GenericRunConfig, GenericSchedule, Symbol};
    use egglog::TermDag;
    use llhd::table::TableKey;

    use super::*;

    #[test]
    fn llhd_egglog_dfg_expression_tree1() {
        let unit_data = utilities::build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let insts = LLHDUtils::iterate_unit_insts(&unit).collect_vec();
        let _value_refs = LLHDUtils::iterate_unit_value_defs(&unit).collect_vec();

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

        let egglog_expr = LLHDEGraph::from_unit(&unit);
        let expected_str = utilities::trim_whitespace(indoc::indoc! {"
            (let 0 (LLHDUnit (Add
                (Add
                    (ConstInt \"i1 0\")
                    (ConstInt \"i1 1\"))
                (Prb (Value 2)))))
        "});
        assert_eq!(
            expected_str,
            egglog_expr.to_string(),
            "Generated LLHD Egglog expression doesn't match expected value."
        );
    }

    #[test]
    fn llhd_egglog_dfg_expression_tree2() {
        let module = utilities::load_llhd_module("2and_1or.llhd");
        let units = LLHDUtils::iterate_unit_ids(&module).collect_vec();
        let unit = module.unit(*units.first().unwrap());
        let insts = LLHDUtils::iterate_unit_insts(&unit).collect_vec();
        let _value_refs = LLHDUtils::iterate_unit_value_defs(&unit).collect_vec();

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

        let egglog_expr = LLHDEGraph::from_unit(&unit);
        let expected_str = utilities::trim_whitespace(indoc::indoc! {"
            (let test_entity (LLHDUnit (Drv
                (Value 4) (Or
                    (And (Value 0) (Value 1))
                    (And (Value 2) (Value 3)))
                (ConstTime \"0s 1e\"))))
        "});
        assert_eq!(
            expected_str,
            egglog_expr.to_string(),
            "Generated LLHD Egglog expression doesn't match expected value."
        );
    }

    #[test]
    #[should_panic]
    fn llhd_testbench_egglog_program() {
        let mut test_module = utilities::load_llhd_module("2and_1or_common.llhd");
        let test_unit_id = LLHDUtils::iterate_unit_ids(&test_module).collect_vec()[0];
        let test_unit_kind = test_module.unit(test_unit_id).kind();
        let test_unit_name = test_module.unit(test_unit_id).name().to_owned();
        let test_unit_sig = test_module.unit(test_unit_id).sig().to_owned();
        let rewrite_unit = |module: &Module,
                            unit_kind: UnitKind,
                            unit_name: UnitName,
                            unit_sig: Signature| {
            let egraph_info = utilities::load_egraph("llhd_div_extract.egg");
            let mut egraph = egraph_info.0;
            assert_eq!(
                0,
                egraph.num_tuples(),
                "There should be 0 facts initially in the egraph."
            );

            let test_unit = module.unit(test_unit_id);
            let egglog_expr = LLHDEGraph::from_unit(&test_unit);
            let egraph_run_facts = egraph.run_program(vec![GenericCommand::Action(egglog_expr)]);
            assert!(egraph_run_facts.is_ok(), "EGraph failed to add facts.");
            assert!(
                egraph
                    .get_overall_run_report()
                    .num_matches_per_rule
                    .values()
                    .next()
                    .is_none(),
                "There should be no matches yet, as the rule schedule hasn't run yet."
            );

            assert_eq!(
                11,
                egraph.num_tuples(),
                "There should be 11 facts remaining in the egraph."
            );

            let div_extract_ruleset_symbol = Symbol::new("div-ext");
            let div_extract_schedule = GenericRunConfig::<Symbol, Symbol, ()> {
                ruleset: div_extract_ruleset_symbol,
                until: None,
            };
            let schedule_cmd =
                GenericCommand::RunSchedule(GenericSchedule::Run(div_extract_schedule).saturate());
            let egraph_run_schedule = egraph.run_program(vec![schedule_cmd]);
            assert!(
                egraph_run_schedule.is_ok(),
                "EGraph failed to run schedule."
            );
            assert_eq!(
                13,
                egraph.num_tuples(),
                "There should be 13 facts remaining in the egraph(new 'And', new 'Or' nodes)."
            );

            let egraph_run_rules_matches = egraph
                .get_overall_run_report()
                .num_matches_per_rule
                .values()
                .next()
                .unwrap();
            assert_eq!(
                1, *egraph_run_rules_matches,
                "There should be 1 match for divisor extraction rewrite rule."
            );

            let test_entity_symbol = Symbol::new("test_entity");
            let extract_cmd = GenericCommand::QueryExtract {
                variants: 0,
                expr: GenericExpr::Var((), test_entity_symbol),
            };
            let egraph_extract_expr = egraph.run_program(vec![extract_cmd]);
            assert!(
                egraph_extract_expr.is_ok(),
                "EGraph failed to extract expression."
            );

            let mut extracted_termdag = TermDag::default();
            let (unit_sort, test_unit_symbol_value) = egraph
                .eval_expr(&GenericExpr::Var((), test_entity_symbol))
                .unwrap();
            let (_unit_cost, unit_term) =
                egraph.extract(test_unit_symbol_value, &mut extracted_termdag, &unit_sort);
            let extracted_expr = extracted_termdag.term_to_expr(&unit_term);
            assert!(
                matches!(extracted_expr, GenericExpr::Call { .. }),
                "Top level expression should be a call."
            );
            assert_eq!(
                extracted_expr.to_string(),
                "(LLHDUnit (Drv (Value 3) (And (Or (Value 0) (Value 2)) (Value 1)) (ConstTime \
                 \"0s 1e\")))"
            );
            // Processing Call(Call): "LLHDUnit"
            // Processing Call(Call): "Drv"
            // Processing Call(Call): "Value"
            // Processing Literal(Lit): Int(3)
            // Processing Call(Call): "And"
            // Processing Call(Call): "Or"
            // Processing Call(Call): "Value"
            // Processing Literal(Lit): Int(0)
            // Processing Call(Call): "Value"
            // Processing Literal(Lit): Int(2)
            // Processing Call(Call): "Value"
            // Processing Literal(Lit): Int(1)
            // Processing Call(Call): "ConstTime"
            // Processing Literal(Lit): String("0s 1e")
            LLHDEGraph::to_unit(extracted_expr, unit_kind, unit_name, unit_sig)
        };
        test_module[test_unit_id] =
            rewrite_unit(&test_module, test_unit_kind, test_unit_name, test_unit_sig);
        let new_unit_data = test_module.unit(test_unit_id);
        let new_unit_insts = new_unit_data.all_insts().collect_vec();
        assert_eq!(
            5,
            new_unit_insts.len(),
            "There should be 5 Insts in rewritten Unit."
        );
    }
}
