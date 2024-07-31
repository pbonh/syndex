use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
// use super::{LLHDNet, LModule};
use llhd::ir::{prelude::*, InstData};

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
pub struct InstBundle {
    pub(crate) unit: UnitIdComponent,
    pub(crate) id: InstIdComponent,
    pub(crate) data: InstDataComponent,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct ValueBundle {
    pub(crate) unit: UnitIdComponent,
    pub(crate) id: InstIdComponent,
    pub(crate) value: ValueComponent,
}
