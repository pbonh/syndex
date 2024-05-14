#[macro_export]
macro_rules! create_llhd_world {
    // Pattern for module followed by component types
    ( $module:expr, $( $x:ty ),* ) => {
        {
            use $crate::world::LWorld;
            use $crate::llhd_world::components::{unit::UnitComponent, value::ValueComponent, inst::InstComponent};
            use $crate::llhd_world::initializer::{build_units, build_insts, build_values};

            let mut world = LWorld::default();

            world.component::<UnitComponent>();
            world.component::<ValueComponent>();
            world.component::<InstComponent>();
            $(
                world.component::<$x>();
            )*
            build_units(&$module).for_each(|unit_component| {
                if let Some(unit_id) = unit_component.id {
                    let unit_name = unit_component
                        .name
                        .to_string();
                    let unit = world.entity()
                        .named(&unit_name)
                        .set::<UnitComponent>(unit_component);
                    build_values(&$module.unit(unit_id)).for_each(|value_component| {
                        if let Some(value_id) = value_component.id {
                            let value_name = value_id.to_string();
                            world.entity()
                                .named(&value_name)
                                .child_of(unit)
                                .set::<ValueComponent>(value_component);
                        }
                    });
                    build_insts(&$module.unit(unit_id)).for_each(|inst_component| {
                        if let Some(inst_id) = inst_component.id {
                            let inst_name = inst_id.to_string();
                            world.entity()
                                .named(&inst_name)
                                .child_of(unit)
                                .set::<InstComponent>(inst_component);
                        }
                    });
                }
            });
            LLHDWorld::from((LLHDModule::from($module), world))
        }
    };

    // Pattern for just component types (fallback)
    ( $( $x:ty ),* ) => {
        {
            use $crate::world::LWorld;
            let mut world = LWorld::default();
            world.component::<UnitComponent>();
            world.component::<InstComponent>();
            world.component::<ValueComponent>();
            $(
                world.component::<$x>();
            )*
            LLHDWorld::from((LLHDModule::default(), world))
        }
    };
}
