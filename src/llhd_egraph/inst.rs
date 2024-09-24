use egglog::ast::{Command, Expr, GenericExpr, Literal, Symbol, Variant, DUMMY_SPAN};
use lazy_static::lazy_static;
use llhd::ir::prelude::*;
use llhd::ir::{InstData, ValueData};
use llhd::table::TableKey;
use llhd::{IntValue, TimeValue};

use crate::egraph::egglog_names::*;
use crate::egraph::EgglogCommandList;
use crate::llhd_egraph::datatype::*;
use crate::llhd_egraph::egglog_names::*;

pub(in crate::llhd_egraph) mod opcode;
use opcode::*;

lazy_static! {
    static ref LLHD_DFG_VARIANTS: Vec<Variant> = vec![
        value_ref_variant(),
        variant(Opcode::ConstInt, vec![EGGLOG_U64_SORT, EGGLOG_STRING_SORT]),
        variant(Opcode::ConstTime, vec![EGGLOG_U64_SORT, EGGLOG_STRING_SORT]),
        variant(
            Opcode::Alias,
            vec![EGGLOG_U64_SORT, LLHD_TYPE_DATATYPE, LLHD_DFG_DATATYPE]
        ),
        variant(
            Opcode::ArrayUniform,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                EGGLOG_U64_SORT,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Array,
            vec![EGGLOG_U64_SORT, LLHD_VEC_VALUE_DATATYPE]
        ),
        variant(
            Opcode::Struct,
            vec![EGGLOG_U64_SORT, LLHD_VEC_VALUE_DATATYPE]
        ),
        variant(
            Opcode::Not,
            vec![EGGLOG_U64_SORT, LLHD_TYPE_DATATYPE, LLHD_DFG_DATATYPE]
        ),
        variant(
            Opcode::Neg,
            vec![EGGLOG_U64_SORT, LLHD_TYPE_DATATYPE, LLHD_DFG_DATATYPE]
        ),
        variant(
            Opcode::Add,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Sub,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::And,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Or,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Xor,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Smul,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Sdiv,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Smod,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Srem,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Umul,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Udiv,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Umod,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Urem,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Eq,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Neq,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Slt,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Sgt,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Sle,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Sge,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Ult,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Ugt,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Ule,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Uge,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Shl,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Shr,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Mux,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Reg,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_VEC_VALUE_DATATYPE,
                LLHD_VEC_REGMODE_DATATYPE
            ]
        ),
        variant(
            Opcode::InsField,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                EGGLOG_U64_SORT,
                EGGLOG_U64_SORT
            ]
        ),
        variant(
            Opcode::InsSlice,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                EGGLOG_U64_SORT,
                EGGLOG_U64_SORT
            ]
        ),
        variant(
            Opcode::ExtField,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                EGGLOG_U64_SORT,
                EGGLOG_U64_SORT
            ]
        ),
        variant(
            Opcode::ExtSlice,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                EGGLOG_U64_SORT,
                EGGLOG_U64_SORT
            ]
        ),
        variant(
            Opcode::Con,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Del,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ]
        ),
        variant(
            Opcode::Call,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_EXT_UNIT_DATATYPE,
                EGGLOG_U64_SORT,
                LLHD_VEC_VALUE_DATATYPE
            ]
        ),
        variant(
            Opcode::Inst,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_EXT_UNIT_DATATYPE,
                EGGLOG_U64_SORT,
                LLHD_VEC_VALUE_DATATYPE
            ]
        ),
        variant(
            Opcode::Sig,
            vec![EGGLOG_U64_SORT, LLHD_TYPE_DATATYPE, LLHD_DFG_DATATYPE]
        ),
        variant(
            Opcode::Prb,
            vec![EGGLOG_U64_SORT, LLHD_TYPE_DATATYPE, LLHD_DFG_DATATYPE]
        ),
        variant(
            Opcode::Drv,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ],
        ),
        variant(
            Opcode::DrvCond,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ],
        ),
        variant(
            Opcode::Var,
            vec![EGGLOG_U64_SORT, LLHD_TYPE_DATATYPE, LLHD_DFG_DATATYPE],
        ),
        variant(
            Opcode::Ld,
            vec![EGGLOG_U64_SORT, LLHD_TYPE_DATATYPE, LLHD_DFG_DATATYPE],
        ),
        variant(
            Opcode::St,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_DFG_DATATYPE
            ],
        ),
        variant(Opcode::Halt, vec![EGGLOG_U64_SORT,],),
        variant(Opcode::Ret, vec![EGGLOG_U64_SORT,],),
        variant(
            Opcode::RetValue,
            vec![EGGLOG_U64_SORT, LLHD_TYPE_DATATYPE, LLHD_DFG_DATATYPE],
        ),
        variant(
            Opcode::Phi,
            vec![
                EGGLOG_U64_SORT,
                LLHD_VEC_VALUE_DATATYPE,
                LLHD_VEC_BLOCK_DATATYPE
            ]
        ),
        variant(Opcode::Br, vec![EGGLOG_U64_SORT, LLHD_BLOCK_DATATYPE]),
        variant(
            Opcode::BrCond,
            vec![
                EGGLOG_U64_SORT,
                LLHD_TYPE_DATATYPE,
                LLHD_DFG_DATATYPE,
                LLHD_BLOCK_DATATYPE,
                LLHD_BLOCK_DATATYPE
            ]
        ),
        variant(
            Opcode::Wait,
            vec![
                EGGLOG_U64_SORT,
                LLHD_BLOCK_DATATYPE,
                LLHD_VEC_VALUE_DATATYPE
            ]
        ),
        variant(
            Opcode::WaitTime,
            vec![
                EGGLOG_U64_SORT,
                LLHD_BLOCK_DATATYPE,
                LLHD_VEC_VALUE_DATATYPE
            ]
        ),
    ];
    static ref LLHD_DFG_VARIANTS_COUNT: usize = LLHD_DFG_VARIANTS.len();
}

