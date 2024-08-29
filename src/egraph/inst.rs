use egglog::ast::{Command, Expr, GenericExpr, Literal, Symbol, Variant};
use egglog::sort::{I64Sort, Sort};
use lazy_static::lazy_static;
use llhd::ir::prelude::*;
use llhd::ir::{InstData, ValueData};
use llhd::table::TableKey;
use llhd::{IntValue, TimeValue};

use crate::egraph::datatype::*;
use crate::egraph::egglog_names::*;

pub(in crate::egraph) mod opcode;
use opcode::*;

lazy_static! {
    static ref LLHD_DFG_VARIANTS: Vec<Variant> = vec![
        value_ref_variant(),
        variant(Opcode::ConstInt, vec![EGGLOG_STRING_SORT]),
        variant(Opcode::ConstTime, vec![EGGLOG_STRING_SORT]),
        variant(Opcode::Alias, vec![LLHD_DFG_DATATYPE]),
        variant(
            Opcode::ArrayUniform,
            vec![EGGLOG_I64_SORT, LLHD_DFG_DATATYPE]
        ),
        variant(Opcode::Array, vec![LLHD_VEC_VALUE_DATATYPE]),
        variant(Opcode::Struct, vec![LLHD_VEC_VALUE_DATATYPE]),
        variant(Opcode::Not, vec![LLHD_DFG_DATATYPE]),
        variant(Opcode::Neg, vec![LLHD_DFG_DATATYPE]),
        variant(Opcode::Add, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Sub, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::And, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Or, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Xor, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Smul, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Sdiv, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Smod, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Srem, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Umul, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Udiv, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Umod, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Urem, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Eq, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Neq, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Slt, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Sgt, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Sle, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Sge, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Ult, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Ugt, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Ule, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(Opcode::Uge, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(
            Opcode::Shl,
            vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]
        ),
        variant(
            Opcode::Shr,
            vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]
        ),
        variant(Opcode::Mux, vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE]),
        variant(
            Opcode::Reg,
            vec![LLHD_VEC_VALUE_DATATYPE, LLHD_VEC_REGMODE_DATATYPE]
        ),
        variant(Opcode::Sig, vec![LLHD_DFG_DATATYPE]),
        variant(Opcode::Prb, vec![LLHD_DFG_DATATYPE]),
        variant(
            Opcode::Drv,
            vec![LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE, LLHD_DFG_DATATYPE],
        ),
        variant(Opcode::Wait, vec![EGGLOG_I64_SORT, LLHD_VEC_VALUE_DATATYPE]),
        unit_root_variant(),
    ];
    static ref LLHD_DFG_VARIANTS_COUNT: usize = LLHD_DFG_VARIANTS.len();
}

pub(in crate::egraph) fn value() -> Command {
    let i64_sort = I64Sort::new(EGGLOG_I64_SORT.into());
    let value_variant = Variant {
        name: Symbol::new(LLHD_VALUE_FIELD),
        types: vec![i64_sort.name()],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_VALUE_DATATYPE);
    Command::Datatype {
        name: symbol,
        variants: vec![value_variant],
    }
}

pub(in crate::egraph) fn int_value() -> Command {
    let i64_sort = I64Sort::new(EGGLOG_I64_SORT.into());
    let int_value_variant = Variant {
        name: Symbol::new(LLHD_INT_VALUE_FIELD),
        types: vec![i64_sort.name()],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_INT_VALUE_DATATYPE);
    Command::Datatype {
        name: symbol,
        variants: vec![int_value_variant],
    }
}

pub(in crate::egraph) fn time_value() -> Command {
    let i64_sort = I64Sort::new(EGGLOG_I64_SORT.into());
    let time_value_variant = Variant {
        name: Symbol::new(LLHD_TIME_VALUE_FIELD),
        types: vec![i64_sort.name()],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_TIME_VALUE_DATATYPE);
    Command::Datatype {
        name: symbol,
        variants: vec![time_value_variant],
    }
}

