use std::collections::HashMap;

use egglog::ast::{Command, Expr, GenericExpr, Literal, Symbol, Variant};
use egglog::sort::{I64Sort, Sort};
use itertools::Itertools;
use lazy_static::lazy_static;
use llhd::ir::prelude::*;
use llhd::ir::{InstData, ValueData};
use llhd::table::TableKey;
use llhd::{IntValue, TimeValue};

use crate::llhd::{LLHDInst, LLHDValue};

const EGGLOG_I64_SORT: &str = "i64";
const EGGLOG_STRING_SORT: &str = "String";
const EGGLOG_VEC_SORT: &str = "Vec";
const LLHD_UNIT_FIELD: &str = "LLHDUnit";
const LLHD_VALUE_FIELD: &str = "Value";
const LLHD_INT_VALUE_FIELD: &str = "IntValue";
const LLHD_TIME_VALUE_FIELD: &str = "TimeValue";
const LLHD_REGMODE_FIELD_LOW: &str = "Low";
const LLHD_REGMODE_FIELD_HIGH: &str = "High";
const LLHD_REGMODE_FIELD_RISE: &str = "Rise";
const LLHD_REGMODE_FIELD_FALL: &str = "Fall";
const LLHD_REGMODE_FIELD_BOTH: &str = "Both";
const LLHD_VEC_VALUE_FIELD: &str = "VecValue";
const LLHD_VALUE_REF_FIELD: &str = "ValueRef";
const LLHD_VALUE_DATATYPE: &str = "LLHDValue";
const LLHD_INT_VALUE_DATATYPE: &str = "LLHDIntValue";
const LLHD_TIME_VALUE_DATATYPE: &str = "LLHDTimeValue";
const LLHD_REGMODE_DATATYPE: &str = "LLHDRegMode";
const LLHD_VEC_VALUE_DATATYPE: &str = "LLHDVecValue";
const LLHD_VEC_REGMODE_DATATYPE: &str = "LLHDVecRegMode";
const LLHD_BLOCK_FIELD: &str = "Block";
const LLHD_BLOCK_DATATYPE: &str = "LLHDBlock";
const LLHD_DFG_DATATYPE: &str = "LLHDDFG";
const LLHD_CFG_DATATYPE: &str = "LLHDCFG";

type LLHDOpcodeSymbolLookup = HashMap<Symbol, Opcode>;

lazy_static! {
    static ref OPCODESYMBOLMAP: LLHDOpcodeSymbolLookup = {
        let mut opcode_symbol_map = HashMap::new();
        opcode_symbol_map.insert(opcode_symbol(Opcode::ConstInt), Opcode::ConstInt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::ConstTime), Opcode::ConstTime);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Alias), Opcode::Alias);
        opcode_symbol_map.insert(opcode_symbol(Opcode::ArrayUniform), Opcode::ArrayUniform);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Array), Opcode::Array);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Struct), Opcode::Struct);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Not), Opcode::Not);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Neg), Opcode::Neg);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Add), Opcode::Add);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sub), Opcode::Sub);
        opcode_symbol_map.insert(opcode_symbol(Opcode::And), Opcode::And);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Or), Opcode::Or);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Xor), Opcode::Xor);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Smul), Opcode::Smul);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sdiv), Opcode::Sdiv);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Smod), Opcode::Smod);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Srem), Opcode::Srem);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Umul), Opcode::Umul);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Udiv), Opcode::Udiv);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Umod), Opcode::Umod);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Urem), Opcode::Urem);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Eq), Opcode::Eq);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Neq), Opcode::Neq);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Slt), Opcode::Slt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sgt), Opcode::Sgt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sle), Opcode::Sle);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sge), Opcode::Sge);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ult), Opcode::Ult);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ugt), Opcode::Ugt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ule), Opcode::Ule);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Uge), Opcode::Uge);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Shl), Opcode::Shl);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Shr), Opcode::Shr);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Mux), Opcode::Mux);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Reg), Opcode::Reg);
        opcode_symbol_map.insert(opcode_symbol(Opcode::InsField), Opcode::InsField);
        opcode_symbol_map.insert(opcode_symbol(Opcode::InsSlice), Opcode::InsSlice);
        opcode_symbol_map.insert(opcode_symbol(Opcode::ExtField), Opcode::ExtField);
        opcode_symbol_map.insert(opcode_symbol(Opcode::ExtSlice), Opcode::ExtSlice);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Con), Opcode::Con);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Del), Opcode::Del);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Call), Opcode::Call);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Inst), Opcode::Inst);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sig), Opcode::Sig);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Prb), Opcode::Prb);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Drv), Opcode::Drv);
        opcode_symbol_map.insert(opcode_symbol(Opcode::DrvCond), Opcode::DrvCond);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Var), Opcode::Var);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ld), Opcode::Ld);
        opcode_symbol_map.insert(opcode_symbol(Opcode::St), Opcode::St);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Halt), Opcode::Halt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ret), Opcode::Ret);
        opcode_symbol_map.insert(opcode_symbol(Opcode::RetValue), Opcode::RetValue);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Phi), Opcode::Phi);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Br), Opcode::Br);
        opcode_symbol_map.insert(opcode_symbol(Opcode::BrCond), Opcode::BrCond);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Wait), Opcode::Wait);
        opcode_symbol_map.insert(opcode_symbol(Opcode::WaitTime), Opcode::WaitTime);
        opcode_symbol_map
    };
    static ref OPCODESYMBOLMAP_COUNT: usize = OPCODESYMBOLMAP.len();
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

