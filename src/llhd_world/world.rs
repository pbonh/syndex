use bevy_ecs::prelude::{Component, Entity, QueryState};
use bevy_ecs::query::QueryData;
use bevy_hierarchy::BuildWorldChildren;
use std::ops::Add;

use crate::llhd_world::initializer::{build_blocks, build_insts, build_units, build_values};
use crate::{llhd::module::LLHDModule, world::LWorld};

#[derive(Debug, Clone, Default, Component)]
pub struct ECSEntityName(String);

impl Add for ECSEntityName {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + &other.0)
    }
}

#[derive(Debug, Default)]
pub struct LLHDWorld(LWorld);

impl LLHDWorld {
    pub fn new(module: LLHDModule) -> Self {
        let mut world = LWorld::default();
        build_units(&module).for_each(|unit_component| {
            if let Some(unit_id) = unit_component.id {
                let unit_name = ECSEntityName(unit_component.name.to_string());
                let _unit_entity = world
                    .spawn(unit_component)
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
                                        build_insts(&module.unit(unit_id)).for_each(
                                            |inst_component| {
                                                if let Some(inst_id) = inst_component.id {
                                                    let inst_name = unit_name.to_owned()
                                                        + ECSEntityName(".".to_string())
                                                        + ECSEntityName(inst_id.to_string());
                                                    parent_block
                                                        .spawn(inst_component)
                                                        .insert(inst_name);
                                                }
                                            },
                                        );
                                    });
                            }
                        });
                        build_values(&module.unit(unit_id)).for_each(|value_component| {
                            if let Some(value_id) = value_component.id {
                                let value_name = unit_name.to_owned()
                                    + ECSEntityName(".".to_string())
                                    + ECSEntityName(value_id.to_string());
                                parent_unit.spawn(value_component).insert(value_name);
                            }
                        });
                    });
            }
        });
        world.insert_resource(module);
        Self(world)
    }

    pub const fn world(&self) -> &LWorld {
        &self.0
    }

    pub fn slow_lookup(&mut self, name: &str) -> Option<Entity> {
        let mut entity_name_query = self.0.query::<(Entity, &ECSEntityName)>();
        entity_name_query
            .iter(&self.0)
            .find(|(_entity, ecs_entity_name_component)| ecs_entity_name_component.0 == name)
            .unzip()
            .0
    }

    pub fn query<D: QueryData>(&mut self) -> QueryState<D, ()> {
        self.0.query::<D>()
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
    };
    use bevy_hierarchy::{Children, Parent};
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
    fn create_llhd_world() {
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

        let mut unit_query = llhd_world.query::<(Entity, &Children, &LLHDUnitComponent)>();
        unit_query.iter(llhd_world.world()).for_each(
            |(parent_unit_entity, child_entity, _unit_component)| {
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

        let mut block_query = llhd_world.query::<(&Children, &LLHDBlockComponent)>();
        block_query
            .iter(llhd_world.world())
            .for_each(|(child_entity, block_component)| {
                let block_id = block_component.id.unwrap();
                let block_name = block_component
                    .data
                    .name
                    .clone()
                    .unwrap_or_else(|| block_id.to_string());
                if block_name == sub_module_name_first_block {
                    assert_eq!(
                        8,
                        child_entity.len(),
                        "There should be 8 child nodes(8 Insts) in %top.and.init block."
                    );
                    let const_int_inst =
                        llhd_world.world().get::<LLHDInstComponent>(child_entity[0]);
                    let const_int_inst_opcode = const_int_inst.unwrap().data.opcode();
                    assert!(
                        matches!(const_int_inst_opcode, llhd::ir::Opcode::ConstTime),
                        "First Inst of %top.and should have Opcode ConstTime."
                    );
                    let wait_inst =
                        llhd_world.world().get::<LLHDInstComponent>(child_entity[7]);
                    let wait_inst_opcode = wait_inst.unwrap().data.opcode();
                    assert!(
                        matches!(wait_inst_opcode, llhd::ir::Opcode::WaitTime),
                        "Last Inst of %top.and should have Opcode WaitTime."
                    );
                } else if block_name == top_module_name_first_block {
                    assert_eq!(
                        11,
                        child_entity.len(),
                        "There should be 11 child nodes(11 Insts) in %top.and.bb0 block."
                    );
                    let const_int_inst =
                        llhd_world.world().get::<LLHDInstComponent>(child_entity[0]);
                    let const_int_inst_opcode = const_int_inst.unwrap().data.opcode();
                    assert!(
                        matches!(const_int_inst_opcode, llhd::ir::Opcode::ConstTime),
                        "First Inst of @top should have Opcode ConstTime."
                    );
                    let instantiation_inst =
                        llhd_world.world().get::<LLHDInstComponent>(child_entity[9]);
                    let instantiation_inst_opcode = instantiation_inst.unwrap().data.opcode();
                    assert!(
                        matches!(instantiation_inst_opcode, llhd::ir::Opcode::Inst),
                        "Last Inst(not include the very last, which is nullary) of @top should have Opcode Inst."
                    );
                } else {
                    panic!("Unknown module name: {}", block_name);
                }
            });
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
        assert!(
            llhd_world.slow_lookup(&magic_entity_name).is_some(),
            "@magic should be present name to lookup in ECS."
        );
        assert!(
            llhd_world.slow_lookup(&func_name).is_some(),
            "@foo should be present name to lookup in ECS."
        );

        let mut blocks: Vec<LLHDBlockComponent> = Default::default();
        let mut block_query = llhd_world.query::<(Entity, &Parent, &LLHDBlockComponent)>();
        block_query.iter(llhd_world.world()).for_each(
            |(_block_entity, parent_unit_entity, block_component)| {
                let parent_unit_component = llhd_world
                    .world()
                    .get::<LLHDUnitComponent>(**parent_unit_entity);
                let parent_unit_name = parent_unit_component.unwrap().name.to_string();
                if parent_unit_name == func_name {
                    blocks.push(block_component.to_owned());
                }
            },
        );
        assert_eq!(
            2,
            blocks.len(),
            "2 Blocks should be present in @foo function."
        );
    }
}