pub(in crate::egraph) fn reg_mode() -> Command {
    let symbol = Symbol::new(LLHD_REGMODE_DATATYPE);
    Command::Datatype {
        name: symbol,
        variants: vec![
            Variant {
                name: Symbol::new(LLHD_REGMODE_FIELD_LOW),
                types: vec![],
                cost: None,
            },
            Variant {
                name: Symbol::new(LLHD_REGMODE_FIELD_HIGH),
                types: vec![],
                cost: None,
            },
            Variant {
                name: Symbol::new(LLHD_REGMODE_FIELD_RISE),
                types: vec![],
                cost: None,
            },
            Variant {
                name: Symbol::new(LLHD_REGMODE_FIELD_FALL),
                types: vec![],
                cost: None,
            },
            Variant {
                name: Symbol::new(LLHD_REGMODE_FIELD_BOTH),
                types: vec![],
                cost: None,
            },
        ],
    }
}

pub(in crate::egraph) fn vec_sort() -> Command {
    let vec_sort_symbol = Symbol::new(LLHD_VEC_VALUE_DATATYPE);
    let symbol_vec = Symbol::new(EGGLOG_VEC_SORT);
    let i64_sort = I64Sort::new(EGGLOG_I64_SORT.into());
    let i64_expr = Expr::Var((), i64_sort.name());
    Command::Sort(vec_sort_symbol, Some((symbol_vec, vec![i64_expr])))
}

pub(in crate::egraph) fn vec_regmode_sort() -> Command {
    let vec_sort_symbol = Symbol::new(LLHD_VEC_REGMODE_DATATYPE);
    let symbol_vec = Symbol::new(EGGLOG_VEC_SORT);
    let regmode_datatype = I64Sort::new(LLHD_REGMODE_DATATYPE.into());
    let regmode_expr = Expr::Var((), regmode_datatype.name());
    Command::Sort(vec_sort_symbol, Some((symbol_vec, vec![regmode_expr])))
}

pub(in crate::egraph) fn block() -> Command {
    let i64_sort = I64Sort::new(EGGLOG_I64_SORT.into());
    let block_variant = Variant {
        name: Symbol::new(LLHD_BLOCK_FIELD),
        types: vec![i64_sort.name()],
        cost: None,
    };
    let symbol = Symbol::new(LLHD_BLOCK_DATATYPE);
    Command::Datatype {
        name: symbol,
        variants: vec![block_variant],
    }
}

pub(in crate::egraph) fn dfg() -> Command {
    let dfg_symbol = Symbol::new(LLHD_DFG_DATATYPE);
    Command::Datatype {
        name: dfg_symbol,
        variants: LLHD_DFG_VARIANTS.to_vec(),
    }
}

pub(in crate::egraph) fn cfg() -> Command {
    let _symbol = Symbol::new(LLHD_CFG_DATATYPE);
    todo!()
}

fn value_def_expr(value: impl TableKey) -> Expr {
    let converted_unsigned_num =
        i64::try_from(value.index()).expect("Out-of-bound value for u32 -> i64 conversion.");
    let converted_literal = Literal::Int(converted_unsigned_num);
    let literal_value = GenericExpr::lit(converted_literal);
    let llhd_value_datatype_symbol = Symbol::new(LLHD_VALUE_FIELD);
    GenericExpr::call(llhd_value_datatype_symbol, [literal_value])
}

fn value_data_expr(unit: &Unit<'_>, value_data: &ValueData) -> Expr {
    match value_data {
        ValueData::Inst { inst, .. } => inst_expr(unit, &unit[inst.to_owned()]),
        ValueData::Arg { arg, .. } => value_def_expr(arg.to_owned()),
        _ => panic!("Value type not supported."),
    }
}

pub(in crate::egraph) fn expr_value_data(literal: &Literal) -> Value {
    match literal {
        Literal::Int(value) => {
            Value::new(usize::try_from(*value).expect("Failure to convert from i64 to usize."))
        }
        _ => panic!("Non-Int Literal"),
    }
}

