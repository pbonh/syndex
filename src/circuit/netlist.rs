use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;

use super::graph::LCircuit;

#[derive(Debug, Clone, Default, Resource, Component)]
pub struct LNetlist(LCircuit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_netlist() {
        let _netlist = LNetlist::default();
    }
}
