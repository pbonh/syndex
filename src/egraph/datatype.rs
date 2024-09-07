use egglog::ast::{Symbol, Variant, DUMMY_SPAN};
use itertools::Itertools;
use llhd::ir::Opcode;

use super::egglog_names::*;
use super::inst::opcode::*;
use super::{inst, EgglogProgram};

#[derive(Debug, Clone)]
pub struct EgglogSorts(pub(in crate::egraph) EgglogProgram);

impl EgglogSorts {
    pub fn llhd_dfg() -> Self {
        Self(inst::dfg())
    }
}

impl Into<EgglogProgram> for EgglogSorts {
    fn into(self) -> EgglogProgram {
        self.0
    }
}

pub(in crate::egraph) fn variant(opcode: Opcode, symbol_strs: Vec<&str>) -> Variant {
    Variant {
        span: DUMMY_SPAN.clone(),
        name: opcode_symbol(opcode),
        types: symbol_strs.into_iter().map(Symbol::new).collect_vec(),
        cost: None,
    }
}

pub(in crate::egraph) fn value_ref_variant() -> Variant {
    Variant {
        span: DUMMY_SPAN.clone(),
        name: Symbol::new(LLHD_VALUE_REF_FIELD),
        types: vec![Symbol::new(EGGLOG_I64_SORT)],
        cost: None,
    }
}

fn reg_mode_variant() -> Variant {
    Variant {
        span: DUMMY_SPAN.clone(),
        name: unit_root_variant_symbol(),
        types: vec![Symbol::new(LLHD_DFG_DATATYPE)],
        cost: None,
    }
}

pub(in crate::egraph) fn unit_root_variant() -> Variant {
    Variant {
        span: DUMMY_SPAN.clone(),
        name: unit_root_variant_symbol(),
        types: vec![Symbol::new(LLHD_DFG_DATATYPE)],
        cost: None,
    }
}

pub(in crate::egraph) fn unit_root_variant_symbol() -> Symbol {
    Symbol::new(LLHD_UNIT_FIELD)
}

#[cfg(test)]
mod tests {
    use egglog::EGraph;

    use super::*;

    #[test]
    fn valid_egglog_datatypes() {
        let llhd_dfg_sort = EgglogSorts::llhd_dfg();
        let mut egraph = EGraph::default();
        let egraph_msgs = egraph.run_program(llhd_dfg_sort.into());
        assert!(
            egraph_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph_msgs.err().unwrap()
        );
    }
}
