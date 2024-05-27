use crate::llhd_world::components::{
    block::LLHDBlockComponent, inst::LLHDInstComponent, unit::LLHDUnitComponent,
    value::LLHDValueDefComponent, value::LLHDValueRefComponent,
};
use llhd::ir::{Block, Inst, InstData, Module, Unit};

pub(crate) fn build_units(module: &Module) -> impl Iterator<Item = LLHDUnitComponent> + '_ {
    module.units().map(|unit| LLHDUnitComponent {
        id: Some(unit.id()),
        name: unit.name().clone(),
        kind: unit.kind(),
    })
}

pub(crate) fn build_value_defs<'unit>(
    unit: &'unit Unit,
) -> impl Iterator<Item = LLHDValueDefComponent> + 'unit {
    unit.args()
        .map(|arg| LLHDValueDefComponent {
            id: Some(arg),
            data: unit[arg].clone(),
        })
        .chain(
            unit.all_insts()
                .filter(|inst| unit.get_inst_result(*inst).is_some())
                .map(|inst| {
                    let value_id = unit.inst_result(inst);
                    let value_data = &unit[value_id];
                    LLHDValueDefComponent {
                        id: Some(value_id),
                        data: value_data.clone(),
                    }
                }),
        )
}

pub(crate) fn build_value_refs<'unit>(
    inst_id: Inst,
    inst_data: &'unit InstData,
) -> impl Iterator<Item = LLHDValueRefComponent> + 'unit {
    inst_data
        .args()
        .iter()
        .map(move |inst_arg| LLHDValueRefComponent {
            id: Some(*inst_arg),
            inst: Some(inst_id),
        })
}

pub(crate) fn build_insts<'unit>(
    unit: &'unit Unit,
    block_id: Block,
) -> impl Iterator<Item = LLHDInstComponent> + 'unit {
    unit.insts(block_id).map(|inst| {
        let inst_value = &unit.get_inst_result(inst);
        let inst_data = &unit[inst];
        LLHDInstComponent {
            id: Some(inst),
            value: *inst_value,
            data: inst_data.clone(),
        }
    })
}

pub(crate) fn build_blocks<'unit>(
    unit: &'unit Unit,
) -> impl Iterator<Item = LLHDBlockComponent> + 'unit {
    unit.blocks().map(|block| {
        let block_data = &unit[block];
        LLHDBlockComponent {
            id: Some(block),
            data: block_data.clone(),
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
    fn create_value_def_component() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let value_def_components: Vec<LLHDValueDefComponent> = build_value_defs(&unit).collect();
        assert_eq!(
            9,
            value_def_components.len(),
            "There should be 9 Values defined in Unit."
        );
        assert_eq!(
            Value::new(0),
            value_def_components[0].id.unwrap(),
            "First Id should be Arg with Id: 0"
        );
        assert_eq!(
            Value::new(1),
            value_def_components[1].id.unwrap(),
            "Second Id should be Arg with Id: 1"
        );
        let add_value_def_component = value_def_components.last().unwrap();
        if let ValueData::Inst { inst, .. } = add_value_def_component.data {
            let add_inst_data = &unit[inst];
            let opcode = add_inst_data.opcode();
            assert!(matches!(opcode, Opcode::Add), "Inst should be Add type.");
        } else {
            panic!("Value(7) should correspond to an add inst.");
        }
        assert_eq!(
            Value::new(8),
            add_value_def_component.id.unwrap(),
            "Last Id should be Value with Id: 7"
        );
    }

    #[test]
    fn create_value_ref_component() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let value_def_components: Vec<LLHDValueDefComponent> = build_value_defs(&unit).collect();
        let add_value_def_component = value_def_components.last().unwrap();
        if let ValueData::Inst { inst, .. } = add_value_def_component.data {
            let add_inst_id = inst;
            let add_inst_data = &unit[add_inst_id];
            let opcode = add_inst_data.opcode();
            assert!(matches!(opcode, Opcode::Add), "Inst should be Add type.");
            let value_ref_components: Vec<LLHDValueRefComponent> =
                build_value_refs(add_inst_id, &add_inst_data).collect();
            assert_eq!(
                2,
                value_ref_components.len(),
                "There should be 2 Value Refs defined in Add Instruction."
            );
        } else {
            panic!("Value(7) should correspond to an add inst.");
        }
    }

    #[test]
    fn create_inst_component() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let block_id = Block::new(0);
        let inst_components: Vec<LLHDInstComponent> = build_insts(&unit, block_id).collect();
        assert_eq!(
            6,
            inst_components.len(),
            "There should be 6 Insts defined in Unit."
        );
        assert_eq!(
            Inst::new(1),
            inst_components[0].id.unwrap(),
            "First Id should be Inst with Id: 1"
        );
        assert_eq!(
            Inst::new(2),
            inst_components[1].id.unwrap(),
            "Second Id should be Inst with Id: 0"
        );
        let add_component = &inst_components[2];
        let add_inst_data = &add_component.data;
        let opcode = add_inst_data.opcode();
        assert_eq!(
            Inst::new(3),
            add_component.id.unwrap(),
            "Last Id should be Inst with Id: 3"
        );
        assert!(matches!(opcode, Opcode::Add), "Inst should be Add type.");
    }

    #[test]
    fn create_block_component() {
        let input = indoc::indoc! {"
            declare @bar (i32, i9) i32

            func @foo (i32 %x, i8 %y) i32 {
            %entry:
                %asdf0 = const i32 42
                %1 = const time 1.489ns 10d 9e
                %hello = alias i32 %asdf0
                %2 = not i32 %asdf0
                %3 = neg i32 %2
                %4 = add i32 %2, %3
                %5 = sub i32 %2, %3
                %6 = and i32 %2, %3
                %7 = or i32 %2, %3
                %8 = xor i32 %2, %3
                %cmp = eq i32 %7, %7
                br %cmp, %entry, %next
            %next:
                %a = exts i9, i32 %7, 4, 9
                %b = neg i9 %a
                %r = call i32 @bar (i32 %8, i9 %b)
                %many = [32 x i9 %b]
                %some = exts [9 x i9], [32 x i9] %many, 2, 9
                %one = extf i9, [9 x i9] %some, 3
                neg i9 %one
                ret i32 %3
            }
        "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let func_unit = module.units().next().unwrap();
        let block_components: Vec<LLHDBlockComponent> = build_blocks(&func_unit).collect();
        assert_eq!(
            2,
            block_components.len(),
            "There should be 2 Blocks defined in Unit."
        );
    }
}
