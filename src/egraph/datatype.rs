use egglog::ast::{Symbol, Variant};
use itertools::Itertools;
use llhd::ir::Opcode;

use super::egglog_names::*;
use super::inst::opcode::*;

pub(in crate::egraph) fn variant(opcode: Opcode, symbol_strs: Vec<&str>) -> Variant {
    Variant {
        name: opcode_symbol(opcode),
        types: symbol_strs.into_iter().map(Symbol::new).collect_vec(),
        cost: None,
    }
}

pub(in crate::egraph) fn value_ref_variant() -> Variant {
    Variant {
        name: Symbol::new(LLHD_VALUE_FIELD),
        types: vec![Symbol::new(EGGLOG_I64_SORT)],
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

pub(in crate::egraph) fn unit_root_variant() -> Variant {
    Variant {
        name: unit_root_variant_symbol(),
        types: vec![Symbol::new(LLHD_DFG_DATATYPE)],
        cost: None,
    }
}

pub(in crate::egraph) fn unit_root_variant_symbol() -> Symbol {
    Symbol::new(LLHD_UNIT_FIELD)
}
