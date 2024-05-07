#[macro_export]
macro_rules! create_llhd_world {
    // Pattern for string literal followed by component types
    ( $module:expr, $( $x:ty ),* ) => {
        {
            let mut index = $crate::llhd_world::world::LLHDWorld::new();
            index.world.component::<$crate::llhd_world::components::unit::UnitComponent>();
            index.world.component::<$crate::llhd_world::components::inst::InstComponent>();
            index.world.component::<$crate::llhd_world::components::value::ValueComponent>();
            $(
                index.world.component::<$x>();
            )*
            $crate::llhd_world::initializer::build_units($module).for_each(|unit_component| {
                index.world.entity().named(
                    unit_component
                    .name
                    .get_name()
                    .unwrap_or("")
                ).set::<$crate::llhd_world::components::unit::UnitComponent>(unit_component);
            });
            index
        }
    };

    // Pattern for just component types (fallback)
    ( $( $x:ty ),* ) => {
        {
            let mut index = $crate::llhd_world::world::LLHDWorld::new();
            index.world.component::<$crate::llhd_world::components::unit::UnitComponent>();
            index.world.component::<$crate::llhd_world::components::inst::InstComponent>();
            index.world.component::<$crate::llhd_world::components::value::ValueComponent>();
            $(
                index.world.component::<$x>();
            )*
            index
        }
    };
}
