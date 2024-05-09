#[macro_export]
macro_rules! create_llhd_world {
    // Pattern for module followed by component types
    ( $module:expr, $( $x:ty ),* ) => {
        {
            let _world = $crate::world::LWorld::default();
            let mut world = $crate::world::LWorld::default();
            world.component::<$crate::llhd_world::components::unit::UnitComponent>();
            world.component::<$crate::llhd_world::components::inst::InstComponent>();
            world.component::<$crate::llhd_world::components::value::ValueComponent>();
            $(
                world.component::<$x>();
            )*
            $crate::llhd_world::initializer::build_units(&$module).for_each(|unit_component| {
                let unit_name = unit_component
                    .name
                    .get_name()
                    .unwrap_or("");
                world.entity().named(unit_name)
                    .set::<$crate::llhd_world::components::unit::UnitComponent>(unit_component);
            });
            $crate::llhd_world::world::LLHDWorld::from(($crate::llhd::module::LLHDModule::from($module), world))
        }
    };

    // Pattern for just component types (fallback)
    ( $( $x:ty ),* ) => {
        {
            let mut world = $crate::world::LWorld::default();
            world.component::<$crate::llhd_world::components::unit::UnitComponent>();
            world.component::<$crate::llhd_world::components::inst::InstComponent>();
            world.component::<$crate::llhd_world::components::value::ValueComponent>();
            $(
                world.component::<$x>();
            )*
            $crate::llhd_world::world::LLHDWorld::from(($crate::llhd::module::LLHDModule::default(), world))
        }
    };
}
