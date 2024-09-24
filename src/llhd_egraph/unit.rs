use std::collections::VecDeque;

use egglog::ast::{
    Action, Command, Expr, GenericCommand, GenericExpr, Literal, Symbol, Variant, DUMMY_SPAN,
};
use egglog::sort::{Sort, StringSort, U64Sort};
use itertools::Itertools;
use llhd::ir::prelude::*;
use llhd::{IntValue, TimeValue};
use rayon::iter::ParallelIterator;

use crate::egraph::egglog_names::*;
use crate::egraph::EgglogCommandList;
use crate::llhd::LLHDUtils;
use crate::llhd_egraph::datatype::*;
use crate::llhd_egraph::egglog_names::*;
use crate::llhd_egraph::inst::*;

#[derive(Debug, Clone, Default)]
pub struct LLHDEgglogFacts(pub(in crate::llhd_egraph) EgglogCommandList);

impl LLHDEgglogFacts {
    pub fn from_module(module: &Module) -> Self {
        Self(
            module
                .par_units()
                .map(|unit| GenericCommand::Action(from_unit(&unit)))
                .collect(),
        )
    }

    pub fn from_unit(unit: &Unit) -> Self {
        Self(vec![GenericCommand::Action(from_unit(unit))])
    }
}

impl From<LLHDEgglogFacts> for EgglogCommandList {
    fn from(llhd_facts: LLHDEgglogFacts) -> Self {
        llhd_facts.0
    }
}

type ExprFIFO = VecDeque<Expr>;
type ValueStack = VecDeque<Value>;
type IntValueStack = VecDeque<IntValue>;
type TimeValueStack = VecDeque<TimeValue>;

const UNIT_LET_STMT_PREFIX: &str = "unit_";

fn unit_symbol(unit: &Unit<'_>) -> Symbol {
    let mut unit_name = unit.name().to_string().replace(&['@', '%', ','][..], "");
    unit_name.insert_str(0, UNIT_LET_STMT_PREFIX);
    Symbol::new(unit_name)
}

fn from_unit(unit: &Unit<'_>) -> Action {
    let insts = LLHDUtils::iterate_unit_insts(unit).collect_vec();
    let root_inst_data = &unit[insts
        .last()
        .expect("Empty Unit can't construct a valid Egglog Expr.")
        .1];
    let root_inst_expr = inst_expr(unit, root_inst_data);
    let unit_expr = GenericExpr::Call(
        DUMMY_SPAN.clone(),
        unit_root_variant_symbol(),
        vec![root_inst_expr],
    );
    Action::Let(DUMMY_SPAN.clone(), unit_symbol(unit), unit_expr)
}

fn process_expr(expr: &Expr, expr_fifo: &mut ExprFIFO) {
    match expr {
        GenericExpr::Lit(_span, literal) => {
            expr_fifo.push_front(Expr::Lit(DUMMY_SPAN.clone(), literal.to_owned()));
        }
        GenericExpr::Var(_span, symbol) => {
            expr_fifo.push_front(Expr::Var(DUMMY_SPAN.clone(), symbol.to_owned()));
        }
        GenericExpr::Call(_, symbol, dependencies) => {
            if opcode::get_symbol_opcode(symbol).is_some() {
                expr_fifo.push_front(Expr::Call(DUMMY_SPAN.clone(), symbol.to_owned(), vec![]));
            }
            for dep in dependencies {
                process_expr(dep, expr_fifo);
            }
        }
    }
}