pub(in crate::llhd_egraph) fn dfg_insts() -> Command {
    let dfg_symbol = Symbol::new(LLHD_DFG_DATATYPE);
    Command::Datatype {
        span: DUMMY_SPAN.clone(),
        name: dfg_symbol,
        variants: LLHD_DFG_VARIANTS.to_vec(),
    }
}

pub(in crate::llhd_egraph) fn dfg() -> EgglogCommandList {
    vec![dfg_insts()]
}

pub(in crate::llhd_egraph) fn cfg() -> EgglogCommandList {
    let _symbol = Symbol::new(LLHD_CFG_DATATYPE);
    todo!()
}

fn value_def_expr(table_key: impl TableKey) -> Expr {
    let converted_u64_num =
        u64::try_from(table_key.index()).expect("Out-of-bound value for usize -> u64 conversion.");
    let converted_literal = Literal::UInt(converted_u64_num);
    let literal_value = GenericExpr::Lit(DUMMY_SPAN.clone(), converted_literal);
    let llhd_value_datatype_symbol = Symbol::new(LLHD_VALUE_REF_FIELD);
    GenericExpr::Call(
        DUMMY_SPAN.clone(),
        llhd_value_datatype_symbol,
        [literal_value].to_vec(),
    )
}

fn value_data_expr(unit: &Unit<'_>, value_data: &ValueData) -> Expr {
    match value_data {
        ValueData::Inst { inst, .. } => inst_expr(unit, &unit[inst.to_owned()]),
        ValueData::Arg { arg, .. } => value_def_expr(arg.to_owned()),
        _ => panic!("Value type not supported."),
    }
}

pub(in crate::llhd_egraph) fn expr_value_data(literal: &Literal) -> Value {
    match literal {
        Literal::Int(value) => {
            Value::new(usize::try_from(*value).expect("Failure to convert from u64 to usize."))
        }
        _ => panic!("Non-Int Literal"),
    }
}

fn int_value_expr(int_value: IntValue) -> Expr {
    let converted_literal = Literal::String(int_value.to_string().into());
    GenericExpr::Lit(DUMMY_SPAN.clone(), converted_literal)
}

pub(super) fn expr_int_value(literal: &Literal) -> IntValue {
    match literal {
        Literal::Int(value) => IntValue::from_isize(
            64,
            isize::try_from(*value).expect("Failure to convert from u64 to isize."),
        ),
        _ => panic!("Non-Int Literal"),
    }
}

fn time_value_expr(time_value: TimeValue) -> Expr {
    let converted_literal = Literal::String(time_value.to_string().into());
    GenericExpr::Lit(DUMMY_SPAN.clone(), converted_literal)
}

pub(in crate::llhd_egraph) fn expr_time_value(_literal: &Literal) -> TimeValue {
    TimeValue::zero()
}

