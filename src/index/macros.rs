// Macro definition
macro_rules! create_map_struct {
    // Match a struct definition with a list of secondary map types
    ($struct_name:ident, $key_type:ty, $main_value_type:ty, $( $sec_map_name:ident : $sec_value_type:ty ),*) => {
        struct $struct_name {
            main_map: HashMap<$key_type, $main_value_type>,
            $(
                $sec_map_name: HashMap<$key_type, $sec_value_type>,
            )*
        }

        impl $struct_name {
            // Constructor to initialize the struct
            fn new() -> Self {
                Self {
                    main_map: HashMap::new(),
                    $(
                        $sec_map_name: HashMap::new(),
                    )*
                }
            }
        }
    };
}

macro_rules! define_syn_map {
    // Match a struct definition with a list of secondary map types
    ($struct_name:ident, $key_type:ident, $main_value_type:ty, $( $sec_map_name:ident : $sec_value_type:ty ),*) => {
        new_key_type! {
            struct $key_type;
        }

        #[derive(Debug, Clone, Default)]
        pub struct $struct_name {
            main_map: SlotMap<$key_type, $main_value_type>,
            $(
                $sec_map_name: SecondaryMap<$key_type, $sec_value_type>,
            )*
        }

        impl $struct_name {
            // Constructor to initialize the struct
            pub fn new() -> Self {
                Self {
                    main_map: SlotMap::<$key_type, $main_value_type>::default(),
                    $(
                        $sec_map_name: SecondaryMap::<$key_type, $sec_value_type>::default(),
                    )*
                }
            }
        }
    };
}