fn process_expr_fifo(
    expr_fifo: ExprFIFO,
    value_stack: &mut ValueStack,
    _int_value_stack: &mut IntValueStack,
    time_value_stack: &mut TimeValueStack,
    unit_builder: &mut UnitBuilder,
) {
    for expr in expr_fifo {
        match expr {
            GenericExpr::Lit(_span, literal) => match literal {
                Literal::Int(_value) => {
                    value_stack.push_back(expr_value_data(&literal));
                }
                Literal::String(_value) => {
                    time_value_stack.push_back(expr_time_value(&literal));
                }
                _ => {}
            },
            GenericExpr::Var(_span, _symbol) => {}
            GenericExpr::Call(_, symbol, _dependencies) => {
                if let Some(opcode) = opcode::get_symbol_opcode(&symbol) {
                    match opcode {
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
                            let arg1_value = time_value_stack
                                .pop_back()
                                .expect("Stack empty despite still trying to process operation.");
                            value_stack.push_back(unit_builder.ins().const_time(arg1_value));
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
                            let _ = unit_builder.ins().drv(arg1_value, arg2_value, arg3_value);
                        }
                        _ => {
                            println!("Unknown opcode.");
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn to_unit(
    unit_expr: Expr,
    unit_kind: UnitKind,
    unit_name: UnitName,
    unit_sig: Signature,
) -> UnitData {
    let mut unit_data = UnitData::new(unit_kind, unit_name, unit_sig);
    let mut unit_builder = UnitBuilder::new_anonymous(&mut unit_data);
    let mut expr_fifo: ExprFIFO = Default::default();

    process_expr(&unit_expr, &mut expr_fifo);

    let mut value_stack: ValueStack = Default::default();
    let mut int_value_stack: IntValueStack = Default::default();
    let mut time_value_stack: TimeValueStack = Default::default();

    process_expr_fifo(
        expr_fifo,
        &mut value_stack,
        &mut int_value_stack,
        &mut time_value_stack,
        &mut unit_builder,
    );

    unit_data
}

fn ty() -> Command {
    let void_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_VOID_FIELD),
        types: vec![],
        cost: None,
    };
    let time_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_TIME_FIELD),
        types: vec![],
        cost: None,
    };
    let u64_sort = U64Sort::new(EGGLOG_U64_SORT.into());
    let int_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_INT_FIELD),
        types: vec![u64_sort.name()],
        cost: None,
    };
    let enum_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_ENUM_FIELD),
        types: vec![u64_sort.name()],
        cost: None,
    };
    let pointer_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_POINTER_FIELD),
        types: vec![LLHD_TYPE_DATATYPE.into()],
        cost: None,
    };
    let signal_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_SIGNAL_FIELD),
        types: vec![LLHD_TYPE_DATATYPE.into()],
        cost: None,
    };
    let array_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_ARRAY_FIELD),
        types: vec![u64_sort.name(), LLHD_TYPE_DATATYPE.into()],
        cost: None,
    };
    let struct_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_STRUCT_FIELD),
        types: vec![],
        cost: None,
    };
    let func_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_FUNC_FIELD),
        types: vec![LLHD_TYPE_DATATYPE.into()],
        cost: None,
    };
    let entity_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TYPE_ENTITY_FIELD),
        types: vec![],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_TYPE_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: symbol,
        variants: vec![
            void_variant,
            time_variant,
            int_variant,
            enum_variant,
            pointer_variant,
            signal_variant,
            array_variant,
            struct_variant,
            func_variant,
            entity_variant,
        ],
    }
}

fn vec_ty_sort() -> Command {
    let ty_sort_symbol = Symbol::new(LLHD_VEC_TYPE_DATATYPE);
    let symbol_vec = Symbol::new(EGGLOG_VEC_SORT);
    let ty_sort = Symbol::new(LLHD_TYPE_DATATYPE);
    let ty_expr = Expr::Var(DUMMY_SPAN.clone(), ty_sort);
    Command::Sort(
        DUMMY_SPAN.clone(),
        ty_sort_symbol,
        Some((symbol_vec, vec![ty_expr])),
    )
}

fn unit_kind_sort() -> Command {
    let entity_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_UNIT_ENTITY_FIELD),
        types: vec![],
        cost: None,
    };
    let function_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_UNIT_FUNCTION_FIELD),
        types: vec![],
        cost: None,
    };
    let process_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_UNIT_PROCESS_FIELD),
        types: vec![],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_UNIT_KIND_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: symbol,
        variants: vec![entity_variant, function_variant, process_variant],
    }
}

