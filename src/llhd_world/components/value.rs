use bevy_ecs::prelude::*;
use llhd::ir::{Value, ValueData};

#[derive(Debug, Default, Component)]
pub struct LLHDValueComponent {
    pub(crate) id: Option<Value>,
    pub(crate) data: ValueData,
}

impl From<&(Value, ValueData)> for LLHDValueComponent {
    fn from(value: &(Value, ValueData)) -> Self {
        Self {
            id: Some(value.0),
            data: value.1.clone(),
        }
    }
}

impl PartialEq for LLHDValueComponent {
    fn eq(&self, other: &Self) -> bool {
        if self.id.is_some() && other.id.is_some() {
            self.id.unwrap() == other.id.unwrap()
        } else {
            false
        }
    }
}

impl Eq for LLHDValueComponent {}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn create_value_component_default() {
        let _unit_component = LLHDValueComponent::default();
    }

    #[test]
    fn create_value_component() {
        let entity_data = build_entity(UnitName::anonymous(0));
        let entity = Unit::new(UnitId::new(0), &entity_data);
        let mut value_components: Vec<LLHDValueComponent> = Default::default();
        entity.args().for_each(|value| {
            let value_data = entity[value].clone();
            value_components.push(LLHDValueComponent::from(&(value, value_data)));
        });
        entity.all_insts().for_each(|inst| {
            if let Some(value) = entity.get_inst_result(inst) {
                let value_data = entity[value].clone();
                value_components.push(LLHDValueComponent::from(&(value, value_data)));
            }
        });
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
        if let ValueData::Inst { inst, .. } = value_components[8].data {
            let add_inst_data = &entity[inst];
            let opcode = add_inst_data.opcode();
            assert!(matches!(opcode, Opcode::Add), "Inst should be Add type.");
        } else {
            panic!("Value(8) should correspond to an add inst.");
        }
        assert_eq!(
            Value::new(8),
            value_components[8].id.unwrap(),
            "Last Id should be Value with Id: 8"
        );
    }
}
