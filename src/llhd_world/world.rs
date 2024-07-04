use bevy_ecs::prelude::{Component, Entity, QueryState};
use bevy_ecs::query::QueryData;
use bevy_hierarchy::{BuildWorldChildren, Children};
use hypergraph::VertexIndex;
use llhd::ir::{Inst, UnitId, Value};
use std::collections::{BTreeSet, HashMap};
use std::ops::Add;

use crate::llhd_world::initializer::{
    build_blocks, build_insts, build_units, build_value_defs, build_value_refs,
};
use crate::{llhd::module::LLHDModule, world::LWorld};

// use super::components::inst::LLHDInstComponent;
// use super::components::unit::LLHDUnitComponent;
// use super::components::block::LLHDBlockComponent;
use super::components::inst::LLHDInstComponent;

pub type InstIndex = (UnitId, Inst);
pub type ValueDefIndex = (UnitId, Value);
pub type ValueRefIndex = (UnitId, Inst, Value);
pub type AnalogCircuitIndex = BTreeSet<VertexIndex>;
type UnitMapper = HashMap<UnitId, Entity>;
type InstMapper = HashMap<InstIndex, Entity>;
type ValueDefMapper = HashMap<ValueDefIndex, Entity>;
type ValueRefMapper = HashMap<ValueRefIndex, Entity>;

#[derive(Debug, Clone, Default, Component)]
pub struct ECSEntityName(String);

impl Add for ECSEntityName {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + &other.0)
    }
}

#[derive(Debug, Default)]
pub struct LLHDWorld {
    world: LWorld,
    unit_map: UnitMapper,
    inst_map: InstMapper,
    value_def_map: ValueDefMapper,
    value_ref_map: ValueRefMapper,
}

impl LLHDWorld {
    pub fn new(module: LLHDModule) -> Self {
        let mut world = LWorld::default();
        let mut unit_map = UnitMapper::default();
        let mut inst_map = InstMapper::default();
        let mut value_def_map = ValueDefMapper::default();
        let mut value_ref_map = ValueRefMapper::default();
        build_units(&module).for_each(|unit_component| {
            if let Some(unit_id) = unit_component.id {
                let unit_name = ECSEntityName(unit_component.name.to_string());
                let mut unit_entity = world.spawn(unit_component);
                unit_entity
                    .insert(unit_name.to_owned())
                    .with_children(|parent_unit| {
                        build_blocks(&module.unit(unit_id)).for_each(|block_component| {
                            if let Some(block_id) = block_component.id {
                                let block_name = unit_name.to_owned()
                                    + ECSEntityName(".".to_string())
                                    + ECSEntityName(
                                        block_component
                                            .to_owned()
                                            .data
                                            .name
                                            .unwrap_or(block_id.to_string()),
                                    );
                                parent_unit
                                    .spawn(block_component)
                                    .insert(block_name)
                                    .with_children(|parent_block| {
                                        build_insts(&module.unit(unit_id), block_id).for_each(
                                            |inst_component| {
                                                if let Some(inst_id) = inst_component.id {
                                                    let inst_name = unit_name.to_owned()
                                                        + ECSEntityName(".".to_string())
                                                        + ECSEntityName(inst_id.to_string());
                                                    let inst_data = inst_component.data.to_owned();
                                                    let mut inst_entity =
                                                        parent_block.spawn(inst_component);
                                                    inst_entity.insert(inst_name).with_children(
                                                        |parent_inst| {
                                                            build_value_refs(inst_id, &inst_data)
                                                                .for_each(|value_ref_component| {
                                                                    let value_def_id = value_ref_component.id.expect("Unexpected missing Value Def in ValueRef Component.");
                                                                    let value_ref_entity =
                                                                        parent_inst.spawn(
                                                                            value_ref_component,
                                                                        );
                                                                    value_ref_map.insert(
                                                                        (unit_id, inst_id, value_def_id),
                                                                        value_ref_entity.id(),
                                                                    );
                                                                });
                                                        },
                                                    );
                                                    inst_map
                                                        .insert((unit_id, inst_id), inst_entity.id());
                                                }
                                            },
                                        );
                                    });
                            }
                        });
                        build_value_defs(&module.unit(unit_id)).for_each(|value_component| {
                            if let Some(value_id) = value_component.id {
                                let value_name = unit_name.to_owned()
                                    + ECSEntityName(".".to_string())
                                    + ECSEntityName(value_id.to_string());
                                let mut value_def_entity = parent_unit.spawn(value_component);
                                value_def_entity.insert(value_name);
                                value_def_map.insert((unit_id, value_id), value_def_entity.id());
                            }
                        });
                    });
                unit_map.insert(unit_id, unit_entity.id());
            }
        });
        world.insert_resource(module);
        Self {
            world,
            unit_map,
            inst_map,
            value_def_map,
            value_ref_map,
        }
    }