fn value() -> Command {
    let u64_sort = U64Sort::new(EGGLOG_U64_SORT.into());
    let ty_datatype = Symbol::new(LLHD_TYPE_DATATYPE);
    let value_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_VALUE_FIELD),
        types: vec![ty_datatype, u64_sort.name()],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_VALUE_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: symbol,
        variants: vec![value_variant],
    }
}

fn int_value() -> Command {
    let u64_sort = U64Sort::new(EGGLOG_U64_SORT.into());
    let int_value_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_INT_VALUE_FIELD),
        types: vec![u64_sort.name()],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_INT_VALUE_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: symbol,
        variants: vec![int_value_variant],
    }
}

fn time_value() -> Command {
    let u64_sort = U64Sort::new(EGGLOG_U64_SORT.into());
    let time_value_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_TIME_VALUE_FIELD),
        types: vec![u64_sort.name()],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_TIME_VALUE_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: symbol,
        variants: vec![time_value_variant],
    }
}

fn reg_mode() -> Command {
    let symbol = Symbol::new(LLHD_REGMODE_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: symbol,
        variants: vec![
            Variant {
                span: DUMMY_SPAN.clone(),
                name: Symbol::new(LLHD_REGMODE_FIELD_LOW),
                types: vec![],
                cost: None,
            },
            Variant {
                span: DUMMY_SPAN.clone(),
                name: Symbol::new(LLHD_REGMODE_FIELD_HIGH),
                types: vec![],
                cost: None,
            },
            Variant {
                span: DUMMY_SPAN.clone(),
                name: Symbol::new(LLHD_REGMODE_FIELD_RISE),
                types: vec![],
                cost: None,
            },
            Variant {
                span: DUMMY_SPAN.clone(),
                name: Symbol::new(LLHD_REGMODE_FIELD_FALL),
                types: vec![],
                cost: None,
            },
            Variant {
                span: DUMMY_SPAN.clone(),
                name: Symbol::new(LLHD_REGMODE_FIELD_BOTH),
                types: vec![],
                cost: None,
            },
        ],
    }
}

fn vec_value_sort() -> Command {
    let vec_sort_symbol = Symbol::new(LLHD_VEC_VALUE_DATATYPE);
    let symbol_vec = Symbol::new(EGGLOG_VEC_SORT);
    let value_sort = Symbol::new(LLHD_VALUE_DATATYPE);
    let value_expr = Expr::Var(DUMMY_SPAN.clone(), value_sort);
    Command::Sort(
        DUMMY_SPAN.clone(),
        vec_sort_symbol,
        Some((symbol_vec, vec![value_expr])),
    )
}

fn vec_regmode_sort() -> Command {
    let vec_sort_symbol = Symbol::new(LLHD_VEC_REGMODE_DATATYPE);
    let symbol_vec = Symbol::new(EGGLOG_VEC_SORT);
    let regmode_datatype = Symbol::new(LLHD_REGMODE_DATATYPE);
    let regmode_expr = Expr::Var(DUMMY_SPAN.clone(), regmode_datatype);
    Command::Sort(
        DUMMY_SPAN.clone(),
        vec_sort_symbol,
        Some((symbol_vec, vec![regmode_expr])),
    )
}

fn block() -> Command {
    let u64_sort = U64Sort::new(EGGLOG_U64_SORT.into());
    let block_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_BLOCK_FIELD),
        types: vec![u64_sort.name()],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_BLOCK_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: symbol,
        variants: vec![block_variant],
    }
}