pub(super) fn symbol_opcode(symbol: Symbol) -> Opcode {
    OPCODESYMBOLMAP[&symbol]
}

fn uppercase_first_letter(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

pub(super) fn opcode_symbol(opcode: Opcode) -> Symbol {
    let mut opcode_str = opcode.to_string();
    match opcode {
        Opcode::ConstTime => opcode_str.push_str("Time"),
        Opcode::ConstInt => opcode_str.push_str("Int"),
        _ => (),
    }
    uppercase_first_letter(&mut opcode_str);
    Symbol::new(opcode_str)
}

fn variant(opcode: Opcode, symbol_strs: Vec<&str>) -> Variant {
    Variant {
        name: opcode_symbol(opcode),
        types: symbol_strs.into_iter().map(Symbol::new).collect_vec(),
        cost: None,
    }
}

fn value_ref_variant() -> Variant {
    Variant {
        name: Symbol::new(LLHD_VALUE_FIELD),
        types: vec![Symbol::new(EGGLOG_I64_SORT)],
        cost: None,
    }
}

pub(super) fn unit_root_variant_symbol() -> Symbol {
    Symbol::new(LLHD_UNIT_FIELD)
}

fn unit_root_variant() -> Variant {
    Variant {
        name: unit_root_variant_symbol(),
        types: vec![Symbol::new(LLHD_DFG_DATATYPE)],
        cost: None,
    }
}

fn reg_mode_variant() -> Variant {
    Variant {
        name: unit_root_variant_symbol(),
        types: vec![Symbol::new(LLHD_DFG_DATATYPE)],
        cost: None,
    }
}

#[derive(Debug)]
pub(crate) struct LLHDDatatypes;

impl LLHDDatatypes {
    pub(crate) fn value() -> Command {
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

    pub(crate) fn int_value() -> Command {
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

    pub(crate) fn time_value() -> Command {
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

    pub(crate) fn reg_mode() -> Command {
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

    pub(crate) fn vec_sort() -> Command {
        let vec_sort_symbol = Symbol::new(LLHD_VEC_VALUE_DATATYPE);
        let symbol_vec = Symbol::new(EGGLOG_VEC_SORT);
        let i64_sort = I64Sort::new(EGGLOG_I64_SORT.into());
        let i64_expr = Expr::Var((), i64_sort.name());
        Command::Sort(vec_sort_symbol, Some((symbol_vec, vec![i64_expr])))
    }

    pub(crate) fn vec_regmode_sort() -> Command {
        let vec_sort_symbol = Symbol::new(LLHD_VEC_REGMODE_DATATYPE);
        let symbol_vec = Symbol::new(EGGLOG_VEC_SORT);
        let regmode_datatype = I64Sort::new(LLHD_REGMODE_DATATYPE.into());
        let regmode_expr = Expr::Var((), regmode_datatype.name());
        Command::Sort(vec_sort_symbol, Some((symbol_vec, vec![regmode_expr])))
    }

    pub(crate) fn block() -> Command {
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

    pub(crate) fn dfg() -> Command {
        let dfg_symbol = Symbol::new(LLHD_DFG_DATATYPE);
        Command::Datatype {
            name: dfg_symbol,
            variants: LLHD_DFG_VARIANTS.to_vec(),
        }
    }

    pub(crate) fn cfg() -> Command {
        let _symbol = Symbol::new(LLHD_CFG_DATATYPE);
        todo!()
    }
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

fn int_value_expr(int_value: IntValue) -> Expr {
    let converted_literal = Literal::String(int_value.to_string().into());
    GenericExpr::lit(converted_literal)
}

fn time_value_expr(time_value: TimeValue) -> Expr {
    let converted_literal = Literal::String(time_value.to_string().into());
    GenericExpr::lit(converted_literal)
}

pub(super) fn inst_expr(unit: &Unit<'_>, inst_data: &InstData) -> Expr {
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

pub(crate) fn iterate_unit_insts<'unit>(
    unit: &'unit Unit,
) -> impl Iterator<Item = LLHDInst> + 'unit {
    unit.all_insts().filter_map(|inst| {
        let unit_id = unit.id();
        let inst_data = &unit[inst];
        if !matches!(inst_data, InstData::Nullary { .. }) {
            Some((unit_id, inst))
        } else {
            None
        }
    })
}

pub(crate) fn last_unit_inst<'unit>(unit: &'unit Unit) -> LLHDInst {
    let blocks = unit.blocks().collect_vec();
    let last_block = blocks.last().expect("Unit empty.");
    (
        unit.id(),
        unit.last_inst(*last_block).expect("Empty Unit Block."),
    )
    // let last_block = unit.last_block().expect("Unit empty.");
    // (unit.id(), unit.last_inst(last_block).expect("Empty Unit Block."))
}

pub(crate) fn iterate_unit_value_defs<'unit>(
    unit: &'unit Unit,
) -> impl Iterator<Item = LLHDValue> + 'unit {
    unit.all_insts()
        .filter(|inst| unit.get_inst_result(*inst).is_some())
        .map(|inst| {
            let value_id = unit.inst_result(inst);
            (unit.id(), inst, value_id)
        })
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use llhd::table::TableKey;

    use super::*;

    fn trim_whitespace(s: &str) -> String {
        // first attempt: allocates a vector and a string
        let words: Vec<_> = s.split_whitespace().collect();
        words.join(" ")
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

    #[test]
    fn create_insts_and_value_defs() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let insts = iterate_unit_insts(&unit).collect_vec();
        let value_defs = iterate_unit_value_defs(&unit).collect_vec();
        assert_eq!(5, insts.len(), "There should be 5 Insts defined in Unit.");
        assert_eq!(
            5,
            value_defs.len(),
            "There should be 5 Values defined in Unit."
        );
        assert_eq!(
            Value::new(4),
            value_defs[0].2,
            "First Id should be Arg with Id: 4(4 args first)"
        );
        assert_eq!(
            Value::new(5),
            value_defs[1].2,
            "Second Id should be Arg with Id: 5(4 args first)"
        );
    }

    #[test]
    #[should_panic]
    fn get_last_llhd_unit_inst() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let add2_inst = last_unit_inst(&unit);
        let add2_inst_data = &unit[add2_inst.1];
        assert_eq!(Opcode::Add, add2_inst_data.opcode(), "Inst should be Add.");
    }

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
    }

    #[test]
    fn llhd_egglog_value_datatypes() {
        let value_datatype = LLHDDatatypes::value();
        let expected_str = "(datatype LLHDValue (Value i64))".to_owned();
        assert_eq!(
            expected_str,
            value_datatype.to_string(),
            "Datatype should be named 'LLHDValue' and should have 1 field named (Value i64)."
        );
        let int_value_datatype = LLHDDatatypes::int_value();
        let int_expected_str = "(datatype LLHDIntValue (IntValue i64))".to_owned();
        assert_eq!(
            int_expected_str,
            int_value_datatype.to_string(),
            "Datatype should be named 'LLHDIntValue' and should have 1 field named (IntValue i64)."
        );
        let time_value_datatype = LLHDDatatypes::time_value();
        let time_expected_str = "(datatype LLHDTimeValue (TimeValue i64))".to_owned();
        assert_eq!(
            time_expected_str,
            time_value_datatype.to_string(),
            "Datatype should be named 'LLHDTimeValue' and should have 1 field named (TimeValue \
             i64)."
        );
        let reg_mode_datatype = LLHDDatatypes::reg_mode();
        let reg_mode_expected_str = trim_whitespace(indoc::indoc! {"
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
        let vec_sort = LLHDDatatypes::vec_sort();
        let expected_str = "(sort LLHDVecValue (Vec i64))".to_owned();
        assert_eq!(
            expected_str,
            vec_sort.to_string(),
            "Sort should be named 'LLHDVecValue' and should have 1 field named (Vec i64)."
        );
        let vec_regmode_sort = LLHDDatatypes::vec_regmode_sort();
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
        let block_datatype = LLHDDatatypes::block();
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
        let dfg_datatype = LLHDDatatypes::dfg();
        let expected_str = trim_whitespace(indoc::indoc! {"
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
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let insts = iterate_unit_insts(&unit).collect_vec();
        let add2_inst = insts[4];
        let add2_inst_data = &unit[add2_inst.1];
        assert_eq!(Opcode::Add, add2_inst_data.opcode(), "Inst should be Add.");
        let add2_expr = inst_expr(&unit, &add2_inst_data);
        let expected_str = trim_whitespace(indoc::indoc! {"
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
