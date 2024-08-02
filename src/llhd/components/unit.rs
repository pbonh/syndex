use bevy_ecs::prelude::{Bundle, Component};
use llhd::ir::prelude::{Unit, UnitKind, UnitName, Value};
use llhd::ir::{Signature, UnitId};

pub type UnitIndex = UnitId;
pub type UnitArgIndex = (UnitId, Value);

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct UnitIdComponent {
    pub(crate) id: UnitId,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct UnitNameComponent {
    pub(crate) name: UnitName,
    pub(crate) kind: UnitKind,
    pub(crate) signature: Signature,
}

impl From<&Unit<'_>> for UnitNameComponent {
    fn from(unit: &Unit) -> Self {
        Self {
            name: unit.name().clone(),
            kind: unit.kind(),
            signature: unit.sig().clone(),
        }
    }
}

impl Default for UnitNameComponent {
    fn default() -> Self {
        Self {
            name: UnitName::anonymous(0),
            kind: llhd::ir::UnitKind::Entity,
            signature: Signature::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Bundle)]
pub struct UnitBundle {
    pub(crate) unit: UnitIdComponent,
    pub(crate) name: UnitNameComponent,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct ValueComponent {
    pub(crate) value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Bundle)]
pub struct PortBundle {
    pub(crate) unit: UnitIdComponent,
    pub(crate) arg: ValueComponent,
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
        let input = indoc::indoc! {"
            entity @test_entity (i1 %clk, i1 %rst, i1$ %inp) -> (i1$ %out1) {
                %v1 = const i1 0
                %v2 = const i1 1
                %v3 = add i1 %v1, %v2
                %inp_prb = prb i1$ %inp
                %sum = add i1 %v3, %inp_prb
                %instant = const time 0s 1e
                drv i1$ %out1, %sum, %instant
            }
        "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let test_unit = module.units().next().unwrap();
        let _unit_component = UnitNameComponent::from(&test_unit);
    }
}