    pub fn module(&self) -> &LLHDModule {
        self.world()
            .get_resource::<LLHDModule>()
            .expect("Missing LLHDModule")
    }

    pub const fn world(&self) -> &LWorld {
        &self.world
    }

    pub fn get_unit<T: Component>(&self, unit_id: UnitId) -> Option<&T> {
        let entity = self.unit_map[&unit_id];
        self.world.get::<T>(entity)
    }

    pub fn get_inst<T: Component>(&self, unit_id: UnitId, inst_id: Inst) -> Option<&T> {
        let entity = self.inst_map[&(unit_id, inst_id)];
        self.world.get::<T>(entity)
    }

    pub fn set_inst<T: Component>(&mut self, unit_id: UnitId, inst_id: Inst, value: T) {
        let entity = self.inst_map[&(unit_id, inst_id)];
        let mut entity_mut = self
            .world
            .get_entity_mut(entity)
            .expect("Unexpected missing entity.");
        entity_mut.insert(value);
    }

    pub fn get_value_def<T: Component>(&self, unit_id: UnitId, value_id: Value) -> Option<&T> {
        let entity = self.value_def_map[&(unit_id, value_id)];
        self.world.get::<T>(entity)
    }

    pub fn get_value_ref<T: Component>(
        &self,
        unit_id: UnitId,
        inst_id: Inst,
        value_id: Value,
    ) -> Option<&T> {
        let entity = self.value_ref_map[&(unit_id, inst_id, value_id)];
        self.world.get::<T>(entity)
    }

    pub fn slow_lookup(&mut self, name: &str) -> Option<Entity> {
        let mut entity_name_query = self.world.query::<(Entity, &ECSEntityName)>();
        entity_name_query
            .iter(&self.world)
            .find(|(_entity, ecs_entity_name_component)| ecs_entity_name_component.0 == name)
            .unzip()
            .0
    }

    pub fn query<D: QueryData>(&mut self) -> QueryState<D, ()> {
        self.world.query::<D>()
    }

    pub fn unit_program_inst<T: Component + Clone>(
        &self,
        unit_id: UnitId,
    ) -> impl Iterator<Item = (InstIndex, LLHDInstComponent, T)> + '_ {
        let unit_entity = self.unit_map[&unit_id];
        let unit_children = self
            .world
            .get::<Children>(unit_entity)
            .expect("Unit should contain child entities.");
        unit_children
            .iter()
            .filter(|block_entity| self.world.get::<Children>(**block_entity).is_some())
            .flat_map(move |block_entity| {
                let block_children = self
                    .world
                    .get::<Children>(*block_entity)
                    .expect("Block should contain child entities.");
                block_children
                    .iter()
                    .map(move |inst_entity| {
                        let inst_component = self
                            .world
                            .get::<LLHDInstComponent>(*inst_entity)
                            .expect("Inst entity should be present.");
                        let inst_id = inst_component.id.expect("Inst should have Id.");
                        (inst_id, inst_entity, inst_component.to_owned())
                    })
                    .filter(|(_inst_id, inst_entity, _inst_component)| {
                        self.world.get::<T>(**inst_entity).is_some()
                    })
                    .map(move |(inst_id, inst_entity, inst_component)| {
                        let inst_data = self
                            .world
                            .get::<T>(*inst_entity)
                            .cloned()
                            .expect("Inst Component data should be present.");
                        ((unit_id, inst_id), inst_component.to_owned(), inst_data)
                    })
            })
    }
}

impl From<LLHDModule> for LLHDWorld {
    fn from(module: LLHDModule) -> Self {
        Self::new(module)
    }
}

