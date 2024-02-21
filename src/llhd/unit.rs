use llhd::ir::prelude::{UnitData, UnitKind, UnitName};

#[derive(Debug, PartialEq, Eq)]
pub struct UnitComponent {
    pub(crate) name: UnitName,
    pub(crate) kind: UnitKind,
}

impl From<&UnitData> for UnitComponent {
    fn from(unit: &UnitData) -> Self {
        Self {
            name: unit.name.clone(),
            kind: unit.kind,
        }
    }
}

impl Default for UnitComponent {
    fn default() -> Self {
        Self {
            name: UnitName::anonymous(0),
            kind: llhd::ir::UnitKind::Entity,
        }
    }
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
        let _unit_component = UnitComponent::from(&entity);
    }
}
