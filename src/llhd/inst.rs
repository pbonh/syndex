use bevy_ecs::component::Component;
// use super::{LLHDNet, LModule};
use llhd::ir::{prelude::*, InstData};

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct InstComponent {
    pub(crate) unit: UnitId,
    pub(crate) id: Inst,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct InstDataComponent {
    pub(crate) data: InstData,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct InstValueComponent {
    pub(crate) unit: UnitId,
    pub(crate) id: Inst,
    pub(crate) value: Value,
}
