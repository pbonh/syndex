use llhd::ir::{ValueData,Value};

#[derive(Debug,Default)]
pub struct ValueComponent {
    pub(crate) id: Option<Value>,
    pub(crate) data: ValueData,
}

impl From<&(Value,ValueData)> for ValueComponent {
    fn from(value: &(Value,ValueData)) -> Self {
        Self {
            id: Some(value.0),
            data: value.1.clone(),
        }
    }
}

impl PartialEq for ValueComponent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ValueComponent {}

#[cfg(test)]
mod tests {
    use llhd::ir::prelude::*;
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
    fn create_value_component_default() {
        let _unit_component = ValueComponent::default();
    }

    #[test]
    fn create_value_component() {
        let entity_data = build_entity(UnitName::anonymous(0));
        let entity = Unit::new(UnitId::new(0), &entity_data);
        let mut value_components: Vec<ValueComponent> = Default::default();
        entity.args().for_each(|value| {
            let value_data = entity[value].clone();
            value_components.push(ValueComponent::from(&(value,value_data)));
        });
        entity.all_insts().for_each(|inst| {
            if let Some(value) = entity.get_inst_result(inst) {
                let value_data = entity[value].clone();
                value_components.push(ValueComponent::from(&(value,value_data)));
            }
        });
        assert_eq!(9, value_components.len(), "There should be 9 Values defined in Unit.");
    }
}
