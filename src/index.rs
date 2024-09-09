pub mod category;
#[macro_use]
pub mod macros;
pub mod unit;

use std::collections::HashMap;

// Usage of the macro
create_map_struct!(
    OneMapManySubmaps,
    String,           // Key type (K)
    i32,              // Main map value type (V)
    secondary_map1: f32,   // Secondary map 1 type (SecV1)
    secondary_map2: bool   // Secondary map 2 type (SecV2)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_llhd_slotmap() {
        // Create an instance of the generated struct
        let mut my_maps = OneMapManySubmaps::new();

        // Inserting values into the main and secondary maps
        my_maps.main_map.insert("key1".to_string(), 42);
        my_maps.secondary_map1.insert("key1".to_string(), 3.14);
        my_maps.secondary_map2.insert("key1".to_string(), true);

        println!("Main map: {:?}", my_maps.main_map);
        println!("Secondary map 1: {:?}", my_maps.secondary_map1);
        println!("Secondary map 2: {:?}", my_maps.secondary_map2);
    }
}
