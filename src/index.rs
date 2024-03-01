pub mod macros;

use std::collections::HashMap;
use std::ops::Index;

use flecs::{Component, Entity};
use llhd::ir::{DeclData, DeclId, Signature, UnitBuilder, UnitId, UnitName};
use rayon::prelude::*;

use crate::llhd::module::LModule;
use crate::llhd::unit::UnitComponent;
use crate::llhd::LLHDNet;
use crate::world::LWorld;

type ComponentTypeSet = Vec<Entity>;
type LinkedUnitMap = HashMap<UnitId, Entity>;
type LinkedNetMap = HashMap<LLHDNet, Entity>;

/// Synthesis Database
#[derive(Debug)]
pub struct Syndex {
    module: LModule,
    world: LWorld,
    component_types: ComponentTypeSet,
    unit_map: LinkedUnitMap,
    net_map: LinkedNetMap,
}

impl Syndex {
    fn new() -> Self {
        Self {
            module: LModule::default(),
            world: LWorld::default(),
            component_types: Vec::default(),
            unit_map: HashMap::default(),
            net_map: HashMap::default(),
        }
    }

    fn load(&mut self, module: LModule) {
        module.module().units().for_each(|unit| {
            let unit_id = unit.id();
            module.all_nets(unit_id).for_each(|llhd_net| {
                let net_entity = self.world.entity();
                self.net_map.insert(llhd_net, net_entity);
            });
        });

        self.module = module;
    }

    #[must_use]
    pub const fn module(&self) -> &LModule {
        &self.module
    }

    #[must_use]
    pub const fn types(&self) -> &ComponentTypeSet {
        &self.component_types
    }

    // pub fn nets<T: 'static>(&self) -> impl Iterator<Item = (LLHDNet,T)> + '_ {
    //     self.net_map
    //         .iter()
    //         .map(|(net, net_entity)| (net, self.world.get::<T>(net_entity).unwrap()))
    // }

    pub fn add_unit(&mut self, name: &str) -> UnitId {
        let unit_id = self.module.add_unit(name);
        let unit_data = self.module.module().unit(unit_id).data();
        let unit_component = UnitComponent::from(unit_data);
        let unit_entity = self.world.entity().set(unit_component);
        self.unit_map.insert(unit_id, unit_entity);
        unit_id
    }

    pub fn remove_unit(&mut self, unit: UnitId) {
        self.module.remove_unit(unit);
    }

    pub fn declare(&mut self, name: UnitName, sig: Signature) -> DeclId {
        self.module.declare(name, sig)
    }

    pub fn add_decl(&mut self, data: DeclData) -> DeclId {
        self.module.add_decl(data)
    }

    pub fn remove_decl(&mut self, decl: DeclId) {
        self.module.remove_decl(decl);
    }

    pub fn units_mut<'a>(&'a mut self) -> impl Iterator<Item = UnitBuilder<'a>> + 'a {
        self.module.units_mut()
    }

    pub fn par_units_mut<'a>(&'a mut self) -> impl ParallelIterator<Item = UnitBuilder<'a>> + 'a {
        self.module.par_units_mut()
    }

    pub fn unit_mut(&mut self, unit: UnitId) -> UnitBuilder {
        self.module.unit_mut(unit)
    }

    pub fn link(&mut self) {
        self.module.link();
    }

    pub fn set_location_hint(&mut self, mod_unit: UnitId, loc: usize) {
        self.module.set_location_hint(mod_unit, loc);
    }

    #[must_use]
    pub const fn world(&self) -> &LWorld {
        &self.world
    }

    pub fn component<T: 'static>(&mut self) -> Entity {
        self.world.component::<T>()
    }

    #[must_use]
    pub fn get_net<T: Component>(&self, net: LLHDNet) -> Option<&T> {
        let net_entity = self.net_map[&net];
        self.world.get::<T>(net_entity)
    }

    pub fn add<T: Component>(&self, net: LLHDNet) {
        let net_entity = self.net_map[&net];
        self.world.add::<T>(net_entity);
    }

    pub fn set<T: Component>(&self, net: LLHDNet, value: T) {
        let net_entity = self.net_map[&net];
        self.world.set::<T>(net_entity, value);
    }
}

impl Index<UnitId> for Syndex {
    type Output = UnitComponent;

    fn index(&self, unit_id: UnitId) -> &Self::Output {
        let entity = self.unit_map[&unit_id];
        &self
            .world
            .get::<UnitComponent>(entity)
            .expect("Missing Entity or Component")
    }
}

#[cfg(test)]
mod tests {
    use llhd::ir::prelude::*;

    use super::*;
    use crate::create_index;
    use crate::llhd::common::filter_nullary;

    #[test]
    fn create_default_syndex() {
        let _index = create_index!(());
    }

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
    fn create_unit_data_in_world() {
        let mut index = create_index!(());
        let ent_id = index.add_unit("top");
        let unit_component_data = &index[ent_id];
        assert_eq!(
            "@top",
            unit_component_data.name.to_string(),
            "Unit name should be 'top'."
        );
        assert_eq!(
            UnitKind::Entity,
            unit_component_data.kind,
            "Unit type should be 'Entity'."
        );
    }

    #[derive(Default, Debug, PartialEq)]
    struct TimingNode {
        name: String,
        delay: f64,
    }

    #[derive(Default, Debug, PartialEq)]
    struct TimingEdge {
        delay: f64,
    }

    #[test]
    fn create_world_with_example_timing_data() {
        let input = indoc::indoc! {"
            entity @test_entity (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let index = create_index!(LModule::new(module), TimingNode, TimingEdge);
        assert_eq!(
            3,
            index.types().len(),
            "There should be 3 Component Types present in World."
        );
        let _: Vec<_> = index
            .module()
            .module()
            .units()
            .map(|unit| {
                let unit_id = unit.id();
                unit.blocks().for_each(|block| {
                    unit.insts(block)
                        .filter(|inst| filter_nullary(&unit, *inst))
                        .for_each(|inst| {
                            let inst_value = unit.get_inst_result(inst).unwrap();
                            let llhd_net = (unit_id, inst_value);
                            index.add::<TimingNode>(llhd_net);
                            // index.set::<TimingNode>(llhd_net, TimingNode::default());
                            let inst_component = index.get_net::<TimingNode>(llhd_net);
                            match inst_component {
                                None => panic!("No TimingNode Components Available for Entity."),
                                Some(_inst_component_data) => (),
                            }
                        })
                });
            })
            .collect();
        assert_eq!(
            2,
            index.world().count_component::<TimingNode>(),
            "There should be 2 TimingNodes present in World."
        );
        // for unit in index.module().units() {
        //     let unit_id = unit.id();
        //     for block in unit.blocks() {
        //         for inst in unit.insts(block) {
        //             let inst_value = unit.get_inst_result(inst).unwrap();
        //             let opcode = unit[inst].opcode();
        //             let inst_component_data =
        //                 index.get::<TimingNode>((unit_id, inst_value, opcode));
        //         }
        //     }
        // }
    }
}
