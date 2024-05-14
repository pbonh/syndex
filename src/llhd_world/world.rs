use crate::{llhd::module::LLHDModule, world::LWorld};

#[derive(Debug, Default)]
pub struct LLHDWorld {
    pub(crate) module: LLHDModule,
    pub(crate) world: LWorld,
}

impl LLHDWorld {
    pub fn module(&self) -> &LLHDModule {
        &self.module
    }

    pub fn world(&self) -> &LWorld {
        &self.world
    }
}

impl From<(LLHDModule, LWorld)> for LLHDWorld {
    fn from(init: (LLHDModule, LWorld)) -> Self {
        Self {
            module: init.0,
            world: init.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        create_llhd_world,
        llhd_world::components::{inst::InstComponent, unit::UnitComponent, value::ValueComponent},
    };
    use std::collections::HashSet;

    use super::*;

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
    fn create_default_llhd_world() {
        let _ = LLHDWorld::default();
    }

    #[test]
    fn create_empty_llhd_world_via_macro() {
        let _llhd_world = create_llhd_world!();
    }

    #[test]
    fn create_llhd_world_via_macro() {
        // let mut _world = LWorld::default();
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
        let llhd_world = create_llhd_world!(module, TimingNode, TimingEdge);

        let sub_module_name = "%top.and";
        let top_module_name = "@top";
        assert!(
            llhd_world.world().lookup(&sub_module_name).is_some(),
            "%top.and should be present name to lookup in ECS."
        );
        assert!(
            llhd_world.world().lookup(&top_module_name).is_some(),
            "@top should be present name to lookup in ECS."
        );

        let sub_module_name_first_value = "v0";
        let sub_module_name_last_value = "v9";
        let top_module_name_first_value = "v0";
        let top_module_name_last_value = "v7";
        let sub_module_name_first_value_full_name =
            sub_module_name.to_owned() + "::" + sub_module_name_first_value;
        let sub_module_name_last_value_full_name =
            sub_module_name.to_owned() + "::" + sub_module_name_last_value;
        let top_module_name_first_value_full_name =
            top_module_name.to_owned() + "::" + top_module_name_first_value;
        let top_module_name_last_value_full_name =
            top_module_name.to_owned() + "::" + top_module_name_last_value;
        assert!(
            llhd_world
                .world()
                .lookup(&sub_module_name_first_value_full_name)
                .is_some(),
            "%top.and::v0 should be present name to lookup in ECS."
        );
        assert!(
            llhd_world
                .world()
                .lookup(&sub_module_name_last_value_full_name)
                .is_some(),
            "%top.and::v9 should be present name to lookup in ECS."
        );
        assert!(
            llhd_world
                .world()
                .lookup(&top_module_name_first_value_full_name)
                .is_some(),
            "@top::v0 should be present name to lookup in ECS."
        );
        assert!(
            llhd_world
                .world()
                .lookup(&top_module_name_last_value_full_name)
                .is_some(),
            "@top::v9 should be present name to lookup in ECS."
        );

        let sub_module_name_first_inst = "i0";
        let sub_module_name_last_inst = "i7";
        let top_module_name_first_inst = "i0";
        let top_module_name_last_inst = "i8";
        let sub_module_name_first_inst_full_name =
            sub_module_name.to_owned() + "::" + sub_module_name_first_inst;
        let sub_module_name_last_inst_full_name =
            sub_module_name.to_owned() + "::" + sub_module_name_last_inst;
        let top_module_name_first_inst_full_name =
            top_module_name.to_owned() + "::" + top_module_name_first_inst;
        let top_module_name_last_inst_full_name =
            top_module_name.to_owned() + "::" + top_module_name_last_inst;
        assert!(
            llhd_world
                .world()
                .lookup(&sub_module_name_first_inst_full_name)
                .is_some(),
            "%top.and::i0 should be present name to lookup in ECS."
        );
        assert!(
            llhd_world
                .world()
                .lookup(&sub_module_name_last_inst_full_name)
                .is_some(),
            "%top.and::i7 should be present name to lookup in ECS."
        );
        assert!(
            llhd_world
                .world()
                .lookup(&top_module_name_first_inst_full_name)
                .is_some(),
            "@top::i0 should be present name to lookup in ECS."
        );
        assert!(
            llhd_world
                .world()
                .lookup(&top_module_name_last_inst_full_name)
                .is_some(),
            "@top::i8 should be present name to lookup in ECS."
        );

        let mut units: HashSet<String> = Default::default();
        llhd_world
            .world()
            .each1(|_entity: flecs::Entity, unit_component: &UnitComponent| {
                units.insert(unit_component.name.to_string());
            });
        assert_eq!(2, units.len(), "There should be 2 Units present in ECS.");
        assert!(units.contains(sub_module_name));
        assert!(units.contains(top_module_name));

        llhd_world
            .world()
            .each1(|entity: flecs::Entity, _unit_component: &UnitComponent| {
                let mut value_str_list: HashSet<String> = Default::default();
                if entity.name() == sub_module_name {
                    entity.children(|child_value| {
                        let value_component = child_value.get::<ValueComponent>();
                        if let Some(value_id) = value_component.id {
                            let value_str = value_id.to_string();
                            value_str_list.insert(value_str);
                        } else {
                            panic!("Value should have a valid Id.");
                        }
                    });
                    assert_eq!(
                        10,
                        value_str_list.len(),
                        "10 Values should be present in sub entity."
                    );
                    assert!(value_str_list.contains(sub_module_name_first_value));
                    assert!(value_str_list.contains(sub_module_name_last_value));
                } else if entity.name() == top_module_name {
                    entity.children(|child_value| {
                        let value_component = child_value.get::<ValueComponent>();
                        if let Some(value_id) = value_component.id {
                            let value_str = value_id.to_string();
                            value_str_list.insert(value_str);
                        } else {
                            panic!("Value should have a valid Id.");
                        }
                    });
                    assert_eq!(
                        8,
                        value_str_list.len(),
                        "8 Values should be present in top entity."
                    );
                    assert!(value_str_list.contains(top_module_name_first_value));
                    assert!(value_str_list.contains(top_module_name_last_value));
                } else {
                    panic!("Unknown module name: {}", entity.name());
                }
            });

        let mut values: Vec<String> = Default::default();
        llhd_world
            .world()
            .each1(|_entity: flecs::Entity, value_component: &ValueComponent| {
                values.push(value_component.id.unwrap().to_string());
            });
        assert_eq!(
            18,
            values.len(),
            "There should be 18 Values present in ECS."
        );

        llhd_world
            .world()
            .each1(|entity: flecs::Entity, _unit_component: &UnitComponent| {
                let mut inst_str_list: HashSet<String> = Default::default();
                if entity.name() == sub_module_name {
                    entity.children(|child_inst| {
                        let inst_component = child_inst.get::<InstComponent>();
                        if let Some(inst_id) = inst_component.id {
                            let inst_str = inst_id.to_string();
                            inst_str_list.insert(inst_str);
                        } else {
                            panic!("inst should have a valid Id.");
                        }
                    });
                    assert_eq!(
                        10,
                        inst_str_list.len(),
                        "10 insts should be present in sub entity."
                    );
                    assert!(inst_str_list.contains(sub_module_name_first_inst));
                    assert!(inst_str_list.contains(sub_module_name_last_inst));
                } else if entity.name() == top_module_name {
                    entity.children(|child_inst| {
                        let inst_component = child_inst.get::<InstComponent>();
                        if let Some(inst_id) = inst_component.id {
                            let inst_str = inst_id.to_string();
                            inst_str_list.insert(inst_str);
                        } else {
                            panic!("inst should have a valid Id.");
                        }
                    });
                    assert_eq!(
                        8,
                        inst_str_list.len(),
                        "8 insts should be present in top entity."
                    );
                    assert!(inst_str_list.contains(top_module_name_first_inst));
                    assert!(inst_str_list.contains(top_module_name_last_inst));
                } else {
                    panic!("Unknown module name: {}", entity.name());
                }
            });

        let mut insts: Vec<String> = Default::default();
        llhd_world
            .world()
            .each1(|_entity: flecs::Entity, inst_component: &InstComponent| {
                insts.push(inst_component.id.unwrap().to_string());
            });
        assert_eq!(18, insts.len(), "There should be 18 insts present in ECS.");
    }
}
