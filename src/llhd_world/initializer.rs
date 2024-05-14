use crate::llhd::common::filter_nullary;
use crate::llhd_world::components::{
    inst::LLHDInstComponent, unit::LLHDUnitComponent, value::LLHDValueComponent,
};
use llhd::ir::{Module, Unit};

pub(crate) fn build_units(module: &Module) -> impl Iterator<Item = LLHDUnitComponent> + '_ {
    module.units().map(|unit| LLHDUnitComponent {
        id: Some(unit.id()),
        name: unit.name().clone(),
        kind: unit.kind(),
    })
}

pub(crate) fn build_values<'unit>(
    unit: &'unit Unit,
) -> impl Iterator<Item = LLHDValueComponent> + 'unit {
    unit.args()
        .map(|arg| LLHDValueComponent {
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
                    LLHDValueComponent {
                        id: Some(value_id),
                        data: value_data.clone(),
                    }
                }),
        )
}

pub(crate) fn build_insts<'unit>(
    unit: &'unit Unit,
) -> impl Iterator<Item = LLHDInstComponent> + 'unit {
    unit.all_insts()
        .filter(|inst| filter_nullary(unit, *inst))
        .map(|inst| {
            let inst_data = &unit[inst];
            LLHDInstComponent {
                id: Some(inst),
                data: inst_data.clone(),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use llhd::ir::prelude::*;
    use llhd::ir::ValueData;
    use llhd::table::TableKey;

    #[test]
    fn build_unit_component() {
        let input = indoc::indoc! {"
            proc %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %in3_prb = prb i1$ %in2
                %and1 = and i1 %in1_prb, %in2_prb
                %and2 = and i1 %in3_prb, %and1
                drv i1$ %out1, %and2, %epsilon
                wait %init for %epsilon
            }

            entity @top () -> () {
                %top_input1 = const i1 0
                %in1 = sig i1 %top_input1
                %top_input2 = const i1 1
                %in2 = sig i1 %top_input2
                %top_input3 = const i1 1
                %in3 = sig i1 %top_input3
                %top_out1 = const i1 0
                %out1 = sig i1 %top_out1
                inst %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1)
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let units: Vec<LLHDUnitComponent> = build_units(&module).collect();
        assert_eq!(2, units.len(), "There should be 2 Units present in Module.");
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
    fn create_value_component() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let value_components: Vec<LLHDValueComponent> = build_values(&unit).collect();
        assert_eq!(
            9,
            value_components.len(),
            "There should be 9 Values defined in Unit."
        );
        assert_eq!(
            Value::new(0),
            value_components[0].id.unwrap(),
            "First Id should be Arg with Id: 0"
        );
        assert_eq!(
            Value::new(1),
            value_components[1].id.unwrap(),
            "Second Id should be Arg with Id: 1"
        );
        let add_value_component = value_components.last().unwrap();
        if let ValueData::Inst { inst, .. } = add_value_component.data {
            let add_inst_data = &unit[inst];
            let opcode = add_inst_data.opcode();
            assert!(matches!(opcode, Opcode::Add), "Inst should be Add type.");
        } else {
            panic!("Value(7) should correspond to an add inst.");
        }
        assert_eq!(
            Value::new(8),
            add_value_component.id.unwrap(),
            "Last Id should be Value with Id: 7"
        );
    }

    #[test]
    fn create_inst_component() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let inst_components: Vec<LLHDInstComponent> = build_insts(&unit).collect();
        assert_eq!(
            5,
            inst_components.len(),
            "There should be 5 Insts defined in Unit."
        );
        assert_eq!(
            Inst::new(1),
            inst_components[0].id.unwrap(),
            "First Id should be Inst with Id: 0"
        );
        assert_eq!(
            Inst::new(2),
            inst_components[1].id.unwrap(),
            "Second Id should be Inst with Id: 1"
        );
        let add_component = &inst_components.last().unwrap();
        let add_inst_data = &add_component.data;
        let opcode = add_inst_data.opcode();
        assert_eq!(
            Inst::new(5),
            add_component.id.unwrap(),
            "Last Id should be Inst with Id: 4"
        );
        assert!(matches!(opcode, Opcode::Add), "Inst should be Add type.");
    }
}
