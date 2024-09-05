use itertools::Itertools;
use llhd::ir::prelude::*;
use llhd::ir::InstData;

use crate::llhd::{LLHDInst, LLHDUtils, LLHDValue};

impl LLHDUtils {
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

    pub(super) fn last_unit_inst<'unit>(unit: &'unit Unit) -> LLHDInst {
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
}

#[cfg(test)]
mod tests {
    use llhd::table::TableKey;

    use super::*;

    extern crate utilities;

    #[test]
    fn create_insts_and_value_defs() {
        let unit_data = utilities::build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let insts = LLHDUtils::iterate_unit_insts(&unit).collect_vec();
        let value_defs = LLHDUtils::iterate_unit_value_defs(&unit).collect_vec();
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
        let unit_data = utilities::build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let add2_inst = LLHDUtils::last_unit_inst(&unit);
        let add2_inst_data = &unit[add2_inst.1];
        assert_eq!(Opcode::Add, add2_inst_data.opcode(), "Inst should be Add.");
    }
}