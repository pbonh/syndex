use bevy_ecs::prelude::*;
use llhd::ir::{Inst, Value, ValueData};
use std::cmp::Ordering;

#[derive(Debug, Clone, Default, Component)]
pub struct LLHDValueDefComponent {
    pub(crate) id: Option<Value>,
    pub(crate) data: ValueData,
}

impl From<&(Value, ValueData)> for LLHDValueDefComponent {
    fn from(value: &(Value, ValueData)) -> Self {
        Self {
            id: Some(value.0),
            data: value.1.clone(),
        }
    }
}

impl PartialOrd for LLHDValueDefComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LLHDValueDefComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id.is_some() && other.id.is_some() {
            self.id.cmp(&other.id)
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for LLHDValueDefComponent {
    fn eq(&self, other: &Self) -> bool {
        if self.id.is_some() && other.id.is_some() {
            self.id.unwrap() == other.id.unwrap()
        } else {
            false
        }
    }
}

impl Eq for LLHDValueDefComponent {}

#[derive(Debug, Clone, Default, PartialOrd, Ord, Component)]
pub struct LLHDValueRefComponent {
    pub(crate) id: Option<Value>,
    pub(crate) inst: Option<Inst>,
}

impl From<&(Value, Inst)> for LLHDValueRefComponent {
    fn from(info: &(Value, Inst)) -> Self {
        Self {
            id: Some(info.0),
            inst: Some(info.1),
        }
    }
}

impl PartialEq for LLHDValueRefComponent {
    fn eq(&self, other: &Self) -> bool {
        if (self.id.is_some() && other.id.is_some())
            && (self.inst.is_some() && other.inst.is_some())
        {
            (self.id.unwrap() == other.id.unwrap()) && (self.inst.unwrap() == other.inst.unwrap())
        } else {
            false
        }
    }
}

impl Eq for LLHDValueRefComponent {}

#[cfg(test)]
mod tests {
    use super::*;
    use llhd::ir::prelude::*;
    use llhd::table::TableKey;
    use std::collections::BTreeSet;

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
    fn create_value_def_component_default() {
        let _unit_component = LLHDValueDefComponent::default();
    }

    #[test]
    fn create_value_ref_component_refault() {
        let _unit_component = LLHDValueRefComponent::default();
    }

    #[test]
    fn create_value_def_component() {
        let entity_data = build_entity(UnitName::anonymous(0));
        let entity = Unit::new(UnitId::new(0), &entity_data);
        let mut value_def_components: Vec<LLHDValueDefComponent> = Default::default();
        let mut value_ref_components: BTreeSet<LLHDValueRefComponent> = Default::default();
        entity.args().for_each(|value| {
            let value_data = entity[value].clone();
            value_def_components.push(LLHDValueDefComponent::from(&(value, value_data)));
        });
        entity.all_insts().for_each(|inst| {
            let inst_data = &entity[inst];
            inst_data.args().iter().for_each(|inst_arg| {
                value_ref_components.insert(LLHDValueRefComponent::from(&(*inst_arg, inst)));
            });
            if let Some(value) = entity.get_inst_result(inst) {
                let value_data = entity[value].clone();
                value_def_components.push(LLHDValueDefComponent::from(&(value, value_data)));
            }
        });
        assert_eq!(
            9,
            value_def_components.len(),
            "There should be 9 Values defined in Unit."
        );
        assert_eq!(
            5,
            value_ref_components.len(),
            "There should be 5 Value References in Unit."
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
        if let ValueData::Inst { inst, .. } = value_def_components[8].data {
            let add_inst_data = &entity[inst];
            let opcode = add_inst_data.opcode();
            assert!(matches!(opcode, Opcode::Add), "Inst should be Add type.");
        } else {
            panic!("Value(8) should correspond to an add inst.");
        }
        assert_eq!(
            Value::new(8),
            value_def_components[8].id.unwrap(),
            "Last Id should be Value with Id: 8"
        );
    }
}
