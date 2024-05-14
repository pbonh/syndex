use bevy_ecs::prelude::*;
use llhd::ir::{Inst, InstData};

#[derive(Debug, Default, PartialEq, Eq, Component)]
pub struct LLHDInstComponent {
    pub(crate) id: Option<Inst>,
    pub(crate) data: InstData,
}

impl From<&(Inst, InstData)> for LLHDInstComponent {
    fn from(inst: &(Inst, InstData)) -> Self {
        Self {
            id: Some(inst.0),
            data: inst.1.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llhd::common::filter_nullary;
    use llhd::ir::prelude::*;
    use llhd::table::TableKey;

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
    fn create_inst_component_default() {
        let _unit_component = LLHDInstComponent::default();
    }

    #[test]
    fn create_inst_component() {
        let unit_data = build_entity(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let mut inst_components: Vec<LLHDInstComponent> = Default::default();
        unit.all_insts()
            .filter(|inst| filter_nullary(&unit, *inst))
            .for_each(|inst| {
                let inst_data = unit[inst].clone();
                inst_components.push(LLHDInstComponent::from(&(inst, inst_data)));
            });
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