pub(in crate::llhd_egraph) fn inst_expr(unit: &Unit<'_>, inst_data: &InstData) -> Expr {
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
    GenericExpr::Call(DUMMY_SPAN.clone(), inst_symbol, children)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use llhd::table::TableKey;

    use super::*;
    use crate::llhd::LLHDUtils;

    extern crate utilities;

    #[test]
    fn llhd_egglog_dfg_datatypes() {
        let dfg_datatype = dfg();
        let expected_str = utilities::trim_expr_whitespace(indoc::indoc! {"
            (datatype LLHDDFG
                (ValueRef u64 LLHDValue)
                (ConstInt u64 String)
                (ConstTime u64 String)
                (Alias u64 LLHDTy LLHDDFG)
                (ArrayUniform u64 LLHDTy u64 LLHDDFG)
                (Array u64 LLHDVecValue)
                (Struct u64 LLHDVecValue)
                (Not u64 LLHDTy LLHDDFG)
                (Neg u64 LLHDTy LLHDDFG)
                (Add u64 LLHDTy LLHDDFG LLHDDFG)
                (Sub u64 LLHDTy LLHDDFG LLHDDFG)
                (And u64 LLHDTy LLHDDFG LLHDDFG)
                (Or u64 LLHDTy LLHDDFG LLHDDFG)
                (Xor u64 LLHDTy LLHDDFG LLHDDFG)
                (Smul u64 LLHDTy LLHDDFG LLHDDFG)
                (Sdiv u64 LLHDTy LLHDDFG LLHDDFG)
                (Smod u64 LLHDTy LLHDDFG LLHDDFG)
                (Srem u64 LLHDTy LLHDDFG LLHDDFG)
                (Umul u64 LLHDTy LLHDDFG LLHDDFG)
                (Udiv u64 LLHDTy LLHDDFG LLHDDFG)
                (Umod u64 LLHDTy LLHDDFG LLHDDFG)
                (Urem u64 LLHDTy LLHDDFG LLHDDFG)
                (Eq u64 LLHDTy LLHDDFG LLHDDFG)
                (Neq u64 LLHDTy LLHDDFG LLHDDFG)
                (Slt u64 LLHDTy LLHDDFG LLHDDFG)
                (Sgt u64 LLHDTy LLHDDFG LLHDDFG)
                (Sle u64 LLHDTy LLHDDFG LLHDDFG)
                (Sge u64 LLHDTy LLHDDFG LLHDDFG)
                (Ult u64 LLHDTy LLHDDFG LLHDDFG)
                (Ugt u64 LLHDTy LLHDDFG LLHDDFG)
                (Ule u64 LLHDTy LLHDDFG LLHDDFG)
                (Uge u64 LLHDTy LLHDDFG LLHDDFG)
                (Shl u64 LLHDTy LLHDDFG LLHDDFG LLHDDFG)
                (Shr u64 LLHDTy LLHDDFG LLHDDFG LLHDDFG)
                (Mux u64 LLHDTy LLHDDFG LLHDDFG)
                (Reg u64 LLHDTy LLHDVecValue LLHDVecRegMode)
                (InsField u64 LLHDTy LLHDDFG LLHDDFG u64 u64)
                (InsSlice u64 LLHDTy LLHDDFG LLHDDFG u64 u64)
                (ExtField u64 LLHDTy LLHDDFG LLHDDFG u64 u64)
                (ExtSlice u64 LLHDTy LLHDDFG LLHDDFG u64 u64)
                (Con u64 LLHDTy LLHDDFG LLHDDFG)
                (Del u64 LLHDTy LLHDDFG LLHDDFG LLHDDFG)
                (Call u64 LLHDTy LLHDExtUnit u64 LLHDVecValue)
                (Inst u64 LLHDTy LLHDExtUnit u64 LLHDVecValue)
                (Sig u64 LLHDTy LLHDDFG)
                (Prb u64 LLHDTy LLHDDFG)
                (Drv u64 LLHDTy LLHDDFG LLHDDFG LLHDDFG)
                (DrvCond u64 LLHDTy LLHDDFG LLHDDFG LLHDDFG LLHDDFG)
                (Var u64 LLHDTy LLHDDFG)
                (Ld u64 LLHDTy LLHDDFG)
                (St u64 LLHDTy LLHDDFG LLHDDFG)
                (Halt u64)
                (Ret u64)
                (RetValue u64 LLHDTy LLHDDFG)
                (Phi u64 LLHDVecValue LLHDVecBlock)
                (Br u64 LLHDBlock)
                (BrCond u64 LLHDTy LLHDDFG LLHDBlock LLHDBlock)
                (Wait u64 LLHDBlock LLHDVecValue)
                (WaitTime u64 LLHDBlock LLHDVecValue)
            )
        "});
        assert_eq!(
            expected_str,
            dfg_datatype.into_iter().join(""),
            "LLHD DFG Egglog datatype does not match expected string."
        );
    }

    #[test]
    fn llhd_value_egglog_expr() {
        let value1 = Value::new(1);
        let value1_expr = value_def_expr(value1);
        let expected_str = "(ValueRef _1)";
        assert_eq!(
            expected_str,
            value1_expr.to_string(),
            "Expr should match LLHDValue Constructor, (Value _)."
        );
    }

    #[test]
    fn llhd_inst_egglog_expr() {
        let unit_data = utilities::build_entity_alpha(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let insts = LLHDUtils::iterate_unit_insts(&unit).collect_vec();
        let add2_inst = insts[4];
        let add2_inst_data = &unit[add2_inst.1];
        assert_eq!(Opcode::Add, add2_inst_data.opcode(), "Inst should be Add.");
        let add2_expr = inst_expr(&unit, add2_inst_data);
        let expected_str = utilities::trim_expr_whitespace(indoc::indoc! {"
            (Add
                (Add
                    (ConstInt \"i1 0\")
                    (ConstInt \"i1 1\"))
                (Prb (ValueRef _2)))
        "});
        assert_eq!(
            expected_str,
            add2_expr.to_string(),
            "Expr should match nested Add expr."
        );
    }
}
