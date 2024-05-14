use bevy_ecs::prelude::{Component, Entity};
use bevy_hierarchy::BuildWorldChildren;
use std::ops::Add;

use crate::llhd_world::initializer::{build_insts, build_units, build_values};
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
                        build_values(&module.unit(unit_id)).for_each(|value_component| {
                            if let Some(value_id) = value_component.id {
                                let value_name = unit_name.to_owned()
                                    + ECSEntityName(".".to_string())
                                    + ECSEntityName(value_id.to_string());
                                parent_unit.spawn(value_component).insert(value_name);
                            }
                        });
                        build_insts(&module.unit(unit_id)).for_each(|inst_component| {
                            if let Some(inst_id) = inst_component.id {
                                let inst_name = unit_name.to_owned()
                                    + ECSEntityName(".".to_string())
                                    + ECSEntityName(inst_id.to_string());
                                parent_unit.spawn(inst_component).insert(inst_name);
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
}

impl From<LLHDModule> for LLHDWorld {
    fn from(module: LLHDModule) -> Self {
        Self::new(module)
    }
}

#[cfg(test)]
mod tests {
    // use crate::llhd_world::components::{
    //     inst::LLHDInstComponent, unit::LLHDUnitComponent, value::LLHDValueComponent,
    // };
    // use std::collections::HashSet;

    use crate::llhd_world::components::inst::LLHDInstComponent;

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
        let top_module_name_last_value = "v7";
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

        let sub_module_name_first_inst = "i0";
        let sub_module_name_last_inst = "i7";
        let top_module_name_first_inst = "i0";
        let top_module_name_last_inst = "i8";
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
            "@top.i0 should be present name to lookup in ECS."
        );
        assert!(
            top_module_name_last_inst_entity.is_some(),
            "@top.i8 should be present name to lookup in ECS."
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

        // let mut insts: Vec<String> = Default::default();
        // llhd_world
        //     .world()
        //     .each1(|_entity: flecs::Entity, inst_component: &LLHDInstComponent| {
        //         insts.push(inst_component.id.unwrap().to_string());
        //     });
        // assert_eq!(18, insts.len(), "There should be 18 insts present in ECS.");
    }
}
