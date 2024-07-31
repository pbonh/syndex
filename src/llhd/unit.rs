use bevy_ecs::component::Component;
use llhd::ir::prelude::{UnitData, UnitKind, UnitName, Value};
use llhd::ir::UnitId;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct UnitIdComponent {
    pub(crate) id: UnitId,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct UnitNameComponent {
    pub(crate) name: UnitName,
    pub(crate) kind: UnitKind,
}

impl From<&UnitData> for UnitNameComponent {
    fn from(unit: &UnitData) -> Self {
        Self {
            name: unit.name.clone(),
            kind: unit.kind,
        }
    }
}

impl Default for UnitNameComponent {
    fn default() -> Self {
        Self {
            name: UnitName::anonymous(0),
            kind: llhd::ir::UnitKind::Entity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct UnitArgComponent {
    pub(crate) unit: UnitId,
    pub(crate) arg: Value,
}

#[cfg(test)]
mod tests {
    use llhd::ir::prelude::*;

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
    fn create_unit_component() {
        let entity = build_entity(UnitName::anonymous(0));
        let _unit_component = UnitNameComponent::from(&entity);
    }
}