fn vec_block() -> Command {
    let vec_sort_symbol = Symbol::new(LLHD_VEC_BLOCK_DATATYPE);
    let symbol_vec = Symbol::new(EGGLOG_VEC_SORT);
    let vec_block_datatype = U64Sort::new(LLHD_BLOCK_DATATYPE.into());
    let vec_block_expr = Expr::Var(DUMMY_SPAN.clone(), vec_block_datatype.name());
    Command::Sort(
        DUMMY_SPAN.clone(),
        vec_sort_symbol,
        Some((symbol_vec, vec![vec_block_expr])),
    )
}

fn ext_unit() -> Command {
    let u64_sort = U64Sort::new(EGGLOG_U64_SORT.into());
    let ext_unit_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_EXT_UNIT_FIELD),
        types: vec![u64_sort.name()],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_EXT_UNIT_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: symbol,
        variants: vec![ext_unit_variant],
    }
}

fn unit() -> Command {
    let u64_sort = U64Sort::new(EGGLOG_U64_SORT.into());
    let string_sort = StringSort::new(EGGLOG_STRING_SORT.into());
    let unit_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_UNIT_FIELD),
        types: vec![
            u64_sort.name(),
            LLHD_UNIT_KIND_DATATYPE.into(),
            string_sort.name(),
            LLHD_VEC_VALUE_DATATYPE.into(),
            LLHD_VEC_VALUE_DATATYPE.into(),
            LLHD_DFG_DATATYPE.into(),
        ],
        cost: None,
    };
    let unit_decl_variant = Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_UNIT_DECL_FIELD),
        types: vec![
            u64_sort.name(),
            LLHD_UNIT_KIND_DATATYPE.into(),
            string_sort.name(),
            LLHD_VEC_VALUE_DATATYPE.into(),
            LLHD_VEC_VALUE_DATATYPE.into(),
        ],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_UNIT_DFG_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: symbol,
        variants: vec![unit_variant, unit_decl_variant],
    }
}

pub(in crate::llhd_egraph) fn unit_types() -> EgglogCommandList {
    vec![
        ty(),
        vec_ty_sort(),
        unit_kind_sort(),
        value(),
        vec_value_sort(),
        block(),
        vec_block(),
        ext_unit(),
        time_value(),
        reg_mode(),
        vec_regmode_sort(),
    ]
}

pub(in crate::llhd_egraph) fn dfg() -> EgglogCommandList {
    vec![unit()]
}

#[cfg(test)]
mod tests {
    use egglog::ast::{
        GenericAction, GenericCommand, GenericExpr, GenericRunConfig, GenericSchedule, Symbol,
    };
    use egglog::{EGraph, TermDag};
    use llhd::ir::InstData;
    use llhd::table::TableKey;

    use super::*;