#[cfg(test)]
mod tests {
    use crate::llhd_world::components::{
        block::LLHDBlockComponent, inst::LLHDInstComponent, unit::LLHDUnitComponent,
        value::LLHDValueDefComponent, value::LLHDValueRefComponent,
    };
    use bevy_hierarchy::{Children, Parent};
    use itertools::Itertools;
    use llhd::{
        ir::{Inst, InstData, Opcode},
        table::TableKey,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn create_default_llhd_world() {
        let _ = LLHDWorld::default();
    }

    #[test]
    fn create_empty_llhd_world() {
        let llhd_module = LLHDModule::default();
        let _llhd_world = LLHDWorld::new(llhd_module);
    }

    #[test]
    fn create_llhd_world_hierarchy() {
        let input = indoc::indoc! {"
            proc %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %in3_prb = prb i1$ %in2
                %and1 = and i1 %in1_prb, %in2_prb
                %and2 = and i1 %in3_prb, %and1
                drv i1$ %out1, %and2, %epsilon
                wait %init for %epsilon
            }

            entity @top () -> () {
                %epsilon = const time 0s 1e
                %top_input1 = const i1 0
                %in1 = sig i1 %top_input1
                %top_input2 = const i1 1
                %in2 = sig i1 %top_input2
                %top_input3 = const i1 1
                %in3 = sig i1 %top_input3
                %top_out1 = const i1 0
                %out1 = sig i1 %top_out1
                inst %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1)
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let mut llhd_world = LLHDWorld::new(LLHDModule::from(module));
        assert!(
            llhd_world.world().get_resource::<LLHDModule>().is_some(),
            "LLHDWorld should contain a LLHDModule resource."
        );

        let sub_module_name = "%top.and";
        let top_module_name = "@top";
        assert!(
            llhd_world.slow_lookup(&sub_module_name).is_some(),
            "%top.and should be present name to lookup in ECS."
        );
        assert!(
            llhd_world.slow_lookup(&top_module_name).is_some(),
            "@top should be present name to lookup in ECS."
        );

        let sub_module_name_first_value = "v0";
        let sub_module_name_last_value = "v9";
        let top_module_name_first_value = "v0";
        let top_module_name_last_value = "v8";
        let sub_module_name_first_value_full_name =
            sub_module_name.to_owned() + "." + sub_module_name_first_value;
        let sub_module_name_last_value_full_name =
            sub_module_name.to_owned() + "." + sub_module_name_last_value;
        let top_module_name_first_value_full_name =
            top_module_name.to_owned() + "." + top_module_name_first_value;
        let top_module_name_last_value_full_name =
            top_module_name.to_owned() + "." + top_module_name_last_value;
        let sub_module_name_first_value_entity =
            llhd_world.slow_lookup(&sub_module_name_first_value_full_name);
        let sub_module_name_last_value_entity =
            llhd_world.slow_lookup(&sub_module_name_last_value_full_name);
        let top_module_name_first_value_entity =
            llhd_world.slow_lookup(&top_module_name_first_value_full_name);
        let top_module_name_last_value_entity =
            llhd_world.slow_lookup(&top_module_name_last_value_full_name);
        assert!(
            sub_module_name_first_value_entity.is_some(),
            "%top.and.v0 should be present name to lookup in ECS."
        );
        assert!(
            sub_module_name_last_value_entity.is_some(),
            "%top.and.v9 should be present name to lookup in ECS."
        );
        assert!(
            top_module_name_first_value_entity.is_some(),
            "@top.v0 should be present name to lookup in ECS."
        );
        assert!(
            top_module_name_last_value_entity.is_some(),
            "@top.v9 should be present name to lookup in ECS."
        );

        let sub_module_name_first_block = "init";
        let top_module_name_first_block = "bb0";
        let sub_module_name_first_block_full_name =
            sub_module_name.to_owned() + "." + sub_module_name_first_block;
        let top_module_name_first_block_full_name =
            top_module_name.to_owned() + "." + top_module_name_first_block;
        let sub_module_name_first_block_entity =
            llhd_world.slow_lookup(&sub_module_name_first_block_full_name);
        let top_module_name_first_block_entity =
            llhd_world.slow_lookup(&top_module_name_first_block_full_name);
        assert!(
            sub_module_name_first_block_entity.is_some(),
            "%top.and.init should be present name to lookup in ECS."
        );
        assert!(
            top_module_name_first_block_entity.is_some(),
            "@top.bb0 should be present name to lookup in ECS."
        );

        let sub_module_name_first_inst = "i0";
        let sub_module_name_last_inst = "i7";
        let top_module_name_first_inst = "i2";
        let top_module_name_last_inst = "i10";
        let sub_module_name_first_inst_full_name =
            sub_module_name.to_owned() + "." + sub_module_name_first_inst;
        let sub_module_name_last_inst_full_name =
            sub_module_name.to_owned() + "." + sub_module_name_last_inst;
        let top_module_name_first_inst_full_name =
            top_module_name.to_owned() + "." + top_module_name_first_inst;
        let top_module_name_last_inst_full_name =
            top_module_name.to_owned() + "." + top_module_name_last_inst;
        let sub_module_name_first_inst_entity =
            llhd_world.slow_lookup(&sub_module_name_first_inst_full_name);
        let sub_module_name_last_inst_entity =
            llhd_world.slow_lookup(&sub_module_name_last_inst_full_name);
        let top_module_name_first_inst_entity =
            llhd_world.slow_lookup(&top_module_name_first_inst_full_name);
        let top_module_name_last_inst_entity =
            llhd_world.slow_lookup(&top_module_name_last_inst_full_name);
        assert!(
            sub_module_name_first_inst_entity.is_some(),
            "%top.and.i0 should be present name to lookup in ECS."
        );
        assert!(
            sub_module_name_last_inst_entity.is_some(),
            "%top.and.i7 should be present name to lookup in ECS."
        );
        assert!(
            top_module_name_first_inst_entity.is_some(),
            "@top.i1 should be present name to lookup in ECS."
        );
        assert!(
            top_module_name_last_inst_entity.is_some(),
            "@top.i9 should be present name to lookup in ECS."
        );

        let sub_module_name_first_inst_data = llhd_world
            .world()
            .get::<LLHDInstComponent>(sub_module_name_first_inst_entity.unwrap())
            .unwrap();
        assert!(
            matches!(
                sub_module_name_first_inst_data.data.opcode(),
                llhd::ir::Opcode::ConstTime
            ),
            "First Instruction in sub-module should be ConstTime."
        );
        let sub_module_name_last_inst_data = llhd_world
            .world()
            .get::<LLHDInstComponent>(sub_module_name_last_inst_entity.unwrap())
            .unwrap();
        assert!(
            matches!(
                sub_module_name_last_inst_data.data.opcode(),
                llhd::ir::Opcode::WaitTime
            ),
            "Last Instruction in sub-module should be Wait."
        );

        let top_module_name_first_inst_data = llhd_world
            .world()
            .get::<LLHDInstComponent>(top_module_name_first_inst_entity.unwrap())
            .unwrap();
        assert!(
            matches!(
                top_module_name_first_inst_data.data.opcode(),
                llhd::ir::Opcode::ConstInt
            ),
            "First Instruction in top-module should be ConstInt."
        );
        let top_module_name_last_inst_data = llhd_world
            .world()
            .get::<LLHDInstComponent>(top_module_name_last_inst_entity.unwrap())
            .unwrap();
        assert!(
            matches!(
                top_module_name_last_inst_data.data.opcode(),
                llhd::ir::Opcode::Inst
            ),
            "Last Instruction in top-module should be Inst."
        );

        let mut unit_component_count = 0;
        let mut unit_query = llhd_world.query::<(Entity, &Children, &LLHDUnitComponent)>();
        unit_query.iter(llhd_world.world()).for_each(
            |(parent_unit_entity, child_entity, _unit_component)| {
                unit_component_count += 1;
                let unit_name = llhd_world
                    .world()
                    .get::<LLHDUnitComponent>(parent_unit_entity)
                    .unwrap()
                    .name
                    .to_string();
                if unit_name == sub_module_name {
                    assert_eq!(
                        11,
                        child_entity.len(),
                        "There should be 11 child nodes(10 Values + 1 Block) in %top.and module."
                    );
                } else if unit_name == top_module_name {
                    assert_eq!(
                        10,
                        child_entity.len(),
                        "There should be 10 child nodes(9 Values + 1 Block) in @top module."
                    );
                } else {
                    panic!("Unknown module name: {}", unit_name);
                }
            },
        );
        assert_eq!(
            2, unit_component_count,
            "There should be 2 Unit Components present in Module."
        );

        let mut sub_block_component_count = 0;
        let mut top_block_component_count = 0;
        let mut block_query = llhd_world.query::<(&Children, &LLHDBlockComponent)>();
        block_query
            .iter(llhd_world.world())
            .for_each(|(child_insts, block_component)| {
                let block_id = block_component.id.unwrap();
                let block_name = block_component
                    .data
                    .name
                    .clone()
                    .unwrap_or_else(|| block_id.to_string());
                if block_name == sub_module_name_first_block {
                    sub_block_component_count += 1;
                    assert_eq!(
                        8,
                        child_insts.len(),
                        "There should be 8 child nodes(8 Insts) in %top.and.init block."
                    );
                    let const_int_inst =
                        llhd_world.world().get::<LLHDInstComponent>(child_insts[0]);
                    let const_int_inst_opcode = const_int_inst.unwrap().data.opcode();
                    assert!(
                        matches!(const_int_inst_opcode, llhd::ir::Opcode::ConstTime),
                        "First Inst of %top.and should have Opcode ConstTime."
                    );
                    let wait_inst =
                        llhd_world.world().get::<LLHDInstComponent>(child_insts[7]);
                    let wait_inst_opcode = wait_inst.unwrap().data.opcode();
                    assert!(
                        matches!(wait_inst_opcode, llhd::ir::Opcode::WaitTime),
                        "Last Inst of %top.and should have Opcode WaitTime."
                    );
                } else if block_name == top_module_name_first_block {
                    top_block_component_count += 1;
                    assert_eq!(
                        11,
                        child_insts.len(),
                        "There should be 11 child nodes(11 Insts) in %top.and.bb0 block."
                    );
                    let const_int_inst =
                        llhd_world.world().get::<LLHDInstComponent>(child_insts[0]);
                    let const_int_inst_opcode = const_int_inst.unwrap().data.opcode();
                    assert!(
                        matches!(const_int_inst_opcode, llhd::ir::Opcode::ConstTime),
                        "First Inst of @top should have Opcode ConstTime."
                    );
                    let instantiation_inst =
                        llhd_world.world().get::<LLHDInstComponent>(child_insts[9]);
                    let instantiation_inst_opcode = instantiation_inst.unwrap().data.opcode();
                    assert!(
                        matches!(instantiation_inst_opcode, llhd::ir::Opcode::Inst),
                        "Last Inst(not include the very last, which is nullary) of @top should have Opcode Inst."
                    );
                } else {
                    panic!("Unknown module name: {}", block_name);
                }
            }
        );
        assert_eq!(
            1, sub_block_component_count,
            "There should be 1 Block Components present in %top.and."
        );
        assert_eq!(
            1, top_block_component_count,
            "There should be 1 Block Components present in @top."
        );

        let inst_count = llhd_world
            .query::<&LLHDInstComponent>()
            .iter(llhd_world.world())
            .count();
        assert_eq!(
            19, inst_count,
            "There should be 19 total Insts in ECS & Module."
        );
        let mut inst_component_count = 0;
        let mut inst_with_args_query = llhd_world.query::<(&Children, &LLHDInstComponent)>();
        inst_with_args_query.iter(llhd_world.world()).for_each(
            |(child_value_refs, inst_component)| {
                inst_component_count += 1;
                let inst_opcode = inst_component.data.opcode();
                if inst_opcode == Opcode::ConstTime {
                    assert_eq!(
                        0,
                        child_value_refs.len(),
                        "There should be 0 child nodes(0 Args) in ConstTime Insts."
                    );
                } else if inst_opcode == Opcode::Prb {
                    assert_eq!(
                        1,
                        child_value_refs.len(),
                        "There should be 1 child nodes(1 Args) in Prb Insts."
                    );
                    let _value_ref_component = llhd_world
                        .world()
                        .get::<LLHDValueRefComponent>(child_value_refs[0])
                        .unwrap();
                } else if inst_opcode == Opcode::And {
                    assert_eq!(
                        2,
                        child_value_refs.len(),
                        "There should be 2 child nodes(2 Args) in And Insts."
                    );
                    let _value_ref_component = llhd_world
                        .world()
                        .get::<LLHDValueRefComponent>(child_value_refs[0])
                        .unwrap();
                } else if inst_opcode == Opcode::Drv {
                    assert_eq!(
                        3,
                        child_value_refs.len(),
                        "There should be 3 child nodes(3 Args) in Drv Insts."
                    );
                    let _value_ref_component = llhd_world
                        .world()
                        .get::<LLHDValueRefComponent>(child_value_refs[0])
                        .unwrap();
                } else if inst_opcode == Opcode::WaitTime {
                    assert_eq!(
                        1,
                        child_value_refs.len(),
                        "There should be 1 child nodes(1 Args) in WaitTime Insts."
                    );
                    let _value_ref_component = llhd_world
                        .world()
                        .get::<LLHDValueRefComponent>(child_value_refs[0])
                        .unwrap();
                } else if inst_opcode == Opcode::Sig {
                    assert_eq!(
                        1,
                        child_value_refs.len(),
                        "There should be 1 child nodes(1 Args) in Sig Insts."
                    );
                    let _value_ref_component = llhd_world
                        .world()
                        .get::<LLHDValueRefComponent>(child_value_refs[0])
                        .unwrap();
                } else if inst_opcode == Opcode::Inst {
                    assert_eq!(
                        4,
                        child_value_refs.len(),
                        "There should be 4 child nodes(4 Args) in Inst Insts."
                    );
                    let _value_ref_component = llhd_world
                        .world()
                        .get::<LLHDValueRefComponent>(child_value_refs[0])
                        .unwrap();
                } else {
                    panic!("Unknown Inst Opcode: {}", inst_opcode);
                }
            },
        );
        assert_eq!(
            12, inst_component_count,
            "There should be 12 Inst Components(w/ Child Nodes) present in Module."
        );
    }

    #[test]
    fn create_llhd_world_with_blocks() {
        let input = indoc::indoc! {"
            declare @bar (i32, i9) i32

            func @foo (i32 %x, i8 %y) i32 {
            %entry:
                %asdf0 = const i32 42
                %1 = const time 1.489ns 10d 9e
                %hello = alias i32 %asdf0
                %2 = not i32 %asdf0
                %3 = neg i32 %2
                %4 = add i32 %2, %3
                %5 = sub i32 %2, %3
                %6 = and i32 %2, %3
                %7 = or i32 %2, %3
                %8 = xor i32 %2, %3
                %cmp = eq i32 %7, %7
                br %cmp, %entry, %next
            %next:
                %a = exts i9, i32 %7, 4, 9
                %b = neg i9 %a
                %r = call i32 @bar (i32 %8, i9 %b)
                %many = [32 x i9 %b]
                %some = exts [9 x i9], [32 x i9] %many, 2, 9
                %one = extf i9, [9 x i9] %some, 3
                neg i9 %one
                ret i32 %3
            }

            entity @magic (i32$ %data, i1$ %clk) -> (i32$ %out) {
                %datap = prb i32$ %data
                %cmp = const i1 0
                reg i32$ %out, [%datap, rise %cmp]
            }
        "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let mut llhd_world = LLHDWorld::new(LLHDModule::from(module));

        let magic_entity_name = "@magic";
        let func_name = "@foo";
        let func_entry_name = "entry";
        let func_next_name = "next";
        let func_entry_full_name = func_name.to_owned() + "." + func_entry_name;
        let func_next_full_name = func_name.to_owned() + "." + func_next_name;
        assert!(
            llhd_world.slow_lookup(&magic_entity_name).is_some(),
            "@magic should be present name to lookup in ECS."
        );
        assert!(
            llhd_world.slow_lookup(&func_name).is_some(),
            "@foo should be present name to lookup in ECS."
        );
        assert!(
            llhd_world.slow_lookup(&func_entry_full_name).is_some(),
            "@foo.entry should be present name to lookup in ECS."
        );
        assert!(
            llhd_world.slow_lookup(&func_next_full_name).is_some(),
            "@foo.next should be present name to lookup in ECS."
        );

        let mut blocks: Vec<LLHDBlockComponent> = Default::default();
        let mut block1_insts: Vec<LLHDInstComponent> = Default::default();
        let mut block2_insts: Vec<LLHDInstComponent> = Default::default();
        let mut block_query = llhd_world.query::<(Entity, &Parent, &LLHDBlockComponent)>();
        block_query.iter(llhd_world.world()).for_each(
            |(block_entity, parent_unit_entity, block_component)| {
                let parent_unit_component = llhd_world
                    .world()
                    .get::<LLHDUnitComponent>(**parent_unit_entity);
                let parent_unit_name = parent_unit_component.unwrap().name.to_string();
                if parent_unit_name == func_name {
                    blocks.push(block_component.to_owned());
                    let block_id = block_component.id.unwrap();
                    let block_name = block_component
                        .data
                        .name
                        .clone()
                        .unwrap_or_else(|| block_id.to_string());
                    let insts = llhd_world
                        .world()
                        .get::<Children>(block_entity)
                        .unwrap()
                        .to_vec()
                        .iter()
                        .map(|inst_entity| {
                            llhd_world
                                .world()
                                .get::<LLHDInstComponent>(*inst_entity)
                                .unwrap()
                                .to_owned()
                        })
                        .collect_vec();
                    if block_name == func_entry_name {
                        block1_insts = insts;
                    } else if block_name == func_next_name {
                        block2_insts = insts;
                    }
                }
            },
        );
        assert_eq!(
            2,
            blocks.len(),
            "2 Blocks should be present in @foo function."
        );
        assert_eq!(
            12,
            block1_insts.len(),
            "12 Insts should be present in @foo.entry block."
        );
        assert_eq!(
            8,
            block2_insts.len(),
            "8 Insts should be present in @foo.next block."
        );
    }

    #[test]
    fn get_llhd_world_entity_components() {
        let input = indoc::indoc! {"
            entity @test_entity (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
                %null = const time 0s 1e
                %and1 = and i1 %in1, %in2
                %and2 = and i1 %in3, %in4
                %or1 = or i1 %and1, %and2
                drv i1$ %out1, %or1, %null
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_world = LLHDWorld::new(LLHDModule::from(module));
        let unit_id = UnitId::new(0);
        let unit_component = llhd_world.get_unit::<LLHDUnitComponent>(unit_id).unwrap();
        assert_eq!(
            "@test_entity",
            unit_component.name.to_string(),
            "Unit name should be '@test_entity'"
        );

        let inst_id = Inst::new(4);
        let inst_component = llhd_world
            .get_inst::<LLHDInstComponent>(unit_id, inst_id)
            .unwrap();
        if let InstData::Binary { opcode, args } = inst_component.data {
            assert!(
                matches!(opcode, llhd::ir::Opcode::Or),
                "Fourth Instruction in sub-module should be Or."
            );

            let value_def = args[0];
            let value_def_component = llhd_world
                .get_value_def::<LLHDValueDefComponent>(unit_id, value_def)
                .unwrap();

            let value_ref_component = llhd_world
                .get_value_ref::<LLHDValueRefComponent>(unit_id, inst_id, value_def)
                .unwrap();
            assert_eq!(
                value_ref_component.id.unwrap(),
                value_def_component.id.unwrap(),
                "Def stored in ValueRef should match original Id Value."
            );
        } else {
            panic!("Unknown Inst");
        }
    }

    #[test]
    fn llhd_world_unit_inst_program() {
        let input = indoc::indoc! {"
            entity @test_entity (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
                %null = const time 0s 1e
                %and1 = and i1 %in1, %in2
                %and2 = and i1 %in3, %in4
                %or1 = or i1 %and1, %and2
                drv i1$ %out1, %or1, %null
            }
        "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_world = LLHDWorld::new(LLHDModule::from(module));
        let unit_id = UnitId::new(0);
        let unit_program_inst = llhd_world
            .unit_program_inst::<LLHDInstComponent>(unit_id)
            .collect::<Vec<(InstIndex, LLHDInstComponent, LLHDInstComponent)>>();
        assert_eq!(
            6,
            unit_program_inst.len(),
            "There should be 6 Instructions in the Unit Program."
        );
    }
}
