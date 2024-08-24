use egglog::ast::{Command, Symbol, Variant};
use egglog::sort::*;
use itertools::Itertools;
use llhd::ir::prelude::*;
use llhd::ir::InstData;

use crate::llhd::{LLHDInst, LLHDValue};

fn uppercase_first_letter(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

fn opcode_symbol(opcode: Opcode) -> Symbol {
    let mut opcode_str = opcode.to_string();
    match opcode {
        Opcode::ConstTime => opcode_str.push_str("Time"),
        Opcode::ConstInt => opcode_str.push_str("Int"),
        _ => (),
    }
    uppercase_first_letter(&mut opcode_str);
    Symbol::new(opcode_str)
}

const EGGLOG_I64_SORT: &str = "i64";
const LLHD_VALUE_FIELD: &str = "Value";
const LLHD_INT_VALUE_FIELD: &str = "IntValue";
const LLHD_TIME_VALUE_FIELD: &str = "TimeValue";
const LLHD_VEC_VALUE_FIELD: &str = "VecValue";
const LLHD_VALUE_REF_FIELD: &str = "ValueRef";
const LLHD_VALUE_DATATYPE: &str = "LLHDValue";
const LLHD_INT_VALUE_DATATYPE: &str = "LLHDIntValue";
const LLHD_TIME_VALUE_DATATYPE: &str = "LLHDTimeValue";
const LLHD_DFG_DATATYPE: &str = "LLHDDFG";
const LLHD_CFG_DATATYPE: &str = "LLHDCFG";

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

    fn value_ref_variant() -> Variant {
        Variant {
            name: Symbol::new(LLHD_VALUE_REF_FIELD),
            types: vec![Symbol::new(LLHD_VALUE_DATATYPE)],
            cost: None,
        }
    }

    fn variant(opcode: Opcode, symbol_strs: Vec<&str>) -> Variant {
        Variant {
            name: opcode_symbol(opcode),
            types: symbol_strs.into_iter().map(Symbol::new).collect_vec(),
            cost: None,
        }
    }

    pub(crate) fn dfg() -> Command {
        let dfg_symbol = Symbol::new(LLHD_DFG_DATATYPE);
        let dfg_variants = vec![
            Self::value_ref_variant(),
            Self::variant(Opcode::ConstInt, vec![LLHD_INT_VALUE_DATATYPE]),
            Self::variant(Opcode::ConstTime, vec![LLHD_TIME_VALUE_DATATYPE]),
            Self::variant(Opcode::Alias, vec![LLHD_DFG_DATATYPE]),
        ];
        Command::Datatype {
            name: dfg_symbol,
            variants: dfg_variants,
        }
    }

    pub(crate) fn cfg() -> Command {
        let _symbol = Symbol::new(LLHD_CFG_DATATYPE);
        todo!()
    }
}

pub(crate) fn inst_data_let_stmt(inst_data: &InstData) -> Command {
    match inst_data {
        InstData::Binary { opcode, args } => {
            let symbol = opcode_symbol(*opcode);
            let _arg1 = args[0];
            let _arg2 = args[1];
            Command::Datatype {
                name: symbol,
                variants: vec![],
            }
        }
        InstData::Unary { opcode, args } => {
            let symbol = opcode_symbol(*opcode);
            let _arg1 = args[0];
            Command::Datatype {
                name: symbol,
                variants: vec![],
            }
        }
        _ => {
            panic!("No implementation for this InstData type.")
        }
    }
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

pub(crate) fn iterate_unit_value_refs<'unit>(
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
    fn create_insts_and_value_refs() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let insts = iterate_unit_insts(&unit).collect_vec();
        let value_refs = iterate_unit_value_refs(&unit).collect_vec();
        assert_eq!(5, insts.len(), "There should be 5 Insts defined in Unit.");
        assert_eq!(
            5,
            value_refs.len(),
            "There should be 5 Values defined in Unit."
        );
        assert_eq!(
            Value::new(4),
            value_refs[0].2,
            "First Id should be Arg with Id: 4(4 args first)"
        );
        assert_eq!(
            Value::new(5),
            value_refs[1].2,
            "Second Id should be Arg with Id: 5(4 args first)"
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
    }

    #[test]
    fn llhd_egglog_dfg_datatypes() {
        let dfg_datatype = LLHDDatatypes::dfg();
        let expected_str = trim_whitespace(indoc::indoc! {"
            (datatype LLHDDFG
                (ValueRef LLHDValue)
                (ConstInt LLHDIntValue)
                (ConstTime LLHDTimeValue)
                (Alias LLHDDFG))
        "});
        assert_eq!(
            expected_str,
            dfg_datatype.to_string(),
            "Datatype should be named 'LLHDValue' and should have 1 field named (Value i64)."
        );
    }

    #[test]
    #[should_panic]
    fn egglog_command_builder() {
        let inst_data = InstData::default();
        let _egglog_datatype = inst_data_let_stmt(&inst_data);
    }
}