fn int_value_expr(int_value: IntValue) -> Expr {
    let converted_literal = Literal::String(int_value.to_string().into());
    GenericExpr::lit(converted_literal)
}

pub(super) fn expr_int_value(literal: &Literal) -> IntValue {
    match literal {
        Literal::Int(value) => IntValue::from_isize(
            64,
            isize::try_from(*value).expect("Failure to convert from i64 to isize."),
        ),
        _ => panic!("Non-Int Literal"),
    }
}

fn time_value_expr(time_value: TimeValue) -> Expr {
    let converted_literal = Literal::String(time_value.to_string().into());
    GenericExpr::lit(converted_literal)
}

pub(in crate::egraph) fn expr_time_value(_literal: &Literal) -> TimeValue {
    TimeValue::zero()
}

pub(in crate::egraph) fn inst_expr(unit: &Unit<'_>, inst_data: &InstData) -> Expr {
    let inst_symbol = opcode_symbol(inst_data.opcode());
    let mut children: Vec<Expr> = vec![];
    match inst_data {
        InstData::Binary { args, .. } => {
            let expr_left = value_data_expr(&unit, &unit[args[0]]);
            let expr_right = value_data_expr(&unit, &unit[args[1]]);
            children = vec![expr_left, expr_right];
        }
        InstData::Unary { args, .. } => {
            let value_data = &unit[args[0]];
            let expr_left = value_data_expr(&unit, value_data);
            children = vec![expr_left];
        }
        InstData::ConstInt { imm, .. } => {
            children = vec![int_value_expr(imm.clone())];
        }
        InstData::ConstTime { imm, .. } => {
            children = vec![time_value_expr(imm.clone())];
        }
        InstData::Ternary { args, .. } => {
            let expr_x = value_data_expr(&unit, &unit[args[0]]);
            let expr_y = value_data_expr(&unit, &unit[args[1]]);
            let expr_z = value_data_expr(&unit, &unit[args[2]]);
            children = vec![expr_x, expr_y, expr_z];
        }
        InstData::Nullary { .. } => {}
        _ => {
            panic!("No implementation for this InstData type.")
        }
    }
    GenericExpr::call(inst_symbol, children)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use llhd::table::TableKey;
    use opcode::symbol_opcode;

    use super::*;
    use crate::llhd::LLHDUtils;

    extern crate utilities;

    #[test]
    #[should_panic]
    fn all_opcodes_available_in_egglog() {
        assert_eq!(
            LLHD_DFG_VARIANTS_COUNT.to_owned(),
            OPCODESYMBOLMAP_COUNT.to_owned(),
            "Not all LLHD Inst Opcodes are available in Egglog."
        );
    }

    #[test]
    fn egglog_symbol_from_llhd_opcode() {
        let opcode = Opcode::Eq;
        let egglog_symbol = opcode_symbol(opcode);
        let expected_str = "Eq".to_owned();
        assert_eq!(
            expected_str,
            egglog_symbol.to_string(),
            "Opcode::Eq should be represented as 'Eq'."
        );
        let drv_opcode = Opcode::Drv;
        let drv_egglog_symbol = opcode_symbol(drv_opcode);
        let drv_expected_str = "Drv".to_owned();
        assert_eq!(
            drv_expected_str,
            drv_egglog_symbol.to_string(),
            "Opcode::Drv should be represented as 'Drv'."
        );
    }

    #[test]
    fn llhd_opcode_from_egglog_symbol() {
        let symbol = Symbol::new("Eq");
        let opcode = symbol_opcode(symbol);
        let expected_opcode = Opcode::Eq;
        assert_eq!(
            expected_opcode, opcode,
            "Symbol('Eq') should be map to Opcode::Eq."
        );
        let drv_symbol = Symbol::new("Drv");
        let drv_opcode = symbol_opcode(drv_symbol);
        let drv_expected_opcode = Opcode::Drv;
        assert_eq!(
            drv_expected_opcode, drv_opcode,
            "Symbol('Drv') should be map to Opcode::Drv."
        );
    }

    #[test]
    fn llhd_egglog_value_datatypes() {
        let value_datatype = value();
        let expected_str = "(datatype LLHDValue (Value i64))".to_owned();
        assert_eq!(
            expected_str,
            value_datatype.to_string(),
            "Datatype should be named 'LLHDValue' and should have 1 field named (Value i64)."
        );
        let int_value_datatype = int_value();
        let int_expected_str = "(datatype LLHDIntValue (IntValue i64))".to_owned();
        assert_eq!(
            int_expected_str,
            int_value_datatype.to_string(),
            "Datatype should be named 'LLHDIntValue' and should have 1 field named (IntValue i64)."
        );
        let time_value_datatype = time_value();
        let time_expected_str = "(datatype LLHDTimeValue (TimeValue i64))".to_owned();
        assert_eq!(
            time_expected_str,
            time_value_datatype.to_string(),
            "Datatype should be named 'LLHDTimeValue' and should have 1 field named (TimeValue \
             i64)."
        );
        let reg_mode_datatype = reg_mode();
        let reg_mode_expected_str = utilities::trim_whitespace(indoc::indoc! {"
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
        let vec_sort = vec_sort();
        let expected_str = "(sort LLHDVecValue (Vec i64))".to_owned();
        assert_eq!(
            expected_str,
            vec_sort.to_string(),
            "Sort should be named 'LLHDVecValue' and should have 1 field named (Vec i64)."
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
        let expected_str = "(datatype LLHDBlock (Block i64))".to_owned();
        assert_eq!(
            expected_str,
            block_datatype.to_string(),
            "Datatype should be named 'LLHDBlock' and should have 1 field named (Block i64)."
        );
    }

    #[test]
    #[should_panic]
    fn llhd_egglog_dfg_datatypes() {
        let dfg_datatype = dfg();
        let expected_str = utilities::trim_whitespace(indoc::indoc! {"
            (datatype LLHDDFG
                (Value i64)
                (ConstInt String)
                (ConstTime String)
                (Alias LLHDDFG)
                (Not LLHDDFG)
                (Neg LLHDDFG)
                (Add LLHDDFG LLHDDFG)
                (Sub LLHDDFG LLHDDFG)
                (And LLHDDFG LLHDDFG)
                (Or LLHDDFG LLHDDFG)
                (Xor LLHDDFG LLHDDFG)
                (Sig LLHDDFG)
                (Prb LLHDDFG)
                (Drv LLHDDFG LLHDDFG LLHDDFG)
                (Wait i64 LLHDVecValue)
                (LLHDUnit LLHDDFG))
        "});
        assert_eq!(
            expected_str,
            dfg_datatype.to_string(),
            "Datatype should be named 'LLHDValue' and should have 1 field named (Value i64)."
        );
    }

    #[test]
    fn llhd_value_egglog_expr() {
        let value1 = Value::new(1);
        let value1_expr = value_def_expr(value1);
        let expected_str = "(Value 1)";
        assert_eq!(
            expected_str,
            value1_expr.to_string(),
            "Expr should match LLHDValue Constructor, (Value _)."
        );
    }

    #[test]
    fn llhd_inst_egglog_expr() {
        let unit_data = utilities::build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let insts = LLHDUtils::iterate_unit_insts(&unit).collect_vec();
        let add2_inst = insts[4];
        let add2_inst_data = &unit[add2_inst.1];
        assert_eq!(Opcode::Add, add2_inst_data.opcode(), "Inst should be Add.");
        let add2_expr = inst_expr(&unit, add2_inst_data);
        let expected_str = utilities::trim_whitespace(indoc::indoc! {"
            (Add
                (Add
                    (ConstInt \"i1 0\")
                    (ConstInt \"i1 1\"))
                (Prb (Value 2)))
        "});
        assert_eq!(
            expected_str,
            add2_expr.to_string(),
            "Expr should match nested Add expr."
        );
    }
}
