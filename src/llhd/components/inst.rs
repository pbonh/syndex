use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use llhd::ir::prelude::*;
use llhd::ir::InstData;

use super::unit::{UnitIdComponent, ValueComponent};

pub type InstIndex = (UnitId, Inst);
pub type ValueIndex = (UnitId, Inst, Value);

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct InstIdComponent {
    pub(crate) id: Inst,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct InstDataComponent {
    pub(crate) data: InstData,
}

#[derive(Debug, Clone, PartialEq, Eq, Bundle)]
pub struct GateBundle {
    pub(crate) unit: UnitIdComponent,
    pub(crate) id: InstIdComponent,
    pub(crate) data: InstDataComponent,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct ValueRefBundle {
    pub(crate) unit: UnitIdComponent,
    pub(crate) id: InstIdComponent,
    pub(crate) value: ValueComponent,
}