    #[test]
    fn build_egglog_program_from_unit() {
        let unit_data = utilities::build_entity_alpha(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let egglog_facts = LLHDEgglogFacts::from_unit(&unit);
        assert_eq!(
            1,
            egglog_facts.0.len(),
            "There should be 1 fact in program."
        );
        if let GenericCommand::Action(let_action) = &egglog_facts.0[0] {
            if let GenericAction::Let(_dummy, let_stmt_symbol, _let_stmt) = let_action {
                assert_eq!(
                    "unit_0",
                    let_stmt_symbol.to_string(),
                    "Let Stmt should match UnitName"
                );
            };
        };
    }

    #[test]
    fn llhd_egglog_dfg_expression_tree1() {
        let unit_data = utilities::build_entity_alpha(UnitName::anonymous(0));
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

        let egglog_expr = from_unit(&unit);
        let expected_str = utilities::trim_expr_whitespace(indoc::indoc! {"
            (let unit_0 (LLHDUnit (Add
                (Add
                    (ConstInt \"i1 0\")
                    (ConstInt \"i1 1\"))
                (Prb (ValueRef _2)))))
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

        let egglog_expr = from_unit(&unit);
        let expected_str = utilities::trim_expr_whitespace(indoc::indoc! {"
            (let unit_test_entity (LLHDUnit (Drv
                (ValueRef _4) (Or
                    (And (ValueRef _0) (ValueRef _1))
                    (And (ValueRef _2) (ValueRef _3)))
                (ConstTime \"0s 1e\"))))
        "});
        assert_eq!(
            expected_str,
            egglog_expr.to_string(),
            "Generated LLHD Egglog expression doesn't match expected value."
        );
    }

    #[test]
    fn llhd_rewrite_egglog_program() {
        let mut test_module = utilities::load_llhd_module("2and_1or_common.llhd");
        let test_unit_id = LLHDUtils::iterate_unit_ids(&test_module).collect_vec()[0];
        let test_unit_kind = test_module.unit(test_unit_id).kind();
        let test_unit_name = test_module.unit(test_unit_id).name().to_owned();
        let test_unit_sig = test_module.unit(test_unit_id).sig().to_owned();
        let rewrite_unit =
            |module: &Module, unit_kind: UnitKind, unit_name: UnitName, unit_sig: Signature| {
                let llhd_dfg_sort = LLHDEgglogSorts::llhd_dfg();
                let mut egraph = EGraph::default();
                let _egraph_msgs_datatypes = egraph.run_program(llhd_dfg_sort.into());
                let _egraph_msgs_rules =
                    utilities::load_egraph_rewrite_rules("llhd_div_extract.egg", &mut egraph);
                assert_eq!(
                    0,
                    egraph.num_tuples(),
                    "There should be 0 facts initially in the egraph."
                );

                let module_facts = LLHDEgglogFacts::from_module(module);
                let egraph_run_facts = egraph.run_program(module_facts.into());
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
                let div_extract_schedule = GenericRunConfig::<Symbol, Symbol> {
                    ruleset: div_extract_ruleset_symbol,
                    until: None,
                };
                let schedule_cmd = GenericCommand::RunSchedule(GenericSchedule::Run(
                    DUMMY_SPAN.clone(),
                    div_extract_schedule,
                ));
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

                let test_entity_symbol = Symbol::new("unit_test_entity");
                let extract_cmd = GenericCommand::QueryExtract {
                    span: DUMMY_SPAN.clone(),
                    variants: 0,
                    expr: GenericExpr::Var(DUMMY_SPAN.clone(), test_entity_symbol),
                };
                if let Err(egraph_extract_expr_msg) = egraph.run_program(vec![extract_cmd]) {
                    panic!(
                        "EGraph failed to extract expression. ERROR: {:?}",
                        egraph_extract_expr_msg
                    );
                }

                let mut extracted_termdag = TermDag::default();
                let (unit_sort, test_unit_symbol_value) = egraph
                    .eval_expr(&GenericExpr::Var(DUMMY_SPAN.clone(), test_entity_symbol))
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
                    "(LLHDUnit (Drv (ValueRef 3) (And (Or (ValueRef 0) (ValueRef 2)) (ValueRef \
                     1)) (ConstTime \"0s 1e\")))"
                );
                to_unit(extracted_expr, unit_kind, unit_name, unit_sig)
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
        let inst_const_time_id = new_unit_insts[0];
        let inst_const_time_data = new_unit_data[inst_const_time_id].clone();
        assert_eq!(
            Opcode::ConstTime,
            inst_const_time_data.opcode(),
            "First Inst should be `const time`."
        );
        let inst_and1_id = new_unit_insts[1];
        let inst_and1_data = new_unit_data[inst_and1_id].clone();
        assert_eq!(
            Opcode::Or,
            inst_and1_data.opcode(),
            "Second Inst should be `or`."
        );
        let inst_or1_id = new_unit_insts[2];
        let inst_or1_data = new_unit_data[inst_or1_id].clone();
        assert_eq!(
            Opcode::And,
            inst_or1_data.opcode(),
            "Third Inst should be `And`."
        );
        let inst_drv1_id = new_unit_insts[3];
        let inst_drv1_data = new_unit_data[inst_drv1_id].clone();
        assert_eq!(
            Opcode::Drv,
            inst_drv1_data.opcode(),
            "Fourth Inst should be `drv`."
        );
        let inst_null_id = new_unit_insts[4];
        let inst_null_data = new_unit_data[inst_null_id].clone();
        assert!(
            matches!(inst_null_data, InstData::Nullary { .. }),
            "Fifth Inst should be Null instruction(doesn't actually exist)."
        );
    }

    #[test]
    fn llhd_egglog_value_datatypes() {
        let value_datatype = value();
        let expected_str = "(datatype LLHDValue (Value LLHDTy u64))".to_owned();
        assert_eq!(
            expected_str,
            value_datatype.to_string(),
            "Datatype should be named 'LLHDValue' and should have 1 field named (Value u64)."
        );
        let int_value_datatype = int_value();
        let int_expected_str = "(datatype LLHDIntValue (IntValue u64))".to_owned();
        assert_eq!(
            int_expected_str,
            int_value_datatype.to_string(),
            "Datatype should be named 'LLHDIntValue' and should have 1 field named (IntValue u64)."
        );
        let time_value_datatype = time_value();
        let time_expected_str = "(datatype LLHDTimeValue (TimeValue u64))".to_owned();
        assert_eq!(
            time_expected_str,
            time_value_datatype.to_string(),
            "Datatype should be named 'LLHDTimeValue' and should have 1 field named (TimeValue \
             u64)."
        );
        let reg_mode_datatype = reg_mode();
        let reg_mode_expected_str = utilities::trim_expr_whitespace(indoc::indoc! {"
            (datatype LLHDRegMode
                (Low)
                (High)
                (Rise)
                (Fall)
                (Both))
        "});
        assert_eq!(
            reg_mode_expected_str,
            reg_mode_datatype.to_string(),
            "Datatype should be named 'LLHDRegMode' and should have 5 field names."
        );
    }

    #[test]
    fn llhd_egglog_vec_sort() {
        let vec_sort = vec_value_sort();
        let expected_str = "(sort LLHDVecValue (Vec LLHDValue))".to_owned();
        assert_eq!(
            expected_str,
            vec_sort.to_string(),
            "Sort should be named 'LLHDVecValue' and should have 1 field named (Vec u64)."
        );
        let vec_regmode_sort = vec_regmode_sort();
        let vec_regmode_expected_str = "(sort LLHDVecRegMode (Vec LLHDRegMode))".to_owned();
        assert_eq!(
            vec_regmode_expected_str,
            vec_regmode_sort.to_string(),
            "Sort should be named 'LLHDVecRegMode' and should have 1 field named (Vec \
             LLHDRegMode)."
        );
    }

    #[test]
    fn llhd_egglog_block_datatypes() {
        let block_datatype = block();
        let expected_str = "(datatype LLHDBlock (Block u64))".to_owned();
        assert_eq!(
            expected_str,
            block_datatype.to_string(),
            "Datatype should be named 'LLHDBlock' and should have 1 field named (Block u64)."
        );
    }

    #[test]
    fn llhd_egglog_vec_block_sort() {
        let block_datatype = vec_block();
        let expected_str = "(sort LLHDVecBlock (Vec LLHDBlock))".to_owned();
        assert_eq!(
            expected_str,
            block_datatype.to_string(),
            "Datatype should be named 'LLHDVecBlock' and should have 1 field named (Vec \
             LLHDBlock)."
        );
    }

    #[test]
    fn llhd_egglog_ext_unit_datatypes() {
        let ext_unit_datatype = ext_unit();
        let expected_str = "(datatype LLHDExtUnit (ExtUnit u64))".to_owned();
        assert_eq!(
            expected_str,
            ext_unit_datatype.to_string(),
            "Datatype should be named 'LLHDExtUnit' and should have 1 field named (ExtUnit u64)."
        );
    }
}
