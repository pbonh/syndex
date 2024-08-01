use llhd::ir::prelude::*;
use llhd::ir::InstData;

use crate::llhd::{LLHDInst, LLHDValue};

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
}
