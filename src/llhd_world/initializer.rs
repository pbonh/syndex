use crate::llhd::common::filter_nullary;
use crate::llhd_world::components::value::ValueComponent;
use llhd::ir::Unit;

pub(crate) fn build_values<'unit>(unit: &'unit Unit) -> impl Iterator<Item = ValueComponent> + 'unit {
    unit.input_args()
        .map(|arg| ValueComponent {
            id: Some(arg),
            data: unit[arg].clone(),
        })
        .chain(
            unit.all_insts()
                .filter(|inst| filter_nullary(unit, *inst))
                .filter(|inst| unit.get_inst_result(*inst).is_some())
                .map(|inst| {
                    let value_id = unit.inst_result(inst);
                    let value_data = &unit[value_id];
                    ValueComponent {
                        id: Some(value_id),
                        data: value_data.clone(),
                    }
                }),
        )
}

#[cfg(test)]
mod tests {
    use llhd::ir::prelude::*;
    use llhd::ir::ValueData;
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
    fn create_value_component() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let value_components: Vec<ValueComponent> = build_values(&unit).collect();
        assert_eq!(8, value_components.len(), "There should be 9 Values defined in Unit.");
        assert_eq!(Value::new(0), value_components[0].id.unwrap(), "First Id should be Arg with Id: 0");
        assert_eq!(Value::new(1), value_components[1].id.unwrap(), "Second Id should be Arg with Id: 1");
        let add_value_component = value_components.last().unwrap();
        if let ValueData::Inst{ inst, .. } = add_value_component.data {
            let add_inst_data = &unit[inst];
            let opcode = add_inst_data.opcode();
            assert!(matches!(opcode, Opcode::Add), "Inst should be Add type.");
        } else {
            panic!("Value(7) should correspond to an add inst.");
        }
        assert_eq!(Value::new(8), add_value_component.id.unwrap(), "Last Id should be Value with Id: 7");
    }
}
