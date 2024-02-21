#[macro_export]
macro_rules! create_index {
    // Pattern for string literal followed by component types
    ( $module:expr, $( $x:ty ),* ) => {
        {
            let mut index = Syndex::new();
            let component_entity = index.component::<UnitComponent>();
            index.component_types.push(component_entity);
            $(
                let component_entity = index.component::<$x>();
                index.component_types.push(component_entity);
            )*
            index.load($module);
            index
        }
    };

    // Pattern for just component types (fallback)
    ( $( $x:ty ),* ) => {
        {
            let mut index = Syndex::new();
            let component_entity = index.component::<UnitComponent>();
            index.component_types.push(component_entity);
            $(
                let component_entity = index.component::<$x>();
                index.component_types.push(component_entity);
            )*
            index
        }
    };
}
