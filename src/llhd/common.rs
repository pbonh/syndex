use llhd::ir::{Inst, InstData, Signature, Unit, UnitBuilder, UnitData, UnitName};

use super::enode::LLHDENode;

pub(crate) fn filter_nullary(unit: &Unit, instruction: &Inst) -> bool {
    let inst_data = &unit[*instruction];
    !matches!(inst_data, InstData::Nullary { .. })
}

pub(crate) fn build_unit(nets: &[LLHDENode], name: &UnitName, sig: &Signature) -> UnitData {
    let mut unit = UnitData::new(llhd::ir::UnitKind::Entity, name.clone(), sig.clone());
    {
        let mut builder = UnitBuilder::new_anonymous(&mut unit);
        nets.iter()
            .filter(|enode| InstData::default() != enode.data)
            .for_each(|inst_enode| {
                builder.build_inst(inst_enode.data.clone(), inst_enode.ty.clone());
            });
        Unit::new_anonymous(&unit).verify();
    }
    unit
}

pub(crate) fn build_enodes<'u>(unit: &'u Unit) -> impl Iterator<Item = LLHDENode> + 'u {
    unit.input_args()
        .map(|arg| LLHDENode {
            id: arg,
            ty: unit.value_type(arg),
            data: InstData::default(),
        })
        .chain(
            unit.all_insts()
                .filter(|inst| filter_nullary(unit, inst))
                .map(|inst| LLHDENode {
                    id: unit.inst_result(inst),
                    ty: unit.inst_type(inst),
                    data: unit[inst].clone(),
                }),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use llhd::ir::{Opcode, Value};
    use llhd::table::TableKey;

    #[test]
    fn filter_nullary_from_unit() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let unit = module.units().next().unwrap();
        let unit_insts = unit.all_insts().filter(|inst| filter_nullary(&unit, inst));
        assert_eq!(
            2,
            unit_insts.count(),
            "There should be 2 Instructions present in Unit."
        );
    }

    #[test]
    fn build_unit_from_slice() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
            }
        "};
        let mut module = llhd::assembly::parse_module(input).unwrap();
        let unit_id = module.units().next().unwrap().id();
        let unit_name = module.units().next().unwrap().name().clone();
        let unit_sig = module.units().next().unwrap().sig().clone();
        let args: Vec<Value> = module.unit(unit_id).input_args().collect();
        let mut unit = module.unit_mut(unit_id);

        let arg1 = LLHDENode {
            id: args[0],
            ty: unit.value_type(args[0]),
            data: InstData::default(),
        };
        let arg2 = LLHDENode {
            id: args[1],
            ty: unit.value_type(args[1]),
            data: InstData::default(),
        };
        let arg3 = LLHDENode {
            id: args[2],
            ty: unit.value_type(args[2]),
            data: InstData::default(),
        };

        // Make up the Id's/Types
        let and1_id = Value::new(3);
        let or1_id = Value::new(4);
        let and1 = LLHDENode {
            id: and1_id,
            ty: unit.value_type(args[0]),
            data: InstData::Binary {
                opcode: Opcode::And,
                args: [args[0], args[1]],
            },
        };
        let or1 = LLHDENode {
            id: or1_id,
            ty: unit.value_type(args[0]),
            data: InstData::Binary {
                opcode: Opcode::Or,
                args: [and1_id, args[2]],
            },
        };

        let enodes = vec![arg1, arg2, arg3, and1, or1];
        let updated_unit_data = build_unit(&enodes, &unit_name, &unit_sig);
        *unit.data() = updated_unit_data;

        let final_args: Vec<Value> = unit.input_args().collect();
        assert_eq!(
            3,
            final_args.len(),
            "3 Args should be present in this Unit."
        );
        let final_insts: Vec<Inst> = unit
            .all_insts()
            .filter(|inst| filter_nullary(&unit, inst))
            .collect();
        assert_eq!(
            2,
            final_insts.len(),
            "2 Inst's should be present in this Unit."
        );
        assert_eq!(
            Opcode::And,
            unit[final_insts[0]].opcode(),
            "Inst Opcode should be And."
        );
        assert_eq!(
            Opcode::Or,
            unit[final_insts[1]].opcode(),
            "Inst Opcode should be Or."
        );
    }

    #[test]
    fn construct_enode_from_unit() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let unit = module.units().next().unwrap();
        let enodes: Vec<_> = build_enodes(&unit).collect();
        assert_eq!(5, enodes.len(), "There should be 5 ENodes built from Unit.");
    }
}
